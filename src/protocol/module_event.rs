use serde::Serialize;

#[derive(Debug, Serialize ,Clone )]
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

#[derive(Debug, Serialize ,Clone )]
#[serde(tag = "event_type")]
// ------ LedEvent-----
pub enum LedEvent {
   Brightness{level:u32 }
}


// ------ ServoEvent-----
#[derive(Debug, Serialize ,Clone )]
#[serde(tag = "event_type")]
pub enum  ServoEvent {
    GetAngle{angle:i32},
     GetMinPivot { min_pivot: i32 },
    GetMaxPivot { max_pivot: i32 },
    GetOffset {angle :i32}
}

// ------ LidarEvent-----

#[derive(Debug, Serialize ,Clone )]
#[serde(tag = "event_type")]
pub struct  RangPoint{
    x:i32,
    y:i32,
    distant:u32
}

#[derive(Debug, Serialize ,Clone )]
#[serde(tag = "event_type")]
pub enum LidarEvent{
   Roi{
    id:String,
    x_min:i32 ,
    y_min:i32,
    x_max:i32 ,
    y_max:i32,
   },
   PointMap{id :String , map:Vec<RangPoint>},
   Target{  x:i32 , y:i32}
}


#[derive(Debug, Serialize ,Clone )]
#[serde(tag = "event_type")]
pub  enum ButtonEvent {
    Ckick    
}


