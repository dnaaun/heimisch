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

/// PullRequestReviewRequest : Pull Request Review Request
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequestReviewRequest {
    #[serde(rename = "users")]
    pub users: Vec<models::SimpleUser>,
    #[serde(rename = "teams")]
    pub teams: Vec<models::Team>,
}

impl PullRequestReviewRequest {
    /// Pull Request Review Request
    pub fn new(
        users: Vec<models::SimpleUser>,
        teams: Vec<models::Team>,
    ) -> PullRequestReviewRequest {
        PullRequestReviewRequest { users, teams }
    }
}
