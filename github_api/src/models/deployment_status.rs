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

/// DeploymentStatus : The status of a deployment.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeploymentStatus {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// The state of the status.
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "creator", deserialize_with = "Option::deserialize")]
    pub creator: Option<Box<models::NullableSimpleUser>>,
    /// A short description of the status.
    #[serde(rename = "description")]
    pub description: String,
    /// The environment of the deployment that the status is for.
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// Closing down notice: the URL to associate with this status.
    #[serde(rename = "target_url")]
    pub target_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "deployment_url")]
    pub deployment_url: String,
    #[serde(rename = "repository_url")]
    pub repository_url: String,
    /// The URL for accessing your environment.
    #[serde(rename = "environment_url", skip_serializing_if = "Option::is_none")]
    pub environment_url: Option<String>,
    /// The URL to associate with this status.
    #[serde(rename = "log_url", skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    #[serde(
        rename = "performed_via_github_app",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub performed_via_github_app: Option<Option<Box<models::NullableIntegration>>>,
}

impl DeploymentStatus {
    /// The status of a deployment.
    pub fn new(
        url: String,
        id: i64,
        node_id: String,
        state: State,
        creator: Option<models::NullableSimpleUser>,
        description: String,
        target_url: String,
        created_at: String,
        updated_at: String,
        deployment_url: String,
        repository_url: String,
    ) -> DeploymentStatus {
        DeploymentStatus {
            url,
            id,
            node_id,
            state,
            creator: if let Some(x) = creator {
                Some(Box::new(x))
            } else {
                None
            },
            description,
            environment: None,
            target_url,
            created_at,
            updated_at,
            deployment_url,
            repository_url,
            environment_url: None,
            log_url: None,
            performed_via_github_app: None,
        }
    }
}
/// The state of the status.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
}

impl Default for State {
    fn default() -> State {
        Self::Error
    }
}
