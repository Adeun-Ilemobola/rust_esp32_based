use uuid::Uuid;
 
#[derive(Debug, Clone)]
pub struct modulecore {
    id: String,
    module_type: String,
}

impl modulecore {
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