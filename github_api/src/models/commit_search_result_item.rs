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

/// CommitSearchResultItem : Commit Search Result Item
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitSearchResultItem {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commit")]
    pub commit: Box<models::CommitSearchResultItemCommit>,
    #[serde(rename = "author", deserialize_with = "Option::deserialize")]
    pub author: Option<Box<models::NullableSimpleUser>>,
    #[serde(rename = "committer", deserialize_with = "Option::deserialize")]
    pub committer: Option<Box<models::NullableGitUser>>,
    #[serde(rename = "parents")]
    pub parents: Vec<models::FileCommitCommitParentsInner>,
    #[serde(rename = "repository")]
    pub repository: Box<models::MinimalRepository>,
    #[serde(rename = "score")]
    pub score: f64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "text_matches", skip_serializing_if = "Option::is_none")]
    pub text_matches: Option<Vec<models::SearchResultTextMatchesInner>>,
}

impl CommitSearchResultItem {
    /// Commit Search Result Item
    pub fn new(
        url: String,
        sha: String,
        html_url: String,
        comments_url: String,
        commit: models::CommitSearchResultItemCommit,
        author: Option<models::NullableSimpleUser>,
        committer: Option<models::NullableGitUser>,
        parents: Vec<models::FileCommitCommitParentsInner>,
        repository: models::MinimalRepository,
        score: f64,
        node_id: String,
    ) -> CommitSearchResultItem {
        CommitSearchResultItem {
            url,
            sha,
            html_url,
            comments_url,
            commit: Box::new(commit),
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
            parents,
            repository: Box::new(repository),
            score,
            node_id,
            text_matches: None,
        }
    }
}
