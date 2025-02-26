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

/// MaxFileSize : Prevent commits that exceed a specified file size limit from being pushed to the commit.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MaxFileSize {
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Box<models::MaxFileSizeParameters>>,
}

impl MaxFileSize {
    /// Prevent commits that exceed a specified file size limit from being pushed to the commit.
    pub fn new(r#type: Type) -> MaxFileSize {
        MaxFileSize {
            r#type,
            parameters: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "max_file_size")]
    MaxFileSize,
}

impl Default for Type {
    fn default() -> Type {
        Self::MaxFileSize
    }
}
