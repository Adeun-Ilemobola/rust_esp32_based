use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::json;

use crate::utilities::serdeprotocol::{EventModeType, ModuleType, OutgoingEvent};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize, Ord, Serialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Deserialize, Ord, Serialize)]
pub struct SysLog {}
impl SysLog {
    pub fn emit(text: String, raw_err: Option<String>, priority: Priority) {
        let data = OutgoingEvent {
            id: "000000000".to_string(),
            manuel_id: "00000000".to_string(),
            moduletype: ModuleType::SysLog,
            generated_info: None,
            kind: EventModeType::SysLog,
            version: "1".to_string(),
            payload: json!({
                "text": text,
                "raw_err":raw_err,
                "priority":priority
            }),
            master_id: None,
        };

        serde_json::to_string(&data)
            .map(|s| println!("{}", s))
            .unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));
    }

    pub fn info(text: String, raw_err: Option<String>) {
        Self::emit(text, raw_err, Priority::Low);
    }

    pub fn error(text: String, raw_err: Option<String>) {
        Self::emit(text, raw_err, Priority::Critical);
    }
}
