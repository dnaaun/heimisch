use derive_more::derive::{AsRef, Deref, From, Into};
use github_api::models::AuthorAssociation;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

use super::{
    milestone::MilestoneId, label::LabelId,
    repository::RepositoryId, team::TeamId, user::UserId,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AutoMerge {
    #[doc = "Commit message for the merge commit."]
    pub commit_message: String,
    #[doc = "Title for the merge commit message."]
    pub commit_title: String,
    pub enabled_by_id: UserId,
    #[doc = "The merge method to use."]
    pub merge_method: AutoMergeMergeMethod,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AutoMergeMergeMethod {
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "squash")]
    Squash,
    #[serde(rename = "rebase")]
    Rebase,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Link {
    pub href: String,
}

#[derive(From, Into, Deref, AsRef, Clone, Debug, Serialize, Deserialize)]
pub struct PullRequestId(i64);

#[derive(macros::TypesafeIdb, Deserialize, Serialize, Clone, Debug)]
pub struct PullRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub additions: i64,
    pub assignee_id: Option<UserId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee_ids: Option<Vec<UserId>>,
    #[doc = "How the author is associated with the repository."]
    pub author_association: AuthorAssociation,
    #[doc = "The status of auto merging a pull request."]
    pub auto_merge: Option<AutoMerge>,
    pub base: PullRequestBase,
    pub body: Option<String>,
    pub changed_files: i64,
    pub closed_at: Option<Timestamp>,
    pub comments: i64,
    pub comments_url: String,
    pub commits: i64,
    pub commits_url: String,
    pub created_at: Timestamp,
    pub deletions: i64,
    pub diff_url: String,
    #[doc = "Indicates whether or not the pull request is a draft."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub head: PullRequestHead,
    pub html_url: String,
    #[idb(id)]
    pub id: PullRequestId,
    pub issue_url: String,
    pub label_ids: Vec<LabelId>,
    #[serde(rename = "_links")]
    pub links: PullRequestLinks,
    pub locked: bool,
    #[doc = "Indicates whether maintainers can modify the pull request."]
    pub maintainer_can_modify: bool,
    pub merge_commit_sha: Option<String>,
    pub mergeable: Option<bool>,
    pub mergeable_state: String,
    pub merged: bool,
    pub merged_at: Option<Timestamp>,
    pub merged_by_id: Option<UserId>,
    pub milestone_id: Option<MilestoneId>,
    pub node_id: String,
    #[doc = "Number uniquely identifying the pull request within its repository."]
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_reviewer_ids: Option<Vec<UserId>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_team_ids: Option<Vec<TeamId>>,
    pub review_comment_url: String,
    pub review_comments: i64,
    pub review_comments_url: String,
    #[doc = "State of this Pull Request. Either `open` or `closed`."]
    pub state: PullRequestState,
    pub statuses_url: String,
    #[doc = "The title of the pull request."]
    pub title: String,
    pub updated_at: Timestamp,
    pub url: String,
    pub user_id: UserId,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PullRequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo_id: RepositoryId,
    pub sha: String,
    pub user_id: UserId,
}

/// NOTE: Maybe merge this will `PullRequestBase`?
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PullRequestHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo_id: RepositoryId,
    pub sha: String,
    pub user_id: UserId,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PullRequestLinks {
    pub comments: Link,
    pub commits: Link,
    pub html: Link,
    pub issue: Link,
    pub review_comment: Link,
    pub review_comments: Link,
    #[serde(rename = "self")]
    pub self_: Link,
    pub statuses: Link,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PullRequestState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
