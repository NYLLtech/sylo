use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct IncomingMessage {
    #[serde(rename = "jsonrpc")]
    pub protocol_version: String,

    #[serde(rename = "id", default)]
    pub identifier: Option<Value>,

    pub method: String,

    #[serde(rename = "params", default)]
    pub parameters: Value,
}
