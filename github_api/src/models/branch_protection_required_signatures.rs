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
pub struct BranchProtectionRequiredSignatures {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

impl BranchProtectionRequiredSignatures {
    pub fn new(url: String, enabled: bool) -> BranchProtectionRequiredSignatures {
        BranchProtectionRequiredSignatures { url, enabled }
    }
}
