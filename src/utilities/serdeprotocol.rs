use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize , Clone)]
pub enum ModuleType {
    Servo,
    Led,
    Imu,
    LedCluster,
    Button,
    Lidar,
    SysLog
}
#[derive(Debug, Serialize, Deserialize ,Clone )]
pub enum EventModeType {
    State,
    Register,
    SysLog
}

#[derive(serde::Deserialize)]
pub enum EspCommand {
    SetLed { id: u8, value: u32 },
    Ping,
}

#[derive(Debug, Serialize, Deserialize , Clone )]
pub struct OutgoingEvent {
    pub id: String,
    pub manuel_id: String,
    pub version: String,
    pub kind: EventModeType,
    pub moduletype: ModuleType,
    pub payload: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_info: Option<Value>,
    pub master_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IncomingCommand {
    pub kind: String,
    pub id: String,

    #[serde(flatten)]
    pub command: ModuleCommand,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "moduletype", content = "payload", )]
pub enum ModuleCommand {
    Led(LedCommandPayload),
    ClusterLeds(ClusterCommandPayload),
    Servo(ServoCommandPayload),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "command", )]
pub enum LedCommandPayload {
    SetState { state: u32 },
    Toggle,
}

//Servo
#[derive(Debug, Deserialize)]
#[serde(tag = "command", )]
pub enum ServoCommandPayload {
    SetAngle { angle: i32 },
    SetMinPivot { min_pivot: i32 },
    SetMaxPivot { max_pivot: i32 },
}

//cluster Leds
#[derive(Debug, Deserialize)]
#[serde(tag = "command", )]
pub enum ClusterCommandPayload {
    ToggleAll,
    SetAll { state: u32 },

    Toggle { id: String, state: u32 },
    SetState { id: String, state: u32 },
}


// ---- Lidar




