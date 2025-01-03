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

/// StatusCheckPolicy : Status Check Policy
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatusCheckPolicy {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "strict")]
    pub strict: bool,
    #[serde(rename = "contexts")]
    pub contexts: Vec<String>,
    #[serde(rename = "checks")]
    pub checks: Vec<models::StatusCheckPolicyChecksInner>,
    #[serde(rename = "contexts_url")]
    pub contexts_url: String,
}

impl StatusCheckPolicy {
    /// Status Check Policy
    pub fn new(
        url: String,
        strict: bool,
        contexts: Vec<String>,
        checks: Vec<models::StatusCheckPolicyChecksInner>,
        contexts_url: String,
    ) -> StatusCheckPolicy {
        StatusCheckPolicy {
            url,
            strict,
            contexts,
            checks,
            contexts_url,
        }
    }
}
