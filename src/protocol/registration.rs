use serde::{Deserialize, Serialize};

use crate::protocol::module_event::ModuleEvent;

#[derive(Debug, Serialize , Deserialize ,Clone )]
pub enum ModuleType {
    Servo,
    Led,
    Imu,
    LedCluster,
    Button,
    Lidar,
     SysLog
}
#[derive(Debug, Serialize ,Clone )]
pub struct Registration {
    pub id: String,
    pub module_type: ModuleType,
    pub lool_up_id:String,
    pub parent_id:String
}

#[derive(Debug, Serialize ,Clone )]
pub struct   ServoCapability{
    pub max_angle: i32,
    pub min_angle: i32,
    pub offset: i32,
    pub min_pivot: i32,
    pub max_pivot: i32,
    pub pulse_min:i32,
    pub pulse_max:i32,

}



#[derive(Debug, Serialize ,Clone )]
#[serde(tag = "type", content = "payload")]
pub enum ProtocolMessage {
    Registration(Registration),
    ModuleEvent(ModuleEvent),
    // Command(Command),
}