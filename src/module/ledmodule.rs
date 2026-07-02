use crate::core::hardware::{ledc, OutputPin};
use crate::core::modulecore::{Module, ModuleCore};
use crate::utilities::logger::{EventModeType, Priority};
use crate::utilities::math::map_range;
use crate::utilities::sharetype::{LedCommandPayload, ModuleCommand, OutgoingEvent};
use serde_json::json;

pub struct Ledmodule<'d> {
    core: ModuleCore,
    state: u32,
    pin: u8,
    pwm: ledc::LedcDriver<'d>,
    can_serialize: bool,
}
impl<'d> Ledmodule<'d> {
    pub fn new<T, C>(
        pin: T,
        channel: C,
        timer: &ledc::LedcTimerDriver<'d, ledc::LowSpeed>,
    ) -> anyhow::Result<Ledmodule<'d>>
    where
        T: OutputPin + 'd,
        C: ledc::LedcChannel<SpeedMode = ledc::LowSpeed> + 'd,
    {
        let pin_number = pin.pin() as u8;

        let pwm = ledc::LedcDriver::new(channel, timer, pin)?;

        let ledmodule = Ledmodule {
            core: ModuleCore::new("LED"),
            state: 0,
            pin: pin_number,
            pwm,
            can_serialize: true,
        };

        let _ = ledmodule.serialize(Priority::Medium, EventModeType::Register);

        Ok(ledmodule)
    }

    pub fn set_state(&mut self, state: u32) -> anyhow::Result<()> {
        let p = map_range(state, 0, 100, 0, self.pwm.get_max_duty());
        self.pwm.set_duty(p)?;
        self.state = state;
        if self.can_serialize {
            let _ = self.serialize(Priority::Medium, EventModeType::State);
        }

        Ok(())
    }
    pub fn get_state(&self) -> anyhow::Result<&u32> {
        Ok(&self.state)
    }

    pub fn toggle(&mut self) -> anyhow::Result<()> {
        if self.state == 0 {
            self.set_state(100)?;
        } else {
            self.set_state(0)?;
        }

        Ok(())
    }
    pub fn event_mode(&mut self, mode: bool) {
        self.can_serialize = mode;
    }
    pub fn get_event(&self, event_mode: EventModeType) -> anyhow::Result<OutgoingEvent> {
        let kind = match event_mode {
            EventModeType::Register => "registered",
            EventModeType::State => "event",
        };

        Ok(OutgoingEvent {
            id: self.id().to_string(),
            version: "1.0".to_string(),
            kind: kind.to_string(),
            moduletype: "led".to_string(),
            payload: json!({
                "state": self.state
            }),
            generated_info: None,
        })
    }
}

impl<'d> Module for Ledmodule<'d> {
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
            ModuleCommand::Led(led_command) => match led_command {
                LedCommandPayload::SetState { state } => self.set_state(state.clone())?,
                LedCommandPayload::Toggle => self.toggle()?,
            },
            _ => {
                // handle anything else
            }
        }
        Ok(())
    }

    fn serialize(&self, _priority: Priority, event_mode: EventModeType) -> anyhow::Result<()> {
        serde_json::to_string(&self.get_event(event_mode)?)
            .map(|s| println!("{}", s))
            .unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));

        Ok(())
    }
}
