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
pub struct FileCommitCommitTree {
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "sha", skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
}

impl FileCommitCommitTree {
    pub fn new() -> FileCommitCommitTree {
        FileCommitCommitTree {
            url: None,
            sha: None,
        }
    }
}
