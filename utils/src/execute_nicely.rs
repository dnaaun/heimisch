use std::{fmt::{Debug, Display}, str::FromStr};

pub trait DisplayDebug: std::fmt::Display + std::fmt::Debug {}
impl<T: std::fmt::Display + std::fmt::Debug> DisplayDebug for T {}

#[derive(Debug)]
pub struct ReqwestSendError {
    pub url: url::Url,
    /// Not just a straight up string in case the request body supports pretty printing (and maybe
    /// it's json, who knows).
    pub payload: Option<Box<dyn DisplayDebug + Send + Sync>>,
    pub request_error: reqwest::Error,
}

/// NOTE: Maybe this should be better?
impl Display for ReqwestSendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Debug>::fmt(self, f)
    }
}

impl std::error::Error for ReqwestSendError {}

#[allow(async_fn_in_trait)]
#[cfg_attr(feature = "ssr", async_trait::async_trait)]
pub trait ExecuteNicely {
    async fn execute_nicely(
        &self,
        request: reqwest::Request,
    ) -> std::result::Result<reqwest::Response, ReqwestSendError>;
}

#[cfg_attr(feature = "ssr", async_trait::async_trait)]
impl ExecuteNicely for reqwest::Client {
    /// Has a nicer error (at the cost of more clones).
    async fn execute_nicely(
        &self,
        request: reqwest::Request,
    ) -> std::result::Result<reqwest::Response, ReqwestSendError> {
        // NOTE: as_bytes() returns None in case the body is a stream/file, but I don't have
        // reqwests's `stream` feature on, so we should be good without taking care of that edge
        // case.
        let payload_bytes = request.body().and_then(|b| b.as_bytes());
        let payload =
            payload_bytes.map(
                |payload_bytes| match String::from_utf8(payload_bytes.to_vec()) {
                    Ok(string_payload) => match serde_json::Value::from_str(&string_payload) {
                        Ok(value) => Box::new(value) as Box<dyn DisplayDebug + Send + Sync>,
                        Err(_) => Box::new(string_payload),
                    },
                    Err(_) => Box::new("PAYLOAD IS SOME BINARY/NON-TEXTUAL VALUE"),
                },
            );

        let url = request.url().clone();

        match self.execute(request).await {
            Ok(resp) => Ok(resp),
            Err(request_error) => Err(ReqwestSendError {
                url,
                payload,
                request_error,
            }),
        }
    }
}
