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
pub struct WebhookPush {
    /// The SHA of the most recent commit on `ref` after the push.
    #[serde(rename = "after")]
    pub after: String,
    #[serde(rename = "base_ref", deserialize_with = "Option::deserialize")]
    pub base_ref: Option<String>,
    /// The SHA of the most recent commit on `ref` before the push.
    #[serde(rename = "before")]
    pub before: String,
    /// An array of commit objects describing the pushed commits. (Pushed commits are all commits that are included in the `compare` between the `before` commit and the `after` commit.) The array includes a maximum of 2048 commits. If necessary, you can use the [Commits API](https://docs.github.com/rest/commits) to fetch additional commits.
    #[serde(rename = "commits")]
    pub commits: Vec<models::Commit>,
    /// URL that shows the changes in this `ref` update, from the `before` commit to the `after` commit. For a newly created `ref` that is directly based on the default branch, this is the comparison between the head of the default branch and the `after` commit. Otherwise, this shows all commits until the `after` commit.
    #[serde(rename = "compare")]
    pub compare: String,
    /// Whether this push created the `ref`.
    #[serde(rename = "created")]
    pub created: bool,
    /// Whether this push deleted the `ref`.
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "enterprise", skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Box<models::EnterpriseWebhooks>>,
    /// Whether this push was a force push of the `ref`.
    #[serde(rename = "forced")]
    pub forced: bool,
    #[serde(rename = "head_commit", deserialize_with = "Option::deserialize")]
    pub head_commit: Option<Box<models::Commit1>>,
    #[serde(rename = "installation", skip_serializing_if = "Option::is_none")]
    pub installation: Option<Box<models::SimpleInstallation>>,
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationSimpleWebhooks>>,
    #[serde(rename = "pusher")]
    pub pusher: Box<models::Committer1>,
    /// The full git ref that was pushed. Example: `refs/heads/main` or `refs/tags/v3.14.1`.
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "repository")]
    pub repository: Box<models::Repository2>,
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<Box<models::SimpleUser>>,
}

impl WebhookPush {
    pub fn new(
        after: String,
        base_ref: Option<String>,
        before: String,
        commits: Vec<models::Commit>,
        compare: String,
        created: bool,
        deleted: bool,
        forced: bool,
        head_commit: Option<models::Commit1>,
        pusher: models::Committer1,
        r#ref: String,
        repository: models::Repository2,
    ) -> WebhookPush {
        WebhookPush {
            after,
            base_ref,
            before,
            commits,
            compare,
            created,
            deleted,
            enterprise: None,
            forced,
            head_commit: if let Some(x) = head_commit {
                Some(Box::new(x))
            } else {
                None
            },
            installation: None,
            organization: None,
            pusher: Box::new(pusher),
            r#ref,
            repository: Box::new(repository),
            sender: None,
        }
    }
}
