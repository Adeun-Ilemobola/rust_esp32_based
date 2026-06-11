use crate::core::modulecore::ModuleCore;
use crate::utilities::logger::Level;
use serde_json::{json, Value};


pub struct Ledmodule {
    core: ModuleCore,
    state: bool,
    pin: u8,
}

impl Ledmodule {
    pub fn new(pin: u8) -> Ledmodule {
        Ledmodule {
            core: ModuleCore::new("LED"),
            state: false,
            pin,
        }
    }
    pub fn get_id(&self) -> &str {
        self.core.get_id()
    }


    pub fn state_to_json(&self , priority: Level) -> Value {
        json!({
            "id": self.get_id(),
            "type": self.core.get_module_type(),
            "state": self.state,
            "pin": self.pin,
            "protocol_version": "1.0",
            "priority": format!("{:?}", priority)
        })
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn set_state(&mut self, state: bool) {
        self.state = state;
    }
}