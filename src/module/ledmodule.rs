use crate::core::hardware::{
    ledc,
    OutputPin,
    // OutputPinCore
};
use crate::core::modulecore::{Module, ModuleCore};
use crate::utilities::logger::{EventModeType, Priority};
use crate::utilities::math::map_range;
use crate::utilities::sharetype::{
    LedCommandPayload, ModuleCommand
};
use anyhow::Ok;
use serde_json::{json, Value};

pub struct Ledmodule<'d> {
    core: ModuleCore,
     state: u32,
    pin: u8,
    // pin_driver: OutputPinCore<'d>,
    pwm: ledc::LedcDriver<'d>,
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
        };

        let _ = ledmodule.serialize(Priority::Medium, EventModeType::Register);

        Ok(ledmodule)
    }

    pub fn set_state(&mut self, state: u32) -> anyhow::Result<()> {
        let p = map_range(state, 0, 100, 0, self.pwm.get_max_duty());
        self.pwm.set_duty(p)?;
        self.state = state;
        let  _ = self.serialize(Priority::Medium, EventModeType::State);

        Ok(())
    }
    pub fn  get_state(&self)-> anyhow::Result<&u32>{
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
    fn handle_command(&mut self, command: &ModuleCommand) -> anyhow::Result<()>{
        match command {
            ModuleCommand::Led(led_command) => match led_command {
                LedCommandPayload::SetState { state } => self.set_state(state.clone())?,
                LedCommandPayload::Toggle => self.toggle()?,
            },
           
        }
        Ok(())
    }

    fn serialize(&self, _priority: Priority, event_mode: EventModeType) -> anyhow::Result<()> {

        let kind = match event_mode {
            EventModeType::Register => "registered",
            EventModeType::State => "event",
        };
       let data = json!({
            "id": self.id(),
            "version": "1.0",
            "kind": kind,
            "moduletype": "led",
            "payload": { "state": self.state },
        });

         serde_json::to_string(&data)
            .map(|s| println!("{}", s))
            .unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));

        Ok(())
        
    }
}
