use crate::core::hardware::{sleep_ms, SharedPwm};
use crate::core::modulecore::{Module, ModuleCore};
use crate::utilities::math::{pulse_us_to_tick, range_i32};
use crate::utilities::moduleconflg::ServoConfig;
use crate::utilities::serdeprotocol::{
    EventModeType, ModuleCommand, ModuleType, OutgoingEvent, ServoCommandPayload,
};
use anyhow::Ok;
use pwm_pca9685::Channel;
use serde_json::json;

pub struct ServoModule<'d> {
    core: ModuleCore,
    pwm: SharedPwm<'d>,
    config: ServoConfig,
    channel: Channel,
    can_serialize: bool,

    offset: i32,
    angle: i32,
    min_pivot: i32,
    max_pivot: i32,

    event_mode: EventModeType,
    cluster_id: Option<String>,
}

impl<'d> ServoModule<'d> {
    pub fn new(
        pwm: SharedPwm<'d>,
        manuel_id: String,
        channel: Channel,
        config: ServoConfig,
        cluster_id: Option<String>,
    ) -> anyhow::Result<ServoModule<'d>> {
        let mut s = ServoModule {
            core: ModuleCore::new(ModuleType::Servo, &manuel_id),
            pwm,
            config: config.clone(),
            offset: config.offset,
            angle: config.min_pivot,
            max_pivot: config.max_pivot,
            min_pivot: config.min_pivot,
            channel: channel.clone(),
            can_serialize: true,
            event_mode: EventModeType::Register,
            cluster_id: cluster_id.clone(),
        };

        if cluster_id.is_some() {
            s.can_serialize = false
        }

        let _ = s.serialize();
        s.event_mode = EventModeType::State;
        s.set_offset(s.offset)?;
        s.set_angle(0)?;
       

        let testrang: [i32; 4] = [35, 10, -10, -35];
        for f in testrang {
            s.set_angle(f)?;
          
        }

        Ok(s)
    }
    pub fn set_offset(&mut self, a: i32) -> anyhow::Result<()> {
        self.offset = a.clamp(self.config.min_angle, self.config.max_angle);
        let pulse = range_i32(
            self.offset,
            self.config.min_angle,
            self.config.max_angle,
            self.config.pulse_min,
            self.config.pulse_max,
        );
        self.pwm
            .borrow_mut()
            .set_channel_on_off(self.channel, 0, pulse_us_to_tick(pulse))
            .unwrap();

        if self.can_serialize {
            let _ = self.serialize();
        }

        Ok(())
    }
    pub fn set_angle(&mut self, a: i32) -> anyhow::Result<()> {
        //  -22  ,  25
        let pivotrang = a.clamp(self.min_pivot, self.max_pivot);

        let raw_rang =
            (self.offset + pivotrang).clamp(self.config.min_angle, self.config.max_angle);

        self.angle = raw_rang.clone();

        let pulse = range_i32(
            raw_rang,
            self.config.min_angle,
            self.config.max_angle,
            self.config.pulse_min,
            self.config.pulse_max,
        );
        self.pwm
            .borrow_mut()
            .set_channel_on_off(self.channel, 0, pulse_us_to_tick(pulse))
            .unwrap();

        if self.can_serialize {
            let _ = self.serialize();
        }

        Ok(())
    }

    pub fn pivot_angle(&self) -> i32 {
        self.angle - self.offset
    }
    pub fn angle (&self) -> i32{
        self.angle
    }

    pub fn set_min_pivot(&mut self, min_pivot: i32) {
        self.min_pivot = min_pivot.min(self.max_pivot);
        if self.can_serialize {
            let _ = self.serialize();
        }
    }

    pub fn set_max_pivot(&mut self, max_pivot: i32) {
        self.max_pivot = max_pivot.max(self.min_pivot);
        if self.can_serialize {
            let _ = self.serialize();
        }
    }

    pub fn get_event(&self) -> anyhow::Result<OutgoingEvent> {
        Ok(OutgoingEvent {
            id: self.id().to_string(),
            version: "1.0".to_string(),
            manuel_id: self.core.manuel_id.to_string(),

            kind: self.event_mode.clone(),
            moduletype: ModuleType::Servo,
            payload: json!({
            "config":self.config.clone(),
                        "offset":self.offset,
                        "angle": self.angle,
            "min_pivot":self.min_pivot,
            "max_pivot":self.max_pivot

                    }),
            generated_info: None,
            master_id: self.cluster_id.clone(),
        })
    }
}

impl<'d> Module for ServoModule<'d> {
    fn id(&self) -> &String {
        &self.core.id
    }

    fn core(&self) -> &ModuleCore {
        &self.core
    }
    fn get_module_type(&self) -> &ModuleType {
        &self.core.module_type
    }
    fn handle_command(&mut self, command: &ModuleCommand) -> anyhow::Result<()> {
        match command {
            ModuleCommand::Servo(servo_command) => match servo_command {
                ServoCommandPayload::SetAngle { angle } => self.set_angle(*angle)?,
                ServoCommandPayload::SetMinPivot { min_pivot } => {
                    self.set_min_pivot(*min_pivot)
                }
                ServoCommandPayload::SetMaxPivot { max_pivot } => {
                    self.set_max_pivot(*max_pivot)
                }
            },
            _ => {
                // handle anything else
            }
        }
        Ok(())
    }

    fn serialize(&self) -> anyhow::Result<()> {
        serde_json::to_string(&self.get_event()?)
            .map(|s| println!("{}", s))
            .unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));

        Ok(())
    }
}
