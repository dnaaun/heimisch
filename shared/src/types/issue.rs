use derive_more::derive::{AsRef, Deref, From, Into};
use github_api::models::issue::{ActiveLockReason, AuthorAssociation, State};
use github_api::models::nullable_issue::StateReason;
use github_api::models::{Label, Reactions, WebhooksIssuePullRequest};
use jiff::Timestamp;
use macros::AvailMerge;
use serde::{Deserialize, Serialize};

use crate::avail::Avail;

use super::github_app::GithubAppId;
use super::milestone::MilestoneId;
use super::repository::RepositoryId;
use super::user::UserId;

#[derive(
    From,
    Into,
    Deref,
    AsRef,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    Copy,
    Default,
)]
pub struct IssueId(i64);

#[derive(macros::TypesafeIdb, Deserialize, Serialize, Clone, Debug, AvailMerge)]
pub struct Issue {
    pub active_lock_reason: Avail<Option<ActiveLockReason>>,
    pub assignee_id: Avail<Option<UserId>>,

    pub assignee_ids: Avail<Vec<UserId>>,

    #[doc = "How the author is associated with the repository."]
    pub author_association: Avail<AuthorAssociation>,

    #[doc = "Contents of the issue"]
    pub body: Avail<Option<String>>,

    pub body_html: Avail<Option<String>>,

    pub body_text: Avail<Option<String>>,

    pub closed_at: Avail<Option<Timestamp>>,

    pub closed_by_id: Avail<Option<UserId>>,

    pub comments: Avail<i64>,

    pub comments_url: Avail<String>,

    pub created_at: Avail<Timestamp>,

    pub draft: Avail<Option<bool>>,

    pub events_url: Avail<String>,

    pub html_url: Avail<String>,

    #[idb(id)]
    pub id: IssueId,

    #[doc = "Labels to associate with this issue; pass one or more label names to replace the set of labels on this issue; send an empty array to clear all labels from the issue; note that the labels are silently dropped for users without push access to the repository"]
    pub labels: Avail<Vec<Label>>,

    pub labels_url: Avail<String>,

    pub locked: Avail<bool>,

    pub milestone_id: Avail<Option<MilestoneId>>,

    pub node_id: Avail<String>,

    #[doc = "Number uniquely identifying the issue within its repository"]
    #[idb(index)]
    pub number: i64,

    pub performed_via_github_app_id: Avail<Option<GithubAppId>>,

    pub pull_request: Avail<Option<WebhooksIssuePullRequest>>,

    pub reactions: Avail<Reactions>,

    #[idb(index)]
    pub repository_id: RepositoryId,

    pub repository_url: Avail<String>,

    #[doc = "State of the issue; either 'open' or 'closed'"]
    pub state: Avail<State>,

    #[doc = "The reason for the current state"]
    pub state_reason: Avail<Option<StateReason>>,

    pub timeline_url: Avail<Option<String>>,

    #[doc = "Title of the issue"]
    pub title: Avail<String>,

    pub updated_at: Avail<Timestamp>,

    #[doc = "URL for the issue"]
    pub url: Avail<String>,

    pub user_id: Avail<Option<UserId>>,
}
