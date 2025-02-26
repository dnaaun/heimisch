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

/// DependabotPublicKey : The public key used for setting Dependabot Secrets.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DependabotPublicKey {
    /// The identifier for the key.
    #[serde(rename = "key_id")]
    pub key_id: String,
    /// The Base64 encoded public key.
    #[serde(rename = "key")]
    pub key: String,
}

impl DependabotPublicKey {
    /// The public key used for setting Dependabot Secrets.
    pub fn new(key_id: String, key: String) -> DependabotPublicKey {
        DependabotPublicKey { key_id, key }
    }
}
