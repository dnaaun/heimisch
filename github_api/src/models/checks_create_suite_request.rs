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
pub struct ChecksCreateSuiteRequest {
    /// The sha of the head commit.
    #[serde(rename = "head_sha")]
    pub head_sha: String,
}

impl ChecksCreateSuiteRequest {
    pub fn new(head_sha: String) -> ChecksCreateSuiteRequest {
        ChecksCreateSuiteRequest { head_sha }
    }
}
