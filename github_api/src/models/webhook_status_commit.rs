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
pub struct WebhookStatusCommit {
    #[serde(rename = "author", deserialize_with = "Option::deserialize")]
    pub author: Option<Box<models::User8>>,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commit")]
    pub commit: Box<models::WebhookStatusCommitCommit>,
    #[serde(rename = "committer", deserialize_with = "Option::deserialize")]
    pub committer: Option<Box<models::User8>>,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "parents")]
    pub parents: Vec<models::WebhookStatusCommitParentsInner>,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "url")]
    pub url: String,
}

impl WebhookStatusCommit {
    pub fn new(
        author: Option<models::User8>,
        comments_url: String,
        commit: models::WebhookStatusCommitCommit,
        committer: Option<models::User8>,
        html_url: String,
        node_id: String,
        parents: Vec<models::WebhookStatusCommitParentsInner>,
        sha: String,
        url: String,
    ) -> WebhookStatusCommit {
        WebhookStatusCommit {
            author: if let Some(x) = author {
                Some(Box::new(x))
            } else {
                None
            },
            comments_url,
            commit: Box::new(commit),
            committer: if let Some(x) = committer {
                Some(Box::new(x))
            } else {
                None
            },
            html_url,
            node_id,
            parents,
            sha,
            url,
        }
    }
}
