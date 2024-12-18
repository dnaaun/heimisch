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
pub struct CopilotDotcomPullRequestsRepositoriesInner {
    /// Repository name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of users who generated pull request summaries using Copilot for Pull Requests in the given repository.
    #[serde(
        rename = "total_engaged_users",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_engaged_users: Option<i32>,
    /// List of model metrics for custom models and the default model.
    #[serde(rename = "models", skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<models::CopilotDotcomPullRequestsRepositoriesInnerModelsInner>>,
}

impl CopilotDotcomPullRequestsRepositoriesInner {
    pub fn new() -> CopilotDotcomPullRequestsRepositoriesInner {
        CopilotDotcomPullRequestsRepositoriesInner {
            name: None,
            total_engaged_users: None,
            models: None,
        }
    }
}
