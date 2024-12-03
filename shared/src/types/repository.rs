use derive_more::derive::{AsRef, Deref, From, Into};
use github_api::models::{
    repository::{
        MergeCommitMessage, MergeCommitTitle, SquashMergeCommitMessage, SquashMergeCommitTitle,
        Visibility,
    },
    RepositoryPermissions,
};
use jiff::Timestamp;
use macros::AvailMerge;
use serde::{Deserialize, Serialize};

use crate::avail::Avail;

use super::{installation::InstallationId, license::LicenseId, user::UserId};

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
    Copy,
    Hash,
    Default,
    PartialOrd,
    Ord,
)]
pub struct RepositoryId(i64);

#[derive(
    macros::TypesafeIdb, Deserialize, Serialize, Clone, Debug, AvailMerge, Default, PartialEq, Eq,
)]
pub struct Repository {
    #[doc = "Whether to allow Auto-merge to be used on pull requests."]
    pub allow_auto_merge: Avail<bool>,
    #[doc = "Whether to allow forking this repo"]
    pub allow_forking: Avail<Option<bool>>,
    #[doc = "Whether to allow merge commits for pull requests."]
    pub allow_merge_commit: Avail<bool>,
    #[doc = "Whether to allow rebase merges for pull requests."]
    pub allow_rebase_merge: Avail<bool>,
    #[doc = "Whether to allow squash merges for pull requests."]
    pub allow_squash_merge: Avail<bool>,
    #[doc = "Whether or not a pull request head branch that is behind its base branch can always be updated even if it is not required to be up to date before merging."]
    pub allow_update_branch: Avail<bool>,
    #[doc = "Whether anonymous git access is enabled for this repository"]
    pub anonymous_access_enabled: Avail<Option<bool>>,
    pub archive_url: Avail<String>,
    #[doc = "Whether the repository is archived."]
    pub archived: Avail<bool>,
    pub assignees_url: Avail<String>,
    pub blobs_url: Avail<String>,
    pub branches_url: Avail<String>,
    pub clone_url: Avail<String>,
    pub collaborators_url: Avail<String>,
    pub comments_url: Avail<String>,
    pub commits_url: Avail<String>,
    pub compare_url: Avail<String>,
    pub contents_url: Avail<String>,
    pub contributors_url: Avail<String>,
    pub created_at: Avail<Timestamp>,
    #[doc = "The default branch of the repository."]
    pub default_branch: Avail<String>,
    #[doc = "Whether to delete head branches when pull requests are merged"]
    #[serde(default)]
    pub delete_branch_on_merge: Avail<bool>,
    pub deployments_url: Avail<String>,
    pub description: Avail<Option<String>>,
    #[doc = "Returns whether or not this repository disabled."]
    pub disabled: Avail<bool>,
    pub downloads_url: Avail<String>,
    pub events_url: Avail<String>,
    pub fork: Avail<bool>,
    pub forks: Avail<i64>,
    pub forks_count: Avail<i64>,
    pub forks_url: Avail<String>,
    pub full_name: Avail<String>,
    pub git_commits_url: Avail<String>,
    pub git_refs_url: Avail<String>,
    pub git_tags_url: Avail<String>,
    pub git_url: Avail<String>,
    #[doc = "Whether discussions are enabled."]
    #[serde(default)]
    pub has_discussions: Avail<bool>,
    #[doc = "Whether downloads are enabled."]
    pub has_downloads: Avail<bool>,
    #[doc = "Whether issues are enabled."]
    pub has_issues: Avail<bool>,
    pub has_pages: Avail<bool>,
    #[doc = "Whether projects are enabled."]
    pub has_projects: Avail<bool>,
    #[doc = "Whether the wiki is enabled."]
    pub has_wiki: Avail<bool>,
    pub homepage: Avail<Option<String>>,
    pub hooks_url: Avail<String>,
    pub html_url: Avail<String>,
    #[doc = "Unique identifier of the repository"]
    #[idb(id)]
    pub id: RepositoryId,
    #[doc = "Whether this repository acts as a template that can be used to generate new repositories."]
    #[serde(default)]
    pub is_template: Avail<bool>,
    pub issue_comment_url: Avail<String>,
    pub issue_events_url: Avail<String>,
    pub issues_url: Avail<String>,
    pub keys_url: Avail<String>,
    pub labels_url: Avail<String>,
    pub language: Avail<Option<String>>,
    pub languages_url: Avail<String>,
    pub license_id: Avail<Option<LicenseId>>,
    pub master_branch: Avail<Option<String>>,
    #[doc = "The default value for a merge commit message.\n\n- `PR_TITLE` - default to the pull request's title.\n- `PR_BODY` - default to the pull request's body.\n- `BLANK` - default to a blank commit message."]
    pub merge_commit_message: Avail<Option<MergeCommitMessage>>,
    #[doc = "The default value for a merge commit title.\n\n- `PR_TITLE` - default to the pull request's title.\n- `MERGE_MESSAGE` - default to the classic title for a merge message (e.g., Merge pull request #123 from branch-name)."]
    pub merge_commit_title: Avail<Option<MergeCommitTitle>>,
    pub merges_url: Avail<String>,
    pub milestones_url: Avail<String>,
    pub mirror_url: Avail<Option<String>>,

