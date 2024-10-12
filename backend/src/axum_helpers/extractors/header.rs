use axum::extract::FromRequestParts;
use http::request::Parts;
use serde::de::DeserializeOwned;
use serde_json::{from_value, Map, Value};
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum HeaderError {
    Utf8(FromUtf8Error),
    SerdeJson(serde_json::Error),
}

impl From<FromUtf8Error> for HeaderError {
    fn from(value: FromUtf8Error) -> Self {
        HeaderError::Utf8(value)
    }
}

impl From<serde_json::Error> for HeaderError {
    fn from(value: serde_json::Error) -> Self {
        HeaderError::SerdeJson(value)
    }
}

/// TODO: THis has issues when non-string members of T are there. Write your own macro.
pub struct Header<T: DeserializeOwned>(pub T);

#[async_trait::async_trait]
impl<S, T: DeserializeOwned> FromRequestParts<S> for Header<T> {
    #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
    #[doc = " a kind of error that can be converted into a response."]
    type Rejection = crate::error::Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // NOTE: It's probably possible to avoid this indidrection through serde_json::Value.
        // Mentioning because this extractor will be used in the endpoint that definitely be the be
        // most hit.
        let map = parts
            .headers
            .iter()
            .map(|(k, v)| {
                Ok((
                    k.as_str().to_lowercase(),
                    Value::String(String::from_utf8(v.as_bytes().to_vec())?),
                ))
            })
            .collect::<Result<Map<_, _>, FromUtf8Error>>()
            .map_err(HeaderError::from)?;

        let inner = from_value(Value::Object(map)).map_err(HeaderError::from)?;

        Ok(Self(inner))
    }
}
