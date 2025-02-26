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

/// WebhookIssueCommentCreatedIssue : The [issue](https://docs.github.com/rest/issues/issues#get-an-issue) the comment belongs to.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookIssueCommentCreatedIssue {
    #[serde(
        rename = "active_lock_reason",
        deserialize_with = "Option::deserialize"
    )]
    pub active_lock_reason: Option<String>,
    #[serde(rename = "assignee", deserialize_with = "Option::deserialize")]
    pub assignee: Option<Box<models::User4>>,
    #[serde(rename = "assignees")]
    pub assignees: Vec<serde_json::Value>,
    #[serde(rename = "author_association")]
    pub author_association: String,
    #[serde(rename = "body", deserialize_with = "Option::deserialize")]
    pub body: Option<String>,
    #[serde(rename = "closed_at", deserialize_with = "Option::deserialize")]
    pub closed_at: Option<String>,
    #[serde(rename = "comments")]
    pub comments: i32,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "draft", skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "labels")]
    pub labels: Vec<models::Label>,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "locked")]
    pub locked: bool,
    #[serde(rename = "milestone", deserialize_with = "Option::deserialize")]
    pub milestone: Option<serde_json::Value>,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(
        rename = "performed_via_github_app",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub performed_via_github_app: Option<Option<serde_json::Value>>,
    #[serde(rename = "pull_request", skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<Box<models::WebhooksIssuePullRequest>>,
    #[serde(rename = "reactions")]
    pub reactions: Box<models::WebhookIssueCommentCreatedIssueAllOfReactions>,
    #[serde(rename = "repository_url")]
    pub repository_url: String,
    /// State of the issue; either 'open' or 'closed'
    #[serde(rename = "state")]
    pub state: State,
    #[serde(
        rename = "state_reason",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub state_reason: Option<Option<String>>,
    #[serde(rename = "timeline_url", skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "user")]
    pub user: Box<models::WebhookIssueCommentCreatedIssueAllOfUser>,
}

impl WebhookIssueCommentCreatedIssue {
    /// The [issue](https://docs.github.com/rest/issues/issues#get-an-issue) the comment belongs to.
    pub fn new(
        active_lock_reason: Option<String>,
        assignee: Option<models::User4>,
        assignees: Vec<serde_json::Value>,
        author_association: String,
        body: Option<String>,
        closed_at: Option<String>,
        comments: i32,
        comments_url: String,
        created_at: String,
        events_url: String,
        html_url: String,
        id: i32,
        labels: Vec<models::Label>,
        labels_url: String,
        locked: bool,
        milestone: Option<serde_json::Value>,
        node_id: String,
        number: i32,
        reactions: models::WebhookIssueCommentCreatedIssueAllOfReactions,
        repository_url: String,
        state: State,
        title: String,
        updated_at: String,
        url: String,
        user: models::WebhookIssueCommentCreatedIssueAllOfUser,
    ) -> WebhookIssueCommentCreatedIssue {
        WebhookIssueCommentCreatedIssue {
            active_lock_reason,
            assignee: if let Some(x) = assignee {
                Some(Box::new(x))
            } else {
                None
            },
            assignees,
            author_association,
            body,
            closed_at,
            comments,
            comments_url,
            created_at,
            draft: None,
            events_url,
            html_url,
            id,
            labels,
            labels_url,
            locked,
            milestone,
            node_id,
            number,
            performed_via_github_app: None,
            pull_request: None,
            reactions: Box::new(reactions),
            repository_url,
            state,
            state_reason: None,
            timeline_url: None,
            title,
            updated_at,
            url,
            user: Box::new(user),
        }
    }
}
/// State of the issue; either 'open' or 'closed'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

impl Default for State {
    fn default() -> State {
        Self::Open
    }
}
