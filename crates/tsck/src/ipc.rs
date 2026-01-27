#![allow(unused)]
use anyhow::bail;
use serde::{Deserialize, Serialize};

use crate::{dp, log_error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcRequest<T = serde_json::Value> {
    pub id: i32,
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> IpcRequest<T>
where
    T: Serialize + Deserialize<'static>,
{
    pub fn to_response_success<X>(&self, data: X) -> String
    where
        X: Serialize + Deserialize<'static>,
    {
        {
            let response = IpcResponse::success(self.id, data);

            let json = serde_json::to_string(&response).unwrap_or("undefined".into());
            format!(
                r#"
                 (function() {{
                     const response = {};
                     if (window.__ipc_response_handler) {{
                         window.__ipc_response_handler(response);
                     }}
                 }})();
                 "#,
                json
            )
        }
    }
    pub fn to_response_error(&self, error_data: impl Into<String>) -> String {
        {
            let response = IpcResponse::<String>::error(self.id, error_data);
            let json = serde_json::to_string(&response).unwrap_or("undefined".into());
            format!(
                r#"
                 (function() {{
                     const response = {};
                     if (window.__ipc_response_handler) {{
                         window.__ipc_response_handler(response);
                     }}
                 }})();
                 "#,
                json
            )
        }
    }
}

/// Response sent from Rust to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse<T = serde_json::Value> {
    pub id: i32,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T: Serialize> IpcResponse<T> {
    pub fn success(id: i32, data: T) -> Self {
        Self {
            id,
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(id: i32, error: impl Into<String>) -> Self {
        Self {
            id,
            success: false,
            data: None,
            error: Some(error.into()),
        }
    }
}

pub struct IpcHelper;
impl IpcHelper {
    pub fn compile(
        payload_type: impl Into<String>,
        data: impl Serialize,
    ) -> anyhow::Result<String> {
        let payload = serde_json::to_string(&payload_type.into())?;
        let json = match serde_json::to_string(&data) {
            Ok(j) => j,
            Err(e) => {
                bail!("Parsing failed... {} ", e)
            }
        };
        Ok(format!(
            "window.dispatchEvent(new CustomEvent({}, {{ detail: {} }}));",
            payload, json
        ))
    }
}
