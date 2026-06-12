use crate::core::modulecore::ModuleCore;
use crate::utilities::logger::Priority;
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};



#[derive(Debug, Clone ,Serialize, Deserialize)]
pub struct Ledmodule {
    core: ModuleCore,
    state: bool,
    pin: u8,
}

impl Ledmodule {
    pub fn new(pin: u8) -> Ledmodule {
       let ledmodule = Ledmodule {
            core: ModuleCore::new("LED"),
            state: false,
            pin,
        };
        ledmodule.send_serde_json(Priority::Medium);
        ledmodule
    }
    pub fn get_id(&self) -> &str {
        self.core.get_id()
    }


    fn self_to_json(&self , priority: Priority) -> Value {
        json!({
            "id": self.get_id(),
            "type": self.core.get_module_type(),
            "state": self.state,
            "pin": self.pin,
            "version": "1.0",
            "priority": format!("{:?}", priority)
        })
    }
    pub fn send_serde_json(&self, priority: Priority) {
        let json_data = self.self_to_json(priority);
        // Here you would send json_data to your desired destination
        serde_json::to_string(&json_data).map(|s| println!("Sending JSON: {}", s)).unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));
        println!("Sending JSON: {}", json_data);
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn set_state(&mut self, state: bool) {
        self.state = state;
    }
}