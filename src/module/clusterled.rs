use anyhow::Ok;
use serde_json::{Value, json};

use crate::{core::modulecore::{Module, ModuleCore}, utilities::{logger::EventModeType, serdeprotocol::{ClusterCommandPayload, ModuleCommand, OutgoingEvent}}};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type ModuleHandle<'a> = Rc<RefCell<dyn Module + 'a>>;

pub struct ClusterLed <'d>{
      core: ModuleCore,
      modules: HashMap<String, ModuleHandle<'d>>
}

impl <'d>ClusterLed<'d> {
    pub fn new()-> anyhow::Result<ClusterLed<'d>>{
        let cluster = ClusterLed{
            core: ModuleCore::new("Custer" , "cust-3434"),
            modules:HashMap::new()
        };

        Ok(cluster)
    }
     pub fn get_event(&self, event_mode: EventModeType  , _cluster_id:Option<String>) -> anyhow::Result<OutgoingEvent> {
        let kind = match event_mode {
            EventModeType::Register => "registered",
            EventModeType::State => "event",
        };
        let mut  temp_child_payload:Vec<Value> = vec![];

        Ok(OutgoingEvent {
            id: self.id().to_string(),
            version: "1.0".to_string(),
            manuel_id: self.core.manuel_id.to_string(),

            kind: kind.to_string(),
            moduletype: "led".to_string(),
            payload: json!([temp_child_payload]),
            generated_info: None,
            master_id:None
        })
    }

}





impl<'d> Module for ClusterLed<'d> {
    fn id(&self) -> &String {
        &self.core.id
    }

    fn core(&self) -> &ModuleCore {
        &self.core
    }
    fn get_module_type(&self) -> &String {
        &self.core.module_type
    }
    fn handle_command(&mut self, command: &ModuleCommand) -> anyhow::Result<()> {
        match command {
            ModuleCommand::ClusterLeds(c)
            => match c {
                ClusterCommandPayload::SetAll { state }=>{

                    self.serialize(EventModeType::State, Some(self.id().clone()))?

                },
                ClusterCommandPayload::SetState { id, state }=>{

                    self.serialize(EventModeType::State, Some(self.id().clone()))?

                },
                ClusterCommandPayload::Toggle { id, state }=>{

                    self.serialize(EventModeType::State, Some(self.id().clone()))?

                },
                ClusterCommandPayload::ToggleAll =>{
                    self.serialize(EventModeType::State, Some(self.id().clone()))?
                }
                
            }
            
            _ => {
                // handle anything else
            }
        }
        Ok(())
    }

    fn serialize(&self ,event_mode: EventModeType ,  cluster_id:Option<String>) -> anyhow::Result<()> {
        serde_json::to_string(&self.get_event(event_mode , cluster_id)?)
            .map(|s| println!("{}", s))
            .unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));

        Ok(())
    }
}
