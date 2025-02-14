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
pub struct SimpleCommit {
    #[serde(rename = "author")]
    pub author: Box<models::Committer>,
    #[serde(rename = "committer")]
    pub committer: Box<models::Committer>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
    #[serde(rename = "tree_id")]
    pub tree_id: String,
}

impl SimpleCommit {
    pub fn new(
        author: models::Committer,
        committer: models::Committer,
        id: String,
        message: String,
        timestamp: String,
        tree_id: String,
    ) -> SimpleCommit {
        SimpleCommit {
            author: Box::new(author),
            committer: Box::new(committer),
            id,
            message,
            timestamp,
            tree_id,
        }
    }
}
