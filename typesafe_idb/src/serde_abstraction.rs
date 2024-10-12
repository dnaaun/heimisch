/// This abstraction was necessitated by the fact that `jiff::Timestamp` doesn't play nice with
/// `serde_wasm_bindgen`.
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    Js(JsValue),
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

pub fn from_value<T>(value: JsValue) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let s = js_sys::JSON::stringify(&value)?;
    let s = s.as_string().unwrap();
    Ok(serde_json::from_str(&s)?)
}

pub fn to_value<T>(t: &T) -> Result<JsValue, Error>
where
    T: Serialize,
{
    let s = serde_json::to_string(t)?;
    Ok(js_sys::JSON::parse(&s)?)
}
