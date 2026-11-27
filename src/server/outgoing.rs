use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct ResponseError {
    code: i64,
    message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl ResponseError {
    pub fn new(code: i64, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OutgoingResponse {
    #[serde(rename = "jsonrpc")]
    protocol_version: &'static str,

    #[serde(rename = "id")]
    identifier: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<ResponseError>,
}

impl OutgoingResponse {
    pub fn success(identifier: Value, result: Value) -> Self {
        Self {
            protocol_version: "2.0",
            identifier,
            result: Some(result),
            error: None,
        }
    }

    pub fn failure(identifier: Value, error: ResponseError) -> Self {
        Self {
            protocol_version: "2.0",
            identifier,
            result: None,
            error: Some(error),
        }
    }
}
