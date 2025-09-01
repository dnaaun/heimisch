/// This abstraction was necessitated by the fact that `jiff::Timestamp` doesn't play nice with
/// `serde_wasm_bindgen`.
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Serde(value)
    }
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::Js(value)
    }
}
