use crate::core::hardware::{InputPin, InputPinCore, Pull};
use crate::core::modulecore::{Module, ModuleCore};
use crate::utilities::logger::{EventModeType, Priority};
use anyhow::Ok;
use esp_idf_svc::hal::gpio::Level;
use serde_json::{json, Value};

static BUTTON_MODULE_MAX_TIME: u64 = 200; // Maximum time in milliseconds to consider a button press valid

pub struct Buttonmodule<'d> {
    core: ModuleCore,
    state: Level,
    pin: u8,
    pin_driver: InputPinCore<'d>,
    last_state: Level,
    last_change_time: std::time::Instant,
}

impl<'d> Buttonmodule<'d> {
    pub fn new<T>(pin: T) -> anyhow::Result<Buttonmodule<'d>>
    where
        T: InputPin + 'd,
    {
        let buttonmodule = Buttonmodule {
            core: ModuleCore::new("Button"),
            state: Level::High,
            pin: pin.pin() as u8,
            pin_driver: InputPinCore::new(pin, Pull::Up)?,
            last_state: Level::High,
            last_change_time: std::time::Instant::now(),
        };

        let _ = buttonmodule.serialize(Priority::Medium, EventModeType::Register);

        Ok(buttonmodule)
    }
    pub fn update_state(&mut self) -> anyhow::Result<()> {
        let current_state = self.pin_driver.now()?;
        let now = std::time::Instant::now();

        if current_state != self.last_state {
           self.last_state =current_state;
           self.last_change_time = now;
        }
        if now.duration_since(self.last_change_time).as_millis() >= BUTTON_MODULE_MAX_TIME as u128 {
            self.state = self.last_state;
        }
        Ok(())
    }
    pub fn is_pressed(&mut self) -> anyhow::Result<bool> {
        self.update_state()?;
        if self.state == Level::Low {
             let _ = self.serialize(Priority::Medium, EventModeType::State);
        }

        Ok(self.state == Level::Low)
    }
}
impl<'d> Module for Buttonmodule<'d> {
    fn id(&self) -> &String {
        &self.core.id
    }

    fn core(&self) -> &ModuleCore {
        &self.core
    }
    fn get_module_type(&self) -> &String {
        &self.core.module_type
    }
    fn handle_command(
        &mut self,
        _command: &crate::utilities::sharetype::ModuleCommand,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn serialize(&self, _priority: Priority, event_mode: EventModeType) -> anyhow::Result<()> {
        let kind = match event_mode {
            EventModeType::Register => "registered",
            EventModeType::State => "event",
        };
        let data = json!({
            "id": self.core.get_id(),
            "version": "1.0",
            "kind": kind,
            "moduletype": "button",
            "payload": { "pressed": self.state == Level::Low},
        });

        serde_json::to_string(&data)
            .map(|s| println!("{}", s))
            .unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));

        Ok(())
    }
}
