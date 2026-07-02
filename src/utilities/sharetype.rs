use serde::{Deserialize , Serialize};
use serde_json::Value;


#[derive(serde::Deserialize)]
pub enum EspCommand {
    SetLed { id: u8, value: u32 },
    Ping,
}

#[derive(Debug, Serialize , Deserialize)]
pub struct OutgoingEvent {
    pub id: String,
    pub version: String,
    pub kind: String,
    pub moduletype: String,
    pub payload: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_info: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct IncomingCommand {
    pub kind: String,
    pub id: String,

    #[serde(flatten)]
    pub command: ModuleCommand,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "moduletype", content = "payload", rename_all = "snake_case")]
pub enum ModuleCommand {
    Led(LedCommandPayload),
    ClusterLeds(ClusterCommandPayload)  
}

#[derive(Debug, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum LedCommandPayload {
    SetState { state: u32 },
    Toggle,
}

//cluster Leds
#[derive(Debug, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum ClusterCommandPayload {
    ToggleAll,
    SetAll {state: u32,},

    Toggle {id:String , state: u32},
    SetState { id:String , state: u32 },
}