    #[idb(index)]
    pub name: String,

    pub node_id: Avail<String>,
    pub notifications_url: Avail<String>,
    pub open_issues: Avail<i64>,
    pub open_issues_count: Avail<i64>,
    pub organization: Avail<Option<String>>,
    pub owner_id: Avail<UserId>,
    pub permissions: Avail<Option<RepositoryPermissions>>,
    #[doc = "Whether the repository is private or public."]
    pub private: Avail<bool>,
    pub pulls_url: Avail<String>,
    pub pushed_at: Avail<Option<Timestamp>>,
    pub releases_url: Avail<String>,
    pub role_name: Avail<Option<String>>,
    #[doc = "The size of the repository, in kilobytes. Size is calculated hourly. When a repository is initially created, the size is 0."]
    pub size: Avail<i64>,
    #[doc = "The default value for a squash merge commit message:\n\n- `PR_BODY` - default to the pull request's body.\n- `COMMIT_MESSAGES` - default to the branch's commit messages.\n- `BLANK` - default to a blank commit message."]
    pub squash_merge_commit_message: Avail<Option<SquashMergeCommitMessage>>,
    #[doc = "The default value for a squash merge commit title:\n\n- `PR_TITLE` - default to the pull request's title.\n- `COMMIT_OR_PR_TITLE` - default to the commit's title (if only one commit) or the pull request's title (when more than one commit)."]
    pub squash_merge_commit_title: Avail<Option<SquashMergeCommitTitle>>,
    pub ssh_url: Avail<String>,
    pub stargazers_count: Avail<i64>,
    pub stargazers_url: Avail<String>,
    pub starred_at: Avail<Option<String>>,
    pub statuses_url: Avail<String>,
    pub subscribers_url: Avail<String>,
    pub subscription_url: Avail<String>,
    pub svn_url: Avail<String>,
    pub tags_url: Avail<String>,
    pub teams_url: Avail<String>,
    pub temp_clone_token: Avail<Option<String>>,
    pub topics: Avail<Vec<String>>,
    pub trees_url: Avail<String>,
    pub updated_at: Avail<Option<Timestamp>>,
    pub url: Avail<String>,
    #[doc = "Whether a squash merge commit can use the pull request title as default. **This property is closing down. Please use `squash_merge_commit_title` instead."]
    #[serde(default)]
    pub use_squash_pr_title_as_default: Avail<bool>,
    #[doc = "The repository visibility: public, private, or internal."]
    pub visibility: Avail<Visibility>,
    pub watchers: Avail<i64>,
    pub watchers_count: Avail<i64>,
    #[doc = "Whether to require contributors to sign off on web-based commits"]
    pub web_commit_signoff_required: Avail<bool>,

    /// This isn't present in API responses, but is useful for us.
    #[idb(index)]
    pub installation_id: InstallationId,
}
