use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SorvoConfig {
    pub max_angle: u32,
    pub min_angle: u32,
    pub offset: u32,
    pub min_pivot: u32,
    pub max_pivot: u32,
    pub pulse_min:u32,
    pub pulse_max:u32,
}
