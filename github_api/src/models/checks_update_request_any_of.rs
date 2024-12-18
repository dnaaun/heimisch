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
pub struct ChecksUpdateRequestAnyOf {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl ChecksUpdateRequestAnyOf {
    pub fn new() -> ChecksUpdateRequestAnyOf {
        ChecksUpdateRequestAnyOf { status: None }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "completed")]
    Completed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Completed
    }
}
