use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub enum Command {
    GetBootEntries,
}

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub enum Response {
    GetBootEntries(Result<Vec<String>, String>),
}

#[derive(Deserialize, Serialize, Debug, Clone, JsonSchema)]
pub enum Message {
    Command(Command),
    Response(Response),
}