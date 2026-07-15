use crate::utilities::serdeprotocol::{EventModeType, ModuleCommand, ModuleType};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ModuleCore {
    pub id: String,
    pub module_type: ModuleType,
    pub manuel_id: String,
}

impl ModuleCore {
    pub fn new(module_type: ModuleType, manuel_id: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            module_type: module_type,
            manuel_id: manuel_id.to_string(),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_module_type(&self) -> &ModuleType {
        &self.module_type
    }
}

pub trait Module {
    fn core(&self) -> &ModuleCore;
    fn id(&self) -> &String;
    fn get_module_type(&self) -> &ModuleType;
    fn handle_command(&mut self, command: &ModuleCommand) -> anyhow::Result<()>;
    fn serialize(&self) -> anyhow::Result<()>;
}
