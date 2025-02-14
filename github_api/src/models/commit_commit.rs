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
pub struct CommitCommit {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "author", deserialize_with = "Option::deserialize")]
    pub author: Option<Box<models::NullableGitUser>>,
    #[serde(rename = "committer", deserialize_with = "Option::deserialize")]
    pub committer: Option<Box<models::NullableGitUser>>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "comment_count")]
    pub comment_count: i32,
    #[serde(rename = "tree")]
    pub tree: Box<models::CommitCommitTree>,
    #[serde(rename = "verification", skip_serializing_if = "Option::is_none")]
    pub verification: Option<Box<models::Verification>>,
}

impl CommitCommit {
    pub fn new(
        url: String,
        author: Option<models::NullableGitUser>,
        committer: Option<models::NullableGitUser>,
        message: String,
        comment_count: i32,
        tree: models::CommitCommitTree,
    ) -> CommitCommit {
        CommitCommit {
            url,
            author: if let Some(x) = author {
                Some(Box::new(x))
            } else {
                None
            },
            committer: if let Some(x) = committer {
                Some(Box::new(x))
            } else {
                None
            },
            message,
            comment_count,
            tree: Box::new(tree),
            verification: None,
        }
    }
}
