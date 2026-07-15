use anyhow::Ok;
use serde_json::{Value, json};

use crate::{
    core::modulecore::{Module, ModuleCore},
    utilities::serdeprotocol::{
        ClusterCommandPayload, EventModeType, ModuleCommand, ModuleType, OutgoingEvent,
    },
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type ModuleHandle<'a> = Rc<RefCell<dyn Module + 'a>>;

pub struct ClusterLed<'d> {
    core: ModuleCore,
    modules: HashMap<String, ModuleHandle<'d>>,
    event_mode: EventModeType,

}

impl<'d> ClusterLed<'d> {
    pub fn new(manuel_id: String) -> anyhow::Result<ClusterLed<'d>> {
        let mut cluster = ClusterLed {
            core: ModuleCore::new(ModuleType::LedCluster, &manuel_id),
            modules: HashMap::new(),
            event_mode:EventModeType::Register,
        };
        cluster.serialize();
        cluster.event_mode = EventModeType::State;

        Ok(cluster)
    }
    pub fn get_event(
        &self,
      
    ) -> anyhow::Result<OutgoingEvent> {
        let mut temp_child_payload: Vec<Value> = vec![];

        Ok(OutgoingEvent {
            id: self.id().to_string(),
            version: "1.0".to_string(),
            manuel_id: self.core.manuel_id.to_string(),

            kind: self.event_mode.clone(),
            moduletype: ModuleType::LedCluster,
            payload: json!([temp_child_payload]),
            generated_info: None,
            master_id: None,
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
    fn get_module_type(&self) -> &ModuleType {
        &self.core.module_type
    }
    fn handle_command(&mut self, command: &ModuleCommand) -> anyhow::Result<()> {
        match command {
            ModuleCommand::ClusterLeds(c) => match c {
                ClusterCommandPayload::SetAll { state } => {
                    self.serialize()?
                }
                ClusterCommandPayload::SetState { id, state } => {
                     self.serialize()?
                }
                ClusterCommandPayload::Toggle { id, state } => {
                     self.serialize()?
                }
                ClusterCommandPayload::ToggleAll => {
                     self.serialize()?
                }
            },

            _ => {
                // handle anything else
            }
        }
        Ok(())
    }

    fn serialize(
        &self
    ) -> anyhow::Result<()> {
        serde_json::to_string(&self.get_event()?)
            .map(|s| println!("{}", s))
            .unwrap_or_else(|e| println!("Failed to serialize JSON: {}", e));

        Ok(())
    }
}
