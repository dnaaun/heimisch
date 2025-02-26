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
pub struct ProtectedBranchRequiredConversationResolution {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl ProtectedBranchRequiredConversationResolution {
    pub fn new() -> ProtectedBranchRequiredConversationResolution {
        ProtectedBranchRequiredConversationResolution { enabled: None }
    }
}
