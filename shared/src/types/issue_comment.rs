use derive_more::derive::{AsRef, Deref, From, Into};
use github_api::models::{AuthorAssociation, Reactions};
use jiff::Timestamp;
use macros::AvailMerge;
use serde::{Deserialize, Serialize};

use crate::avail::Avail;

use super::{github_app::GithubAppId, issue::IssueId, repository::RepositoryId, user::UserId};

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
pub struct IssueCommentId(i64);

#[derive(
    macros::TypesafeIdb, Clone, Default, Debug, PartialEq, Serialize, Deserialize, AvailMerge,
)]
pub struct IssueComment {
    pub author_association: Avail<AuthorAssociation>,
    pub body: Avail<String>,
    pub created_at: Avail<Timestamp>,
    pub html_url: Avail<String>,

    #[idb(id)]
    pub id: IssueCommentId,
    pub issue_url: Avail<String>,
    pub node_id: Avail<String>,
    pub performed_via_github_app_id: Avail<Option<GithubAppId>>,
    pub reactions: Avail<Reactions>,
    pub updated_at: Avail<Timestamp>,
    pub url: Avail<String>,
    pub user_id: Avail<Option<UserId>>,

    /// I don't support indexing on nested attrs yet, which I'd have to support if I want to index
    /// `Avail<IssueId>`, hence:
    #[idb(index)]
    pub issue_id: Option<IssueId>,

    /// I want to store this because the intial fetch of issue comments doesn't include the issue
    /// id, and so we have to reconstruct it from the issue_url, but doing that in bulk will be
    /// cheaper. And storing the repository_id makes doing _that_ easier.
    #[idb(index)]
    pub repository_id: RepositoryId,
}
