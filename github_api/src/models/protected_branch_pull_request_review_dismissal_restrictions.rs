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
pub struct ProtectedBranchPullRequestReviewDismissalRestrictions {
    /// The list of users with review dismissal access.
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<models::SimpleUser>>,
    /// The list of teams with review dismissal access.
    #[serde(rename = "teams", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<models::Team>>,
    /// The list of apps with review dismissal access.
    #[serde(rename = "apps", skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<models::Integration>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "users_url", skip_serializing_if = "Option::is_none")]
    pub users_url: Option<String>,
    #[serde(rename = "teams_url", skip_serializing_if = "Option::is_none")]
    pub teams_url: Option<String>,
}

impl ProtectedBranchPullRequestReviewDismissalRestrictions {
    pub fn new() -> ProtectedBranchPullRequestReviewDismissalRestrictions {
        ProtectedBranchPullRequestReviewDismissalRestrictions {
            users: None,
            teams: None,
            apps: None,
            url: None,
            users_url: None,
            teams_url: None,
        }
    }
}
