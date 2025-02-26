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
pub struct Commit1 {
    /// An array of files added in the commit.
    #[serde(rename = "added", skip_serializing_if = "Option::is_none")]
    pub added: Option<Vec<String>>,
    #[serde(rename = "author")]
    pub author: Box<models::Committer>,
    #[serde(rename = "committer")]
    pub committer: Box<models::Committer>,
    /// Whether this commit is distinct from any that have been pushed before.
    #[serde(rename = "distinct")]
    pub distinct: bool,
    #[serde(rename = "id")]
    pub id: String,
    /// The commit message.
    #[serde(rename = "message")]
    pub message: String,
    /// An array of files modified by the commit.
    #[serde(rename = "modified", skip_serializing_if = "Option::is_none")]
    pub modified: Option<Vec<String>>,
    /// An array of files removed in the commit.
    #[serde(rename = "removed", skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<String>>,
    /// The ISO 8601 timestamp of the commit.
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "tree_id")]
    pub tree_id: String,
    /// URL that points to the commit API resource.
    #[serde(rename = "url")]
    pub url: String,
}

impl Commit1 {
    pub fn new(
        author: models::Committer,
        committer: models::Committer,
        distinct: bool,
        id: String,
        message: String,
        timestamp: String,
        tree_id: String,
        url: String,
    ) -> Commit1 {
        Commit1 {
            added: None,
            author: Box::new(author),
            committer: Box::new(committer),
            distinct,
            id,
            message,
            modified: None,
            removed: None,
            timestamp,
            tree_id,
            url,
        }
    }
}
