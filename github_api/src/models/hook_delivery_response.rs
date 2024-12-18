/*
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * The version of the OpenAPI document: 1.1.4
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HookDeliveryResponse {
    /// The response headers received when the delivery was made.
    #[serde(rename = "headers", deserialize_with = "Option::deserialize")]
    pub headers: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The response payload received.
    #[serde(rename = "payload", deserialize_with = "Option::deserialize")]
    pub payload: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl HookDeliveryResponse {
    pub fn new(
        headers: Option<std::collections::HashMap<String, serde_json::Value>>,
        payload: Option<std::collections::HashMap<String, serde_json::Value>>,
    ) -> HookDeliveryResponse {
        HookDeliveryResponse { headers, payload }
    }
}
