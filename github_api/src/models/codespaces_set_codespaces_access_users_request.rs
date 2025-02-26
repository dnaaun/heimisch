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
pub struct CodespacesSetCodespacesAccessUsersRequest {
    /// The usernames of the organization members whose codespaces be billed to the organization.
    #[serde(rename = "selected_usernames")]
    pub selected_usernames: Vec<String>,
}

impl CodespacesSetCodespacesAccessUsersRequest {
    pub fn new(selected_usernames: Vec<String>) -> CodespacesSetCodespacesAccessUsersRequest {
        CodespacesSetCodespacesAccessUsersRequest { selected_usernames }
    }
}
