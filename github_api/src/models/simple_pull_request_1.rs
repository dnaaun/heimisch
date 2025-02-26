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
pub struct SimplePullRequest1 {
    #[serde(rename = "_links")]
    pub _links: Box<models::WebhooksPullRequest5Links>,
    #[serde(
        rename = "active_lock_reason",
        deserialize_with = "Option::deserialize"
    )]
    pub active_lock_reason: Option<ActiveLockReason>,
    #[serde(rename = "assignee", deserialize_with = "Option::deserialize")]
    pub assignee: Option<Box<models::User4>>,
    #[serde(rename = "assignees")]
    pub assignees: Vec<models::User5>,
    /// How the author is associated with the repository.
    #[serde(rename = "author_association")]
    pub author_association: AuthorAssociation,
    #[serde(rename = "auto_merge", deserialize_with = "Option::deserialize")]
    pub auto_merge: Option<Box<models::PullRequestAutoMerge>>,
    #[serde(rename = "base")]
    pub base: Box<models::SimplePullRequest1Base>,
    #[serde(rename = "body", deserialize_with = "Option::deserialize")]
    pub body: Option<String>,
    #[serde(rename = "closed_at", deserialize_with = "Option::deserialize")]
    pub closed_at: Option<String>,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "diff_url")]
    pub diff_url: String,
    #[serde(rename = "draft")]
    pub draft: bool,
    #[serde(rename = "head")]
    pub head: Box<models::SimplePullRequest1Head>,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "issue_url")]
    pub issue_url: String,
    #[serde(rename = "labels")]
    pub labels: Vec<models::Label>,
    #[serde(rename = "locked")]
    pub locked: bool,
    #[serde(rename = "merge_commit_sha", deserialize_with = "Option::deserialize")]
    pub merge_commit_sha: Option<String>,
    #[serde(rename = "merged_at", deserialize_with = "Option::deserialize")]
    pub merged_at: Option<String>,
    #[serde(rename = "milestone", deserialize_with = "Option::deserialize")]
    pub milestone: Option<Box<models::Milestone>>,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "patch_url")]
    pub patch_url: String,
    #[serde(rename = "requested_reviewers")]
    pub requested_reviewers: Vec<models::PullRequestRequestedReviewersInner>,
    #[serde(rename = "requested_teams")]
    pub requested_teams: Vec<models::Team>,
    #[serde(rename = "review_comment_url")]
    pub review_comment_url: String,
    #[serde(rename = "review_comments_url")]
    pub review_comments_url: String,
    #[serde(rename = "state")]
    pub state: State,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::User3>>,
}

impl SimplePullRequest1 {
    pub fn new(
        _links: models::WebhooksPullRequest5Links,
        active_lock_reason: Option<ActiveLockReason>,
        assignee: Option<models::User4>,
        assignees: Vec<models::User5>,
        author_association: AuthorAssociation,
        auto_merge: Option<models::PullRequestAutoMerge>,
        base: models::SimplePullRequest1Base,
        body: Option<String>,
        closed_at: Option<String>,
        comments_url: String,
        commits_url: String,
        created_at: String,
        diff_url: String,
        draft: bool,
        head: models::SimplePullRequest1Head,
        html_url: String,
        id: i32,
        issue_url: String,
        labels: Vec<models::Label>,
        locked: bool,
        merge_commit_sha: Option<String>,
        merged_at: Option<String>,
        milestone: Option<models::Milestone>,
        node_id: String,
        number: i32,
        patch_url: String,
        requested_reviewers: Vec<models::PullRequestRequestedReviewersInner>,
        requested_teams: Vec<models::Team>,
        review_comment_url: String,
        review_comments_url: String,
        state: State,
        statuses_url: String,
        title: String,
        updated_at: String,
        url: String,
        user: Option<models::User3>,
    ) -> SimplePullRequest1 {
        SimplePullRequest1 {
            _links: Box::new(_links),
            active_lock_reason,
            assignee: if let Some(x) = assignee {
                Some(Box::new(x))
            } else {
                None
            },
            assignees,
            author_association,
            auto_merge: if let Some(x) = auto_merge {
                Some(Box::new(x))
            } else {
                None
            },
            base: Box::new(base),
            body,
            closed_at,
            comments_url,
            commits_url,
            created_at,
            diff_url,
            draft,
            head: Box::new(head),
            html_url,
            id,
            issue_url,
            labels,
            locked,
            merge_commit_sha,
            merged_at,
            milestone: if let Some(x) = milestone {
                Some(Box::new(x))
            } else {
                None
            },
            node_id,
            number,
            patch_url,
            requested_reviewers,
            requested_teams,
            review_comment_url,
            review_comments_url,
            state,
            statuses_url,
            title,
            updated_at,
            url,
            user: if let Some(x) = user {
                Some(Box::new(x))
            } else {
                None
            },
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ActiveLockReason {
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "off-topic")]
    OffTopic,
    #[serde(rename = "too heated")]
    TooHeated,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "null")]
    Null,
}

impl Default for ActiveLockReason {
    fn default() -> ActiveLockReason {
        Self::Resolved
    }
}
/// How the author is associated with the repository.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AuthorAssociation {
    #[serde(rename = "COLLABORATOR")]
    Collaborator,
    #[serde(rename = "CONTRIBUTOR")]
    Contributor,
    #[serde(rename = "FIRST_TIMER")]
    FirstTimer,
    #[serde(rename = "FIRST_TIME_CONTRIBUTOR")]
    FirstTimeContributor,
    #[serde(rename = "MANNEQUIN")]
    Mannequin,
    #[serde(rename = "MEMBER")]
    Member,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "OWNER")]
    Owner,
}

impl Default for AuthorAssociation {
    fn default() -> AuthorAssociation {
        Self::Collaborator
    }
}
///
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
