use serde::Deserialize;




#[derive(Debug, Deserialize)]
pub struct IncomingCommand {
    pub id: String,
    #[serde(flatten)]
    pub command: ModuleCommand,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "module_type", content = "payload", )]
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




