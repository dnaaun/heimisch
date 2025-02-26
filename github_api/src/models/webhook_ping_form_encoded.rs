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

/// WebhookPingFormEncoded : The webhooks ping payload encoded with URL encoding.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookPingFormEncoded {
    /// A URL-encoded string of the ping JSON payload. The decoded payload is a JSON object.
    #[serde(rename = "payload")]
    pub payload: String,
}

impl WebhookPingFormEncoded {
    /// The webhooks ping payload encoded with URL encoding.
    pub fn new(payload: String) -> WebhookPingFormEncoded {
        WebhookPingFormEncoded { payload }
    }
}
