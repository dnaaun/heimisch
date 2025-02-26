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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PullRequestRequestedReviewersInner {
    User4(Box<models::User4>),
    Team1(Box<models::Team1>),
}

impl Default for PullRequestRequestedReviewersInner {
    fn default() -> Self {
        Self::User4(Default::default())
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Bot")]
    Bot,
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Organization")]
    Organization,
    #[serde(rename = "Mannequin")]
    Mannequin,
}

impl Default for Type {
    fn default() -> Type {
        Self::Bot
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Privacy {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "secret")]
    Secret,
}

impl Default for Privacy {
    fn default() -> Privacy {
        Self::Open
    }
}
