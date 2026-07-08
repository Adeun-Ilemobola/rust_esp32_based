use crate::core::hardware::{InputPin, InputPinCore, Pull};
use crate::core::modulecore::{Module, ModuleCore};
use crate::utilities::logger::{EventModeType, Priority};
use crate::utilities::sharetype::OutgoingEvent;
use esp_idf_svc::hal::gpio::Level;
use serde_json::json;

static BUTTON_MODULE_MAX_TIME: u64 = 150; // Maximum time in milliseconds to consider a button press valid

pub struct Buttonmodule<'d> {
    core: ModuleCore,
    state: Level,      // debounced/committed level
    prev_state: Level, // previous committed level, for edge detection
    pin: u8,
    pin_driver: InputPinCore<'d>,
    last_state: Level,
    last_change_time: std::time::Instant,
}

impl<'d> Buttonmodule<'d> {
    pub fn new<T>(pin: T ) -> anyhow::Result<Buttonmodule<'d>>
    where
        T: InputPin + 'd,
    {
        let buttonmodule = Buttonmodule {
            core: ModuleCore::new("Button" , "btu-3434"),
            state: Level::High,
            pin: pin.pin() as u8,
            pin_driver: InputPinCore::new(pin, Pull::Up)?,
            last_state: Level::High,
            last_change_time: std::time::Instant::now(),
            prev_state: Level::High,
        };

        let _ = buttonmodule.serialize( EventModeType::Register , None);

        Ok(buttonmodule)
    }
    pub fn update_state(&mut self) -> anyhow::Result<()> {
        let current_state = self.pin_driver.now()?;
        let now = std::time::Instant::now();

        if current_state != self.last_state {
            self.last_state = current_state;
            self.last_change_time = now;
        }
        if now.duration_since(self.last_change_time).as_millis() >= BUTTON_MODULE_MAX_TIME as u128 {
            self.state = self.last_state;
        }
        Ok(())
    }
    pub fn poll(&mut self) -> anyhow::Result<bool> {
        self.update_state()?;

        if self.state != self.prev_state {
            let pressed = self.state == Level::Low;
            self.prev_state = self.state;
            self.serialize(EventModeType::State ,None)?; 
            return Ok(pressed); 
        }
        Ok(false)
    }
    pub fn get_event(&self, event_mode: EventModeType ,  cluster_id:Option<String>) -> anyhow::Result<OutgoingEvent> {
        let kind = match event_mode {
            EventModeType::Register => "registered",
            EventModeType::State => "event",
        };
        Ok(OutgoingEvent {
            id: self.id().to_string(),
            manuel_id: self.core.manuel_id.to_string(),
            version: "1.0".to_string(),
            kind: kind.to_string(),
            moduletype: "button".to_string(),
            payload: json!({
               "pressed": self.state == Level::Low
            }),
            generated_info: None,
            master_id:cluster_id
        })
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

    fn serialize(&self ,event_mode: EventModeType ,  cluster_id:Option<String>) -> anyhow::Result<()> {
        serde_json::to_string(&self.get_event(event_mode , cluster_id)?)
            .map(|s| println!("{}", s))
            .unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));

        Ok(())
    }
}
