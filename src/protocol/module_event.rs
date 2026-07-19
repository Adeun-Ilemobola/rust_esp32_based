use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "module_type", content = "event")]
pub enum ModuleEvent {
    Led(LedEvent),
    Servo(ServoEvent),
    Lidar(LidarEvent),
    Button(ButtonEvent),
    SysLog(SysLogEvent),
}

// ------ SysLogEvent -----

#[derive(Debug, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Clone)]
pub struct SysLogEvent {
    pub text: String,
    pub raw_err: Option<String>,
    pub priority: LogPriority,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "event_type")]
// ------ LedEvent-----
pub enum LedEvent {
    Brightness { id: String, level: u32 },
}

// ------ ServoEvent-----
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "event_type")]
pub enum ServoEvent {
    GetAngle { id: String, angle: i32 },
    GetMinPivot { id: String, min_pivot: i32 },
    GetMaxPivot { id: String, max_pivot: i32 },
    GetOffset { id: String, angle: i32 },
}

// ------ LidarEvent-----

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event_type")]
pub struct RangPoint {
    pub x: i32,
    pub y: i32,
    pub distant: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ScanState {
    Idol,
    Scanning,
    StopScan,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event_type")]
pub enum LidarEvent {
    Roi { id: String, min: Point, max: Point },
    PointMap { id: String, map: Vec<RangPoint> },
    Target { id: String, point: Point },
    ScanState { id: String, state: ScanState },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "event_type")]
pub enum ButtonEvent {
    Ckick { id: String },
}
