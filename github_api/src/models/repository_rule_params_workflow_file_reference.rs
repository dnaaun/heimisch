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

/// RepositoryRuleParamsWorkflowFileReference : A workflow that must run for this rule to pass
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RepositoryRuleParamsWorkflowFileReference {
    /// The path to the workflow file
    #[serde(rename = "path")]
    pub path: String,
    /// The ref (branch or tag) of the workflow file to use
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    /// The ID of the repository where the workflow is defined
    #[serde(rename = "repository_id")]
    pub repository_id: i32,
    /// The commit SHA of the workflow file to use
    #[serde(rename = "sha", skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
}

impl RepositoryRuleParamsWorkflowFileReference {
    /// A workflow that must run for this rule to pass
    pub fn new(path: String, repository_id: i32) -> RepositoryRuleParamsWorkflowFileReference {
        RepositoryRuleParamsWorkflowFileReference {
            path,
            r#ref: None,
            repository_id,
            sha: None,
        }
    }
}
