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
pub struct StatusCheckPolicyChecksInner {
    #[serde(rename = "context")]
    pub context: String,
    #[serde(rename = "app_id", deserialize_with = "Option::deserialize")]
    pub app_id: Option<i32>,
}

impl StatusCheckPolicyChecksInner {
    pub fn new(context: String, app_id: Option<i32>) -> StatusCheckPolicyChecksInner {
        StatusCheckPolicyChecksInner { context, app_id }
    }
}
