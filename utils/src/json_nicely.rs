use serde::de::DeserializeOwned;
use std::str::FromStr;

use crate::DisplayDebug;

#[derive(Debug)]
pub struct ReqwestJsonError {
    /// Not just a straight up string in case the body supports pretty printing (and maybe
    /// it's json, who knows).
    pub body: Option<Box<dyn DisplayDebug + Send>>,
    pub reqwest_error: Option<reqwest::Error>,
    pub serde_error: Option<serde_json::error::Error>,
}

#[async_trait::async_trait]
pub trait JsonNicely {
    async fn json_nicely<T: DeserializeOwned>(
        self: Self,
    ) -> std::result::Result<T, ReqwestJsonError>;
}

#[async_trait::async_trait]
impl JsonNicely for reqwest::Response {
    /// Has a nicer error (at the cost of more clones).
    async fn json_nicely<T: DeserializeOwned>(
        self: Self,
    ) -> std::result::Result<T, ReqwestJsonError> {
        let body_bytes = match self.bytes().await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(ReqwestJsonError {
                    body: None,
                    reqwest_error: Some(err),
                    serde_error: None,
                })
            }
        };

        let body = match String::from_utf8(body_bytes.to_vec()) {
            Ok(string_body) => match serde_json::Value::from_str(&string_body) {
                Ok(value) => Box::new(value) as Box<dyn DisplayDebug + Send>,
                Err(_) => Box::new(string_body),
            },
            Err(_) => Box::new("BODY IS SOME BINARY/NON-TEXTUAL VALUE"),
        };

        match serde_json::de::from_slice(&body_bytes) {
            Ok(resp) => Ok(resp),
            Err(error) => Err(ReqwestJsonError {
                body: Some(body),
                reqwest_error: None,
                serde_error: Some(error),
            }),
        }
    }
}
