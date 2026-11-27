use crate::server::{INVALID_REQUEST, METHOD_NOT_FOUND, SERVER_NOT_INITIALIZED};

use serde_json::{Value, json};

use crate::server::outgoing::ResponseError;

#[derive(Debug, PartialEq)]
enum LifecycleState {
    Uninitialized,
    Running,
    ShutdownRequested,
}

pub struct Server {
    lifecycle_state: LifecycleState,
}

impl Server {
    pub fn new() -> Self {
        Self {
            lifecycle_state: LifecycleState::Uninitialized,
        }
    }

    /// Handles a request that expects a response.
    pub fn handle_request(
        &mut self,
        method: &str,
        parameters: Value,
    ) -> Result<Value, ResponseError> {
        match (&self.lifecycle_state, method) {
            (LifecycleState::Uninitialized, "initialize") => {
                self.lifecycle_state = LifecycleState::Running;

                // TODO: Send capabilities.
                Ok(json!({ "capabilities": {} }))
            }

            (LifecycleState::Uninitialized, _) => Err(ResponseError::new(
                SERVER_NOT_INITIALIZED,
                "server has not been initialized.",
            )),

            (LifecycleState::Running, "shutdown") => {
                self.lifecycle_state = LifecycleState::ShutdownRequested;

                Ok(Value::Null)
            }

            (LifecycleState::ShutdownRequested, _) => Err(ResponseError::new(
                INVALID_REQUEST,
                "server is shutting down.",
            )),

            (LifecycleState::Running, _) => self.handle_method(method, parameters),
        }
    }

    /// Handles a notification (no response is ever sent).
    /// Returns `Some(exit_code)` when the server should terminate.
    pub fn handle_notification(&mut self, method: &str, _parameters: Value) -> Option<i32> {
        match method {
            "exit" => Some(
                if self.lifecycle_state == LifecycleState::ShutdownRequested {
                    0
                } else {
                    1
                },
            ),

            _ => None,
        }
    }

    pub fn handle_method(
        &mut self,
        method: &str,
        _parameters: Value,
    ) -> Result<Value, ResponseError> {
        match method {
            // TODO: Methods
            "ping" => Ok(json!("pong")),

            _ => Err(ResponseError::new(
                METHOD_NOT_FOUND,
                format!("method not found: \"{method}\"."),
            )),
        }
    }
}
