use crate::core::hardware::{OutputPin, OutputPinCore};
use crate::core::modulecore::ModuleCore;
use crate::utilities::logger::Priority;
use serde_json::{json, Value};

pub struct Ledmodule<'d> {
    core: ModuleCore,
    state: bool,
    pin: u8,
    pin_driver: OutputPinCore<'d>,
}
impl<'d> Ledmodule<'d> {
    pub fn new<T>(pin_number: u8, pin: T) -> anyhow::Result<Ledmodule<'d>>
    where
        T: OutputPin + 'd,
    {
        let ledmodule = Ledmodule {
            core: ModuleCore::new("LED"),
            state: false,
            pin: pin_number,
            pin_driver: OutputPinCore::new(pin_number, pin)?,
        };

        ledmodule.send_serde_json(Priority::Medium);

        Ok(ledmodule)
    }

    pub fn get_id(&self) -> &str {
        self.core.get_id()
    }

    pub fn set_state(&mut self, state: bool) -> anyhow::Result<()> {
        self.pin_driver.set_state(state)?;
        self.state = state;

        Ok(())
    }

    pub fn self_to_json(&self, priority: Priority) -> Value {
        json!({
            "id": self.get_id(),
            "type": self.core.get_module_type(),
            "state": self.state,
            "pin": self.pin,
            "version": "1.0",
            "priority": format!("{:?}", priority),
        })
    }

    pub fn send_serde_json(&self, priority: Priority) {
        let json_data = self.self_to_json(priority);

        serde_json::to_string(&json_data)
            .map(|s| println!("Sending JSON: {}", s))
            .unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));
    }
}