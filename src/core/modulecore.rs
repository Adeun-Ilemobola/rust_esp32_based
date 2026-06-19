use uuid::Uuid;
use crate::utilities::sharetype::{
    ModuleCommand
};

#[derive(Debug, Clone )]
pub struct ModuleCore {
    pub  id: String,
    pub module_type: String,
}

impl ModuleCore {
    pub fn new( module_type: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            module_type: module_type.to_string(),
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_module_type(&self) -> &str {
        &self.module_type
    }
}

pub trait Module {
    fn core(&self) -> &ModuleCore;
    fn id (&self)->&String;
    fn get_module_type(&self)->&String;
    fn handle_command(&mut self, command: &ModuleCommand) -> anyhow::Result<()>;
    
}