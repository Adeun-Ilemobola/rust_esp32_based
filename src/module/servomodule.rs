use crate::core::hardware::{SharedPwm, sleep_ms};
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
            core: ModuleCore::new("servo", &manuel_id),
            pwm,
            config: config.clone(),
            offset: config.offset,
            angle: config.min_pivot,
            max_pivot: config.max_pivot,
            min_pivot: config.min_pivot,
            channel: channel.clone(),
            can_serialize: true,
        };

        if cluster_id.is_some() {
            s.can_serialize = false
        }

        let _ = s.serialize(EventModeType::Register, cluster_id.clone());
        s.set_offset(s.offset)?;
        s.set_angle(0)?;
        sleep_ms(5000);

        let testrang: [i32; 4] = [35, 10, -10, -35];
        for f in testrang {
            s.set_angle(f)?;
            sleep_ms(600);
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
            let _ = self.serialize(EventModeType::State, None);
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
            let _ = self.serialize(EventModeType::State, None);
        }

        Ok(())
    }

    pub fn set_min_pivot(&mut self, min_pivot: i32) {
        self.min_pivot = min_pivot.min(self.max_pivot);
        if self.can_serialize {
            let _ = self.serialize(EventModeType::State, None);
        }
    }

    pub fn set_max_pivot(&mut self, max_pivot: i32) {
        self.max_pivot = max_pivot.max(self.min_pivot);
        if self.can_serialize {
            let _ = self.serialize(EventModeType::State, None);
        }
    }

    pub fn get_event(
        &self,
        event_mode: EventModeType,
        cluster_id: Option<String>,
    ) -> anyhow::Result<OutgoingEvent> {
        Ok(OutgoingEvent {
            id: self.id().to_string(),
            version: "1.0".to_string(),
            manuel_id: self.core.manuel_id.to_string(),

            kind: event_mode,
            moduletype: ModuleType::Servo,
            payload: json!({
            "config":self.config.clone(),
                        "offset":self.offset,
                        "angle": self.angle,
            "min_pivot":self.min_pivot,
            "max_pivot":self.max_pivot

                    }),
            generated_info: None,
            master_id: cluster_id,
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
    fn get_module_type(&self) -> &String {
        &self.core.module_type
    }
    fn handle_command(&mut self, command: &ModuleCommand) -> anyhow::Result<()> {
        match command {
            ModuleCommand::Servo(servo_command) => match servo_command {
                ServoCommandPayload::SetAngle { angle } => self.set_angle(angle.clone())?,
                ServoCommandPayload::SetMinPivot { min_pivot } => {
                    self.set_min_pivot(min_pivot.clone())
                }
                ServoCommandPayload::SetMaxPivot { max_pivot } => {
                    self.set_max_pivot(max_pivot.clone())
                }
            },
            _ => {
                // handle anything else
            }
        }
        Ok(())
    }

    fn serialize(
        &self,
        event_mode: EventModeType,
        cluster_id: Option<String>,
    ) -> anyhow::Result<()> {
        serde_json::to_string(&self.get_event(event_mode, cluster_id)?)
            .map(|s| println!("{}", s))
            .unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));

        Ok(())
    }
}
