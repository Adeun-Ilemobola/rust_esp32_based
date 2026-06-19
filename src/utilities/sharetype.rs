use serde::Deserialize;

#[derive(serde::Deserialize)]
pub enum EspCommand {
    SetLed { id: u8, value: u32 },
    Ping,
}

#[derive(Debug, Deserialize)]
pub struct IncomingCommand {
    pub kind: String,
    pub id: String,

    #[serde(flatten)]
    pub command: ModuleCommand,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "moduletype", content = "payload", rename_all = "snake_case")]
pub enum ModuleCommand {
    Led(LedCommandPayload),
}

#[derive(Debug, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum LedCommandPayload {
    SetState { state: u32 },
    Toggle,
}