use crate::core::hardware::{InputPin, InputPinCore, Pull};
use crate::core::modulecore::ModuleCore;
use crate::utilities::logger::{EventModeType, Priority};
use serde_json::{json, Value};

static BUTTON_MODULE_MAX_TIME: u64 = 30; // Maximum time in milliseconds to consider a button press valid

pub struct Buttonmodule<'d> {
    core: ModuleCore,
    state: bool,
    pin: u8,
    pin_driver: InputPinCore<'d>,
    last_state: bool,
    last_change_time: std::time::Instant,
}

impl<'d> Buttonmodule<'d> {
    pub fn new<T>(pin_number: u8, pin: T) -> anyhow::Result<Buttonmodule<'d>>
    where
        T: InputPin + 'd,
    {
        let buttonmodule = Buttonmodule {
            core: ModuleCore::new("Button"),
            state: false,
            pin: pin_number,
            pin_driver: InputPinCore::new(pin_number, pin, Pull::Up)?,
            last_state: false,
            last_change_time: std::time::Instant::now(),
        };

        buttonmodule.send_serde_json(Priority::Medium, EventModeType::Register);

        Ok(buttonmodule)
    }

    pub fn update_state(&mut self) -> anyhow::Result<()> {
        let current_state = self.pin_driver.high()?;

        if current_state != self.last_state {
            let now = std::time::Instant::now();
            if now.duration_since(self.last_change_time).as_millis()
                < BUTTON_MODULE_MAX_TIME as u128
            {
                self.state = current_state;
                self.send_serde_json(Priority::Medium, EventModeType::State);
            }
            self.last_change_time = now;
            self.last_state = current_state;
        }

        Ok(())
    }
    pub fn is_pressed(&mut self) -> anyhow::Result<bool> {
        self.update_state()?;

        Ok(self.state && self.pin_driver.high()?)
    }

    pub fn get_id(&self) -> &str {
        self.core.get_id()
    }

    pub fn self_to_json(&self, priority: Priority, event_mode: EventModeType) -> Value {
        json!({
            "id": self.core.get_id(),
            "type": self.core.get_module_type(),
            "state": self.state,
            "pin": self.pin,
            "version": "1.0",
            "priority": format!("{:?}", priority),
            "event_mode": format!("{:?}", event_mode),
        })
    }

    pub fn send_serde_json(&self, priority: Priority, event_mode: EventModeType) {
        let json_data = self.self_to_json(priority, event_mode);

        serde_json::to_string(&json_data)
            .map(|s| println!("{}", s))
            .unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));
    }
}
