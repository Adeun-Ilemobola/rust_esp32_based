use crate::core::hardware::i2c::I2cDriver;
use crate::core::hardware::{ledc, OutputPin};
use crate::core::modulecore::{Module, ModuleCore};
use crate::utilities::logger::EventModeType;
use crate::utilities::math::range_u32;
use crate::utilities::moduleconflg::SorvoConfig;
use crate::utilities::serdeprotocol::{ModuleCommand, OutgoingEvent, SorvoCommandPayload};
use esp_idf_svc::hal::rmt::PulseTicks;
use pwm_pca9685::{Address, Channel, Pca9685};
use serde_json::json;

pub struct SorvoModule<'d> {
    core: ModuleCore,
    pwm: Pca9685<I2cDriver<'d>>,
    config: SorvoConfig,
    channel: Channel,
    can_serialize: bool,

    offset: u32,
    angle: u32,
    min_pivot: u32,
    max_pivot: u32,
}

impl<'d> SorvoModule<'d> {
    pub fn new(
        i2c: I2cDriver<'d>,
        channel: Channel,
        config: SorvoConfig,
        cluster_id: Option<String>,
    ) -> anyhow::Result<SorvoModule<'d>>
where {
        let mut s = SorvoModule {
            core: ModuleCore::new("Sorvo", "Sorvo-3423"),
            pwm: Pca9685::new(i2c, Address::default()).unwrap(),
            config: config.clone(),
            offset: config.offset,
            angle: config.min_pivot,
            max_pivot: config.max_pivot,
            min_pivot: config.min_pivot,
            channel: channel.clone(),
            can_serialize: true,
        };

        s.pwm.set_prescale(100).unwrap();
        s.pwm.enable().unwrap();

        if cluster_id.is_some() {
            s.can_serialize = false
        }

        let _ = s.serialize(EventModeType::Register, cluster_id.clone());

        Ok(s)
    }
    pub fn set_angle(&mut self, a: u32) -> anyhow::Result<()> {
        //  -22  ,  25
        let pivotrang = a.clamp(self.min_pivot, self.max_pivot);

        let raw_rang =
            (self.offset + pivotrang).clamp(self.config.min_angle, self.config.max_angle);

        self.angle = raw_rang.clone();

        let pulse = range_u32(
            raw_rang,
            self.config.min_angle,
            self.config.max_angle,
            self.config.pulse_min,
            self.config.pulse_max,
        );
        self.pwm
            .set_channel_on_off(self.channel, 0, pulse.try_into().unwrap())
            .unwrap();

        if self.can_serialize {
            let _ = self.serialize(EventModeType::State, None);
        }

        Ok(())
    }

    pub fn set_min_pivot(&mut self, min_pivot: u32) {
        self.min_pivot = min_pivot.min(self.max_pivot);
        if self.can_serialize {
            let _ = self.serialize(EventModeType::State, None);
        }
    }

    pub fn set_max_pivot(&mut self, max_pivot: u32) {
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
        let kind = match event_mode {
            EventModeType::Register => "registered",
            EventModeType::State => "event",
        };

        Ok(OutgoingEvent {
            id: self.id().to_string(),
            version: "1.0".to_string(),
            manuel_id: self.core.manuel_id.to_string(),

            kind: kind.to_string(),
            moduletype: "sorvo".to_string(),
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

impl<'d> Module for SorvoModule<'d> {
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
            ModuleCommand::Sorvo(sorvo_command) => match sorvo_command {
                SorvoCommandPayload::SetAngle { angle } => self.set_angle(angle.clone())?,
                SorvoCommandPayload::SetMinPivot { min_pivot } => self.set_min_pivot(min_pivot.clone()),
                SorvoCommandPayload::SetMaxPivot { max_pivot } => self.set_max_pivot(max_pivot.clone()),
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
