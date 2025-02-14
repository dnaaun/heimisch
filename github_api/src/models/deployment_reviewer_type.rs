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

/// DeploymentReviewerType : The type of reviewer.
/// The type of reviewer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DeploymentReviewerType {
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Team")]
    Team,
}

impl std::fmt::Display for DeploymentReviewerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "User"),
            Self::Team => write!(f, "Team"),
        }
    }
}

impl Default for DeploymentReviewerType {
    fn default() -> DeploymentReviewerType {
        Self::User
    }
}
