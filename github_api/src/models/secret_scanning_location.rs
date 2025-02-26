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
pub struct SecretScanningLocation {
    /// The location type. Because secrets may be found in different types of resources (ie. code, comments, issues, pull requests, discussions), this field identifies the type of resource where the secret was found.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Box<models::SecretScanningLocationDetails>>,
}

impl SecretScanningLocation {
    pub fn new() -> SecretScanningLocation {
        SecretScanningLocation {
            r#type: None,
            details: None,
        }
    }
}
/// The location type. Because secrets may be found in different types of resources (ie. code, comments, issues, pull requests, discussions), this field identifies the type of resource where the secret was found.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "commit")]
    Commit,
    #[serde(rename = "wiki_commit")]
    WikiCommit,
    #[serde(rename = "issue_title")]
    IssueTitle,
    #[serde(rename = "issue_body")]
    IssueBody,
    #[serde(rename = "issue_comment")]
    IssueComment,
    #[serde(rename = "discussion_title")]
    DiscussionTitle,
    #[serde(rename = "discussion_body")]
    DiscussionBody,
    #[serde(rename = "discussion_comment")]
    DiscussionComment,
    #[serde(rename = "pull_request_title")]
    PullRequestTitle,
    #[serde(rename = "pull_request_body")]
    PullRequestBody,
    #[serde(rename = "pull_request_comment")]
    PullRequestComment,
    #[serde(rename = "pull_request_review")]
    PullRequestReview,
    #[serde(rename = "pull_request_review_comment")]
    PullRequestReviewComment,
}

impl Default for Type {
    fn default() -> Type {
        Self::Commit
    }
}
