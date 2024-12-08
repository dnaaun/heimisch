#![allow(irrefutable_let_patterns)]

use serde::{Deserialize, Serialize};

pub mod error {
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum BranchProtectionConfiguration {
    #[serde(rename = "disabled")]
    Disabled {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "enabled")]
    Enabled {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&BranchProtectionConfiguration> for BranchProtectionConfiguration {
    fn from(value: &BranchProtectionConfiguration) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum BranchProtectionRule {
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        rule: BranchProtectionRuleCreatedRule,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        rule: BranchProtectionRuleCreatedRule,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        changes: Option<BranchProtectionRuleEditedChanges>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        rule: BranchProtectionRuleCreatedRule,
        sender: Untyped,
    },
}
impl From<&BranchProtectionRule> for BranchProtectionRule {
    fn from(value: &BranchProtectionRule) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct BranchProtectionRuleCreatedRule {
    pub admin_enforced: bool,
    pub allow_deletions_enforcement_level:
        BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel,
    pub allow_force_pushes_enforcement_level:
        BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel,
    pub authorized_actor_names: Vec<String>,
    pub authorized_actors_only: bool,
    pub authorized_dismissal_actors_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_protected: Option<bool>,
    pub created_at: String,
    pub dismiss_stale_reviews_on_push: bool,
    pub id: i64,
    pub ignore_approvals_from_contributors: bool,
    pub linear_history_requirement_enforcement_level:
        BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_allows_fork_sync: Option<bool>,
    pub lock_branch_enforcement_level:
        BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel,
    pub merge_queue_enforcement_level:
        BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel,
    pub name: String,
    pub pull_request_reviews_enforcement_level:
        BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel,
    pub repository_id: i64,
    pub require_code_owner_review: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_last_push_approval: Option<bool>,
    pub required_approving_review_count: i64,
    pub required_conversation_resolution_level:
        BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel,
    pub required_deployments_enforcement_level:
        BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel,
    pub required_status_checks: Vec<String>,
    pub required_status_checks_enforcement_level:
        BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel,
    pub signature_requirement_enforcement_level:
        BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel,
    pub strict_required_status_checks_policy: bool,
    pub updated_at: String,
}
impl From<&BranchProtectionRuleCreatedRule> for BranchProtectionRuleCreatedRule {
    fn from(value: &BranchProtectionRuleCreatedRule) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel {
    #[serde(rename = "everyone")]
    Everyone,
    #[serde(rename = "non_admins")]
    NonAdmins,
    #[serde(rename = "off")]
    Off,
}
impl From<&BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel>
    for BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel
{
    fn from(value: &BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel) -> Self {
        *value
    }
}
impl ::std::fmt::Display for BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Everyone => write!(f, "everyone"),
            Self::NonAdmins => write!(f, "non_admins"),
            Self::Off => write!(f, "off"),
        }
    }
}
impl std::str::FromStr for BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "everyone" => Ok(Self::Everyone),
            "non_admins" => Ok(Self::NonAdmins),
            "off" => Ok(Self::Off),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct BranchProtectionRuleEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_enforced: Option<BranchProtectionRuleEditedChangesAdminEnforced>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized_actor_names:
        Option<RepositoryRulesetEditedChangesConditionsUpdatedChangesInclude>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized_actors_only: Option<BranchProtectionRuleEditedChangesAdminEnforced>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorized_dismissal_actors_only: Option<BranchProtectionRuleEditedChangesAdminEnforced>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linear_history_requirement_enforcement_level:
        Option<BranchProtectionRuleEditedChangesLinearHistoryRequirementEnforcementLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_allows_fork_sync: Option<BranchProtectionRuleEditedChangesAdminEnforced>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_branch_enforcement_level:
        Option<BranchProtectionRuleEditedChangesLinearHistoryRequirementEnforcementLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request_reviews_enforcement_level:
        Option<BranchProtectionRuleEditedChangesLinearHistoryRequirementEnforcementLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_last_push_approval: Option<BranchProtectionRuleEditedChangesAdminEnforced>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_status_checks:
        Option<RepositoryRulesetEditedChangesConditionsUpdatedChangesInclude>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_status_checks_enforcement_level:
        Option<BranchProtectionRuleEditedChangesLinearHistoryRequirementEnforcementLevel>,
}
impl From<&BranchProtectionRuleEditedChanges> for BranchProtectionRuleEditedChanges {
    fn from(value: &BranchProtectionRuleEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct BranchProtectionRuleEditedChangesAdminEnforced {
    pub from: Option<bool>,
}
impl From<&BranchProtectionRuleEditedChangesAdminEnforced>
    for BranchProtectionRuleEditedChangesAdminEnforced
{
    fn from(value: &BranchProtectionRuleEditedChangesAdminEnforced) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct BranchProtectionRuleEditedChangesLinearHistoryRequirementEnforcementLevel {
    pub from: BranchProtectionRuleCreatedRuleAllowDeletionsEnforcementLevel,
}
impl From<&BranchProtectionRuleEditedChangesLinearHistoryRequirementEnforcementLevel>
    for BranchProtectionRuleEditedChangesLinearHistoryRequirementEnforcementLevel
{
    fn from(
        value: &BranchProtectionRuleEditedChangesLinearHistoryRequirementEnforcementLevel,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CheckRun {
    Completed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        action: Option<String>,
        check_run: CheckRunCompletedCheckRun,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        action: Option<String>,
        check_run: CheckRunCompletedCheckRun,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    RequestedAction {
        action: String,
        check_run: CheckRunCompletedCheckRun,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        requested_action: Option<CheckRunRequestedActionRequestedAction>,
        sender: Untyped,
    },
    Rerequested {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        action: Option<String>,
        check_run: CheckRunCompletedCheckRun,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&CheckRun> for CheckRun {
    fn from(value: &CheckRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRun {
    pub app: (),
    pub check_suite: CheckRunCompletedCheckRunCheckSuite,
    pub completed_at: Option<String>,
    pub conclusion: Option<CheckRunCompletedCheckRunConclusion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<CheckRunCompletedCheckRunDeployment>,
    pub details_url: String,
    pub external_id: String,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub output: CheckRunCompletedCheckRunOutput,
    pub pull_requests: Vec<CheckRunCompletedCheckRunCheckSuitePullRequests>,
    pub started_at: String,
    pub status: CheckRunCompletedCheckRunStatus,
    pub url: String,
}
impl From<&CheckRunCompletedCheckRun> for CheckRunCompletedCheckRun {
    fn from(value: &CheckRunCompletedCheckRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRunCheckSuite {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app: Option<IssueCommentDeletedCommentPerformedViaGithubApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<WorkflowRunCompletedWorkflowRunConclusion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pull_requests: Vec<CheckRunCompletedCheckRunCheckSuitePullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<CheckRunCompletedCheckRunCheckSuiteRepository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DeploymentStatusCreatedCheckRunStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&CheckRunCompletedCheckRunCheckSuite> for CheckRunCompletedCheckRunCheckSuite {
    fn from(value: &CheckRunCompletedCheckRunCheckSuite) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRunCheckSuitePullRequests {
    pub base: CheckRunCompletedCheckRunCheckSuitePullRequestsHead,
    pub head: CheckRunCompletedCheckRunCheckSuitePullRequestsHead,
    pub id: i64,
    pub number: i64,
    pub url: String,
}
impl From<&CheckRunCompletedCheckRunCheckSuitePullRequests>
    for CheckRunCompletedCheckRunCheckSuitePullRequests
{
    fn from(value: &CheckRunCompletedCheckRunCheckSuitePullRequests) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRunCheckSuitePullRequestsHead {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: CheckRunCompletedCheckRunCheckSuitePullRequestsHeadRepo,
    pub sha: String,
}
impl From<&CheckRunCompletedCheckRunCheckSuitePullRequestsHead>
    for CheckRunCompletedCheckRunCheckSuitePullRequestsHead
{
    fn from(value: &CheckRunCompletedCheckRunCheckSuitePullRequestsHead) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRunCheckSuitePullRequestsHeadRepo {
    pub id: i64,
    pub name: String,
    pub url: String,
}
impl From<&CheckRunCompletedCheckRunCheckSuitePullRequestsHeadRepo>
    for CheckRunCompletedCheckRunCheckSuitePullRequestsHeadRepo
{
    fn from(value: &CheckRunCompletedCheckRunCheckSuitePullRequestsHeadRepo) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRunCheckSuiteRepository {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    pub archive_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clone_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_of_conduct: Option<CheckRunCompletedCheckRunCheckSuiteRepositoryCodeOfConduct>,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    pub deployments_url: String,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks_count: Option<i64>,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_discussions: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_downloads: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub languages_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<CheckRunCompletedCheckRunCheckSuiteRepositoryLicense>,
    pub merges_url: String,
    pub milestones_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_url: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i64>,
    pub node_id: String,
    pub notifications_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_issues: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_issues_count: Option<i64>,
    pub owner: DiscussionTransferredChangesNewRepositoryOrganization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryPermissions>,
    pub private: bool,
    pub pulls_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pushed_at: Option<String>,
    pub releases_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_and_analysis:
        Option<CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers_count: Option<i64>,
    pub stargazers_url: String,
    pub statuses_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i64>,
    pub subscribers_url: String,
    pub subscription_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svn_url: Option<String>,
    pub tags_url: String,
    pub teams_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temp_clone_token: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    pub trees_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_commit_signoff_required: Option<bool>,
}
impl From<&CheckRunCompletedCheckRunCheckSuiteRepository>
    for CheckRunCompletedCheckRunCheckSuiteRepository
{
    fn from(value: &CheckRunCompletedCheckRunCheckSuiteRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRunCheckSuiteRepositoryCodeOfConduct {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub html_url: Option<String>,
    pub key: String,
    pub name: String,
    pub url: String,
}
impl From<&CheckRunCompletedCheckRunCheckSuiteRepositoryCodeOfConduct>
    for CheckRunCompletedCheckRunCheckSuiteRepositoryCodeOfConduct
{
    fn from(value: &CheckRunCompletedCheckRunCheckSuiteRepositoryCodeOfConduct) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRunCheckSuiteRepositoryLicense {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spdx_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&CheckRunCompletedCheckRunCheckSuiteRepositoryLicense>
    for CheckRunCompletedCheckRunCheckSuiteRepositoryLicense
{
    fn from(value: &CheckRunCompletedCheckRunCheckSuiteRepositoryLicense) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysis {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advanced_security:
        Option<CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependabot_security_updates:
        Option<CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning:
        Option<CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning_ai_detection:
        Option<CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning_non_provider_patterns:
        Option<CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning_push_protection:
        Option<CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurity>,
}
impl From<&CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysis>
    for CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysis
{
    fn from(value: &CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysis) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<
        CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurityStatus,
    >,
}
impl From<&CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurity>
    for CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurity
{
    fn from(
        value: &CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurity,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurityStatus {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "enabled")]
    Enabled,
}
impl From<&CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurityStatus>
    for CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurityStatus
{
    fn from(
        value : & CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurityStatus,
    ) -> Self {
        *value
    }
}
impl ::std::fmt::Display
    for CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurityStatus
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Disabled => write!(f, "disabled"),
            Self::Enabled => write!(f, "enabled"),
        }
    }
}
impl std::str::FromStr
    for CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurityStatus
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "disabled" => Ok(Self::Disabled),
            "enabled" => Ok(Self::Enabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurityStatus
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurityStatus
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysisAdvancedSecurityStatus
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CheckRunCompletedCheckRunConclusion {
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "startup_failure")]
    StartupFailure,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "waiting")]
    Waiting,
}
impl From<&CheckRunCompletedCheckRunConclusion> for CheckRunCompletedCheckRunConclusion {
    fn from(value: &CheckRunCompletedCheckRunConclusion) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CheckRunCompletedCheckRunConclusion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ActionRequired => write!(f, "action_required"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Failure => write!(f, "failure"),
            Self::Neutral => write!(f, "neutral"),
            Self::Pending => write!(f, "pending"),
            Self::Skipped => write!(f, "skipped"),
            Self::Stale => write!(f, "stale"),
            Self::StartupFailure => write!(f, "startup_failure"),
            Self::Success => write!(f, "success"),
            Self::TimedOut => write!(f, "timed_out"),
            Self::Waiting => write!(f, "waiting"),
        }
    }
}
impl std::str::FromStr for CheckRunCompletedCheckRunConclusion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "action_required" => Ok(Self::ActionRequired),
            "cancelled" => Ok(Self::Cancelled),
            "failure" => Ok(Self::Failure),
            "neutral" => Ok(Self::Neutral),
            "pending" => Ok(Self::Pending),
            "skipped" => Ok(Self::Skipped),
            "stale" => Ok(Self::Stale),
            "startup_failure" => Ok(Self::StartupFailure),
            "success" => Ok(Self::Success),
            "timed_out" => Ok(Self::TimedOut),
            "waiting" => Ok(Self::Waiting),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CheckRunCompletedCheckRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CheckRunCompletedCheckRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CheckRunCompletedCheckRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRunDeployment {
    pub created_at: String,
    pub description: Option<String>,
    pub environment: String,
    pub id: i64,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_environment: Option<String>,
    #[serde(default)]
    pub performed_via_github_app: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_environment: Option<bool>,
    pub repository_url: String,
    pub statuses_url: String,
    pub task: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_environment: Option<bool>,
    pub updated_at: String,
    pub url: String,
}
impl From<&CheckRunCompletedCheckRunDeployment> for CheckRunCompletedCheckRunDeployment {
    fn from(value: &CheckRunCompletedCheckRunDeployment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunCompletedCheckRunOutput {
    pub annotations_count: i64,
    pub annotations_url: String,
    pub summary: Option<String>,
    pub text: Option<String>,
    pub title: Option<String>,
}
impl From<&CheckRunCompletedCheckRunOutput> for CheckRunCompletedCheckRunOutput {
    fn from(value: &CheckRunCompletedCheckRunOutput) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CheckRunCompletedCheckRunStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "queued")]
    Queued,
}
impl From<&CheckRunCompletedCheckRunStatus> for CheckRunCompletedCheckRunStatus {
    fn from(value: &CheckRunCompletedCheckRunStatus) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CheckRunCompletedCheckRunStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => write!(f, "completed"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Pending => write!(f, "pending"),
            Self::Queued => write!(f, "queued"),
        }
    }
}
impl std::str::FromStr for CheckRunCompletedCheckRunStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "in_progress" => Ok(Self::InProgress),
            "pending" => Ok(Self::Pending),
            "queued" => Ok(Self::Queued),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CheckRunCompletedCheckRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CheckRunCompletedCheckRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CheckRunCompletedCheckRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckRunRequestedActionRequestedAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
impl From<&CheckRunRequestedActionRequestedAction> for CheckRunRequestedActionRequestedAction {
    fn from(value: &CheckRunRequestedActionRequestedAction) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum CheckSuite {
    #[serde(rename = "completed")]
    Completed {
        check_suite: CheckSuiteCompletedCheckSuite,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "requested")]
    Requested {
        check_suite: CheckSuiteRequestedCheckSuite,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "rerequested")]
    Rerequested {
        check_suite: CheckSuiteRerequestedCheckSuite,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&CheckSuite> for CheckSuite {
    fn from(value: &CheckSuite) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckSuiteCompletedCheckSuite {
    pub after: Option<String>,
    pub app: CheckSuiteCompletedCheckSuiteApp,
    pub before: Option<String>,
    pub check_runs_url: String,
    pub conclusion: Option<WorkflowRunCompletedWorkflowRunConclusion>,
    pub created_at: String,
    pub head_branch: Option<String>,
    pub head_commit: WorkflowRunCompletedWorkflowRunHeadCommit,
    pub head_sha: String,
    pub id: i64,
    pub latest_check_runs_count: i64,
    pub node_id: String,
    pub pull_requests: Vec<DeploymentStatusCreatedWorkflowRunPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerequestable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs_rerequestable: Option<bool>,
    pub status: Option<WorkflowRunInProgressWorkflowRunStatus>,
    pub updated_at: String,
    pub url: String,
}
impl From<&CheckSuiteCompletedCheckSuite> for CheckSuiteCompletedCheckSuite {
    fn from(value: &CheckSuiteCompletedCheckSuite) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckSuiteCompletedCheckSuiteApp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    pub created_at: Option<String>,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    pub external_url: Option<String>,
    pub html_url: String,
    pub id: Option<i64>,
    pub name: String,
    pub node_id: String,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<IssuesReopenedIssuePerformedViaGithubAppPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub updated_at: Option<String>,
}
impl From<&CheckSuiteCompletedCheckSuiteApp> for CheckSuiteCompletedCheckSuiteApp {
    fn from(value: &CheckSuiteCompletedCheckSuiteApp) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckSuiteRequestedCheckSuite {
    pub after: Option<String>,
    pub app: CheckSuiteCompletedCheckSuiteApp,
    pub before: Option<String>,
    pub check_runs_url: String,
    pub conclusion: Option<DeploymentStatusCreatedCheckRunConclusion>,
    pub created_at: String,
    pub head_branch: Option<String>,
    pub head_commit: WorkflowRunCompletedWorkflowRunHeadCommit,
    pub head_sha: String,
    pub id: i64,
    pub latest_check_runs_count: i64,
    pub node_id: String,
    pub pull_requests: Vec<DeploymentStatusCreatedWorkflowRunPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerequestable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs_rerequestable: Option<bool>,
    pub status: Option<CheckSuiteRequestedCheckSuiteStatus>,
    pub updated_at: String,
    pub url: String,
}
impl From<&CheckSuiteRequestedCheckSuite> for CheckSuiteRequestedCheckSuite {
    fn from(value: &CheckSuiteRequestedCheckSuite) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CheckSuiteRequestedCheckSuiteStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "requested")]
    Requested,
}
impl From<&CheckSuiteRequestedCheckSuiteStatus> for CheckSuiteRequestedCheckSuiteStatus {
    fn from(value: &CheckSuiteRequestedCheckSuiteStatus) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CheckSuiteRequestedCheckSuiteStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => write!(f, "completed"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Queued => write!(f, "queued"),
            Self::Requested => write!(f, "requested"),
        }
    }
}
impl std::str::FromStr for CheckSuiteRequestedCheckSuiteStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "in_progress" => Ok(Self::InProgress),
            "queued" => Ok(Self::Queued),
            "requested" => Ok(Self::Requested),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CheckSuiteRequestedCheckSuiteStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CheckSuiteRequestedCheckSuiteStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CheckSuiteRequestedCheckSuiteStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CheckSuiteRerequestedCheckSuite {
    pub after: Option<String>,
    pub app: CheckSuiteCompletedCheckSuiteApp,
    pub before: Option<String>,
    pub check_runs_url: String,
    pub conclusion: Option<DeploymentCreatedWorkflowRunConclusion>,
    pub created_at: String,
    pub head_branch: Option<String>,
    pub head_commit: WorkflowRunCompletedWorkflowRunHeadCommit,
    pub head_sha: String,
    pub id: i64,
    pub latest_check_runs_count: i64,
    pub node_id: String,
    pub pull_requests: Vec<DeploymentStatusCreatedWorkflowRunPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerequestable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runs_rerequestable: Option<bool>,
    pub status: Option<CheckSuiteRequestedCheckSuiteStatus>,
    pub updated_at: String,
    pub url: String,
}
impl From<&CheckSuiteRerequestedCheckSuite> for CheckSuiteRerequestedCheckSuite {
    fn from(value: &CheckSuiteRerequestedCheckSuite) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum CodeScanningAlert {
    #[serde(rename = "appeared_in_branch")]
    AppearedInBranch {
        alert: CodeScanningAlertAppearedInBranchAlert,
        commit_oid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(rename = "ref")]
        ref_: String,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "closed_by_user")]
    ClosedByUser {
        alert: CodeScanningAlertClosedByUserAlert,
        commit_oid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(rename = "ref")]
        ref_: String,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "created")]
    Created {
        alert: CodeScanningAlertCreatedAlert,
        commit_oid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(rename = "ref")]
        ref_: String,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "fixed")]
    Fixed {
        alert: CodeScanningAlertFixedAlert,
        commit_oid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(rename = "ref")]
        ref_: String,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "reopened")]
    Reopened {
        alert: Option<CodeScanningAlertReopenedAlert>,
        commit_oid: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(rename = "ref")]
        ref_: Option<String>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "reopened_by_user")]
    ReopenedByUser {
        alert: CodeScanningAlertReopenedByUserAlert,
        commit_oid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(rename = "ref")]
        ref_: String,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&CodeScanningAlert> for CodeScanningAlert {
    fn from(value: &CodeScanningAlert) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertAppearedInBranchAlert {
    pub created_at: String,
    pub dismissed_at: Option<String>,
    pub dismissed_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub dismissed_reason: Option<CodeScanningAlertAppearedInBranchAlertDismissedReason>,
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub most_recent_instance: Option<CodeScanningAlertAppearedInBranchAlertMostRecentInstance>,
    pub number: i64,
    pub rule: CodeScanningAlertAppearedInBranchAlertRule,
    pub state: CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState,
    pub tool: CodeScanningAlertAppearedInBranchAlertTool,
    pub url: String,
}
impl From<&CodeScanningAlertAppearedInBranchAlert> for CodeScanningAlertAppearedInBranchAlert {
    fn from(value: &CodeScanningAlertAppearedInBranchAlert) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CodeScanningAlertAppearedInBranchAlertDismissedReason {
    #[serde(rename = "false positive")]
    FalsePositive,
    #[serde(rename = "used in tests")]
    UsedInTests,
    #[serde(rename = "won't fix")]
    WontFix,
}
impl From<&CodeScanningAlertAppearedInBranchAlertDismissedReason>
    for CodeScanningAlertAppearedInBranchAlertDismissedReason
{
    fn from(value: &CodeScanningAlertAppearedInBranchAlertDismissedReason) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CodeScanningAlertAppearedInBranchAlertDismissedReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FalsePositive => write!(f, "false positive"),
            Self::UsedInTests => write!(f, "used in tests"),
            Self::WontFix => write!(f, "won't fix"),
        }
    }
}
impl std::str::FromStr for CodeScanningAlertAppearedInBranchAlertDismissedReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "false positive" => Ok(Self::FalsePositive),
            "used in tests" => Ok(Self::UsedInTests),
            "won't fix" => Ok(Self::WontFix),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CodeScanningAlertAppearedInBranchAlertDismissedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CodeScanningAlertAppearedInBranchAlertDismissedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CodeScanningAlertAppearedInBranchAlertDismissedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertAppearedInBranchAlertMostRecentInstance {
    pub analysis_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifications: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    pub environment: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<CodeScanningAlertAppearedInBranchAlertMostRecentInstanceLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<CodeScanningAlertAppearedInBranchAlertMostRecentInstanceMessage>,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub state: CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState,
}
impl From<&CodeScanningAlertAppearedInBranchAlertMostRecentInstance>
    for CodeScanningAlertAppearedInBranchAlertMostRecentInstance
{
    fn from(value: &CodeScanningAlertAppearedInBranchAlertMostRecentInstance) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertAppearedInBranchAlertMostRecentInstanceLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
}
impl From<&CodeScanningAlertAppearedInBranchAlertMostRecentInstanceLocation>
    for CodeScanningAlertAppearedInBranchAlertMostRecentInstanceLocation
{
    fn from(value: &CodeScanningAlertAppearedInBranchAlertMostRecentInstanceLocation) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertAppearedInBranchAlertMostRecentInstanceMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
impl From<&CodeScanningAlertAppearedInBranchAlertMostRecentInstanceMessage>
    for CodeScanningAlertAppearedInBranchAlertMostRecentInstanceMessage
{
    fn from(value: &CodeScanningAlertAppearedInBranchAlertMostRecentInstanceMessage) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState {
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "open")]
    Open,
}
impl From<&CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState>
    for CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState
{
    fn from(value: &CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dismissed => write!(f, "dismissed"),
            Self::Fixed => write!(f, "fixed"),
            Self::Open => write!(f, "open"),
        }
    }
}
impl std::str::FromStr for CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "dismissed" => Ok(Self::Dismissed),
            "fixed" => Ok(Self::Fixed),
            "open" => Ok(Self::Open),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertAppearedInBranchAlertRule {
    pub description: String,
    pub id: String,
    pub severity: Option<CodeScanningAlertAppearedInBranchAlertRuleSeverity>,
}
impl From<&CodeScanningAlertAppearedInBranchAlertRule>
    for CodeScanningAlertAppearedInBranchAlertRule
{
    fn from(value: &CodeScanningAlertAppearedInBranchAlertRule) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CodeScanningAlertAppearedInBranchAlertRuleSeverity {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "warning")]
    Warning,
}
impl From<&CodeScanningAlertAppearedInBranchAlertRuleSeverity>
    for CodeScanningAlertAppearedInBranchAlertRuleSeverity
{
    fn from(value: &CodeScanningAlertAppearedInBranchAlertRuleSeverity) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CodeScanningAlertAppearedInBranchAlertRuleSeverity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Error => write!(f, "error"),
            Self::None => write!(f, "none"),
            Self::Note => write!(f, "note"),
            Self::Warning => write!(f, "warning"),
        }
    }
}
impl std::str::FromStr for CodeScanningAlertAppearedInBranchAlertRuleSeverity {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "error" => Ok(Self::Error),
            "none" => Ok(Self::None),
            "note" => Ok(Self::Note),
            "warning" => Ok(Self::Warning),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CodeScanningAlertAppearedInBranchAlertRuleSeverity {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CodeScanningAlertAppearedInBranchAlertRuleSeverity {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CodeScanningAlertAppearedInBranchAlertRuleSeverity {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertAppearedInBranchAlertTool {
    pub name: String,
    pub version: Option<String>,
}
impl From<&CodeScanningAlertAppearedInBranchAlertTool>
    for CodeScanningAlertAppearedInBranchAlertTool
{
    fn from(value: &CodeScanningAlertAppearedInBranchAlertTool) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertClosedByUserAlert {
    pub created_at: String,
    pub dismissed_at: String,
    pub dismissed_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub dismissed_reason: Option<CodeScanningAlertAppearedInBranchAlertDismissedReason>,
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub most_recent_instance: Option<CodeScanningAlertAppearedInBranchAlertMostRecentInstance>,
    pub number: i64,
    pub rule: CodeScanningAlertClosedByUserAlertRule,
    pub state: CodeScanningAlertClosedByUserAlertState,
    pub tool: CodeScanningAlertClosedByUserAlertTool,
    pub url: String,
}
impl From<&CodeScanningAlertClosedByUserAlert> for CodeScanningAlertClosedByUserAlert {
    fn from(value: &CodeScanningAlertClosedByUserAlert) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertClosedByUserAlertRule {
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_uri: Option<String>,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub severity: Option<CodeScanningAlertAppearedInBranchAlertRuleSeverity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}
impl From<&CodeScanningAlertClosedByUserAlertRule> for CodeScanningAlertClosedByUserAlertRule {
    fn from(value: &CodeScanningAlertClosedByUserAlertRule) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CodeScanningAlertClosedByUserAlertState {
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "fixed")]
    Fixed,
}
impl From<&CodeScanningAlertClosedByUserAlertState> for CodeScanningAlertClosedByUserAlertState {
    fn from(value: &CodeScanningAlertClosedByUserAlertState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CodeScanningAlertClosedByUserAlertState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dismissed => write!(f, "dismissed"),
            Self::Fixed => write!(f, "fixed"),
        }
    }
}
impl std::str::FromStr for CodeScanningAlertClosedByUserAlertState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "dismissed" => Ok(Self::Dismissed),
            "fixed" => Ok(Self::Fixed),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CodeScanningAlertClosedByUserAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CodeScanningAlertClosedByUserAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CodeScanningAlertClosedByUserAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertClosedByUserAlertTool {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    pub name: String,
    pub version: Option<String>,
}
impl From<&CodeScanningAlertClosedByUserAlertTool> for CodeScanningAlertClosedByUserAlertTool {
    fn from(value: &CodeScanningAlertClosedByUserAlertTool) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertCreatedAlert {
    pub created_at: Option<String>,
    pub dismissed_at: (),
    pub dismissed_by: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissed_comment: Option<String>,
    pub dismissed_reason: (),
    #[serde(default)]
    pub fixed_at: (),
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub most_recent_instance: Option<CodeScanningAlertAppearedInBranchAlertMostRecentInstance>,
    pub number: i64,
    pub rule: CodeScanningAlertClosedByUserAlertRule,
    pub state: Option<CodeScanningAlertCreatedAlertState>,
    pub tool: Option<CodeScanningAlertClosedByUserAlertTool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub url: String,
}
impl From<&CodeScanningAlertCreatedAlert> for CodeScanningAlertCreatedAlert {
    fn from(value: &CodeScanningAlertCreatedAlert) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CodeScanningAlertCreatedAlertState {
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "open")]
    Open,
}
impl From<&CodeScanningAlertCreatedAlertState> for CodeScanningAlertCreatedAlertState {
    fn from(value: &CodeScanningAlertCreatedAlertState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CodeScanningAlertCreatedAlertState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dismissed => write!(f, "dismissed"),
            Self::Open => write!(f, "open"),
        }
    }
}
impl std::str::FromStr for CodeScanningAlertCreatedAlertState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "dismissed" => Ok(Self::Dismissed),
            "open" => Ok(Self::Open),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CodeScanningAlertCreatedAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CodeScanningAlertCreatedAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CodeScanningAlertCreatedAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertFixedAlert {
    pub created_at: String,
    pub dismissed_at: Option<String>,
    pub dismissed_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub dismissed_reason: Option<CodeScanningAlertAppearedInBranchAlertDismissedReason>,
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub most_recent_instance: Option<CodeScanningAlertAppearedInBranchAlertMostRecentInstance>,
    pub number: i64,
    pub rule: CodeScanningAlertClosedByUserAlertRule,
    pub state: String,
    pub tool: CodeScanningAlertClosedByUserAlertTool,
    pub url: String,
}
impl From<&CodeScanningAlertFixedAlert> for CodeScanningAlertFixedAlert {
    fn from(value: &CodeScanningAlertFixedAlert) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertReopenedAlert {
    pub created_at: String,
    pub dismissed_at: Option<String>,
    pub dismissed_by: Option<Untyped>,
    pub dismissed_reason: Option<String>,
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub most_recent_instance: Option<CodeScanningAlertAppearedInBranchAlertMostRecentInstance>,
    pub number: i64,
    pub rule: CodeScanningAlertClosedByUserAlertRule,
    pub state: CodeScanningAlertAppearedInBranchAlertMostRecentInstanceState,
    pub tool: CodeScanningAlertClosedByUserAlertTool,
    pub url: String,
}
impl From<&CodeScanningAlertReopenedAlert> for CodeScanningAlertReopenedAlert {
    fn from(value: &CodeScanningAlertReopenedAlert) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CodeScanningAlertReopenedByUserAlert {
    pub created_at: String,
    pub dismissed_at: (),
    pub dismissed_by: (),
    pub dismissed_reason: (),
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub most_recent_instance: Option<CodeScanningAlertAppearedInBranchAlertMostRecentInstance>,
    pub number: i64,
    pub rule: CodeScanningAlertAppearedInBranchAlertRule,
    pub state: CodeScanningAlertReopenedByUserAlertState,
    pub tool: CodeScanningAlertAppearedInBranchAlertTool,
    pub url: String,
}
impl From<&CodeScanningAlertReopenedByUserAlert> for CodeScanningAlertReopenedByUserAlert {
    fn from(value: &CodeScanningAlertReopenedByUserAlert) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CodeScanningAlertReopenedByUserAlertState {
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "open")]
    Open,
}
impl From<&CodeScanningAlertReopenedByUserAlertState>
    for CodeScanningAlertReopenedByUserAlertState
{
    fn from(value: &CodeScanningAlertReopenedByUserAlertState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CodeScanningAlertReopenedByUserAlertState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Fixed => write!(f, "fixed"),
            Self::Open => write!(f, "open"),
        }
    }
}
impl std::str::FromStr for CodeScanningAlertReopenedByUserAlertState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "fixed" => Ok(Self::Fixed),
            "open" => Ok(Self::Open),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CodeScanningAlertReopenedByUserAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CodeScanningAlertReopenedByUserAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CodeScanningAlertReopenedByUserAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum CommitComment {
    #[serde(rename = "created")]
    Created {
        comment: CommitCommentCreatedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&CommitComment> for CommitComment {
    fn from(value: &CommitComment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CommitCommentCreatedComment {
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: String,
    pub commit_id: String,
    pub created_at: String,
    pub html_url: String,
    pub id: i64,
    pub line: Option<i64>,
    pub node_id: String,
    pub path: Option<String>,
    pub position: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<DiscussionAnsweredAnswerReactions>,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&CommitCommentCreatedComment> for CommitCommentCreatedComment {
    fn from(value: &CommitCommentCreatedComment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Create {
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    pub master_branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub pusher_type: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub ref_type: DeleteDefaultRefType,
    pub repository: Untyped,
    pub sender: Untyped,
}
impl From<&Create> for Create {
    fn from(value: &Create) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum CustomProperty {
    #[serde(rename = "created")]
    Created {
        definition: CustomPropertyCreatedDefinition,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "deleted")]
    Deleted {
        definition: CustomPropertyDeletedDefinition,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "updated")]
    Updated {
        definition: CustomPropertyCreatedDefinition,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
}
impl From<&CustomProperty> for CustomProperty {
    fn from(value: &CustomProperty) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CustomPropertyCreatedDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<CustomPropertyCreatedDefinitionDefaultValue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub property_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub value_type: CustomPropertyCreatedDefinitionValueType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values_editable_by: Option<CustomPropertyCreatedDefinitionValuesEditableBy>,
}
impl From<&CustomPropertyCreatedDefinition> for CustomPropertyCreatedDefinition {
    fn from(value: &CustomPropertyCreatedDefinition) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CustomPropertyCreatedDefinitionDefaultValue {
    Variant0(String),
    Variant1(Vec<::serde_json::Value>),
}
impl From<&CustomPropertyCreatedDefinitionDefaultValue>
    for CustomPropertyCreatedDefinitionDefaultValue
{
    fn from(value: &CustomPropertyCreatedDefinitionDefaultValue) -> Self {
        value.clone()
    }
}
impl From<Vec<::serde_json::Value>> for CustomPropertyCreatedDefinitionDefaultValue {
    fn from(value: Vec<::serde_json::Value>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CustomPropertyCreatedDefinitionValueType {
    #[serde(rename = "multi_select")]
    MultiSelect,
    #[serde(rename = "single_select")]
    SingleSelect,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "true_false")]
    TrueFalse,
}
impl From<&CustomPropertyCreatedDefinitionValueType> for CustomPropertyCreatedDefinitionValueType {
    fn from(value: &CustomPropertyCreatedDefinitionValueType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CustomPropertyCreatedDefinitionValueType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MultiSelect => write!(f, "multi_select"),
            Self::SingleSelect => write!(f, "single_select"),
            Self::String => write!(f, "string"),
            Self::TrueFalse => write!(f, "true_false"),
        }
    }
}
impl std::str::FromStr for CustomPropertyCreatedDefinitionValueType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "multi_select" => Ok(Self::MultiSelect),
            "single_select" => Ok(Self::SingleSelect),
            "string" => Ok(Self::String),
            "true_false" => Ok(Self::TrueFalse),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CustomPropertyCreatedDefinitionValueType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CustomPropertyCreatedDefinitionValueType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CustomPropertyCreatedDefinitionValueType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CustomPropertyCreatedDefinitionValuesEditableBy {
    #[serde(rename = "org_actors")]
    OrgActors,
    #[serde(rename = "org_and_repo_actors")]
    OrgAndRepoActors,
}
impl From<&CustomPropertyCreatedDefinitionValuesEditableBy>
    for CustomPropertyCreatedDefinitionValuesEditableBy
{
    fn from(value: &CustomPropertyCreatedDefinitionValuesEditableBy) -> Self {
        *value
    }
}
impl ::std::fmt::Display for CustomPropertyCreatedDefinitionValuesEditableBy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OrgActors => write!(f, "org_actors"),
            Self::OrgAndRepoActors => write!(f, "org_and_repo_actors"),
        }
    }
}
impl std::str::FromStr for CustomPropertyCreatedDefinitionValuesEditableBy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "org_actors" => Ok(Self::OrgActors),
            "org_and_repo_actors" => Ok(Self::OrgAndRepoActors),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for CustomPropertyCreatedDefinitionValuesEditableBy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CustomPropertyCreatedDefinitionValuesEditableBy {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CustomPropertyCreatedDefinitionValuesEditableBy {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CustomPropertyDeletedDefinition {
    pub property_name: String,
}
impl From<&CustomPropertyDeletedDefinition> for CustomPropertyDeletedDefinition {
    fn from(value: &CustomPropertyDeletedDefinition) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum CustomPropertyValues {
    #[serde(rename = "updated")]
    Updated {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        new_property_values: Vec<CustomPropertyValuesUpdatedNewPropertyValues>,
        old_property_values: Vec<CustomPropertyValuesUpdatedNewPropertyValues>,
        organization: Untyped,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
}
impl From<&CustomPropertyValues> for CustomPropertyValues {
    fn from(value: &CustomPropertyValues) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CustomPropertyValuesUpdatedNewPropertyValues {
    pub property_name: String,
    pub value: Option<CustomPropertyValuesUpdatedNewPropertyValuesValue>,
}
impl From<&CustomPropertyValuesUpdatedNewPropertyValues>
    for CustomPropertyValuesUpdatedNewPropertyValues
{
    fn from(value: &CustomPropertyValuesUpdatedNewPropertyValues) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum CustomPropertyValuesUpdatedNewPropertyValuesValue {
    Variant0(String),
    Variant1(Vec<::serde_json::Value>),
}
impl From<&CustomPropertyValuesUpdatedNewPropertyValuesValue>
    for CustomPropertyValuesUpdatedNewPropertyValuesValue
{
    fn from(value: &CustomPropertyValuesUpdatedNewPropertyValuesValue) -> Self {
        value.clone()
    }
}
impl From<Vec<::serde_json::Value>> for CustomPropertyValuesUpdatedNewPropertyValuesValue {
    fn from(value: Vec<::serde_json::Value>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Delete {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub pusher_type: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub ref_type: DeleteDefaultRefType,
    pub repository: Untyped,
    pub sender: Untyped,
}
impl From<&Delete> for Delete {
    fn from(value: &Delete) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeleteDefaultRefType {
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "tag")]
    Tag,
}
impl From<&DeleteDefaultRefType> for DeleteDefaultRefType {
    fn from(value: &DeleteDefaultRefType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeleteDefaultRefType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Branch => write!(f, "branch"),
            Self::Tag => write!(f, "tag"),
        }
    }
}
impl std::str::FromStr for DeleteDefaultRefType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "branch" => Ok(Self::Branch),
            "tag" => Ok(Self::Tag),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DeleteDefaultRefType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DeleteDefaultRefType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DeleteDefaultRefType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum DependabotAlert {
    #[serde(rename = "auto_dismissed")]
    AutoDismissed {
        alert: DependabotAlertAutoDismissedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "auto_reopened")]
    AutoReopened {
        alert: DependabotAlertAutoDismissedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "created")]
    Created {
        alert: DependabotAlertAutoDismissedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "dismissed")]
    Dismissed {
        alert: DependabotAlertAutoDismissedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "fixed")]
    Fixed {
        alert: DependabotAlertAutoDismissedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "reintroduced")]
    Reintroduced {
        alert: DependabotAlertAutoDismissedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "reopened")]
    Reopened {
        alert: DependabotAlertAutoDismissedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&DependabotAlert> for DependabotAlert {
    fn from(value: &DependabotAlert) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DependabotAlertAutoDismissedAlert {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_dismissed_at: Option<String>,
    pub created_at: String,
    pub dependency: DependabotAlertAutoDismissedAlertDependency,
    pub dismissed_at: Option<String>,
    pub dismissed_by: DiscussionTransferredChangesNewRepositoryOrganization,
    pub dismissed_comment: Option<String>,
    pub dismissed_reason: Option<DependabotAlertAutoDismissedAlertDismissedReason>,
    pub fixed_at: Option<String>,
    pub html_url: String,
    pub number: i64,
    pub security_advisory: DependabotAlertAutoDismissedAlertSecurityAdvisory,
    pub security_vulnerability: DependabotAlertAutoDismissedAlertSecurityAdvisoryVulnerabilities,
    pub state: DependabotAlertAutoDismissedAlertState,
    pub updated_at: String,
    pub url: String,
}
impl From<&DependabotAlertAutoDismissedAlert> for DependabotAlertAutoDismissedAlert {
    fn from(value: &DependabotAlertAutoDismissedAlert) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DependabotAlertAutoDismissedAlertDependency {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package: Option<DependabotAlertAutoDismissedAlertDependencyPackage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<DependabotAlertAutoDismissedAlertDependencyScope>,
}
impl From<&DependabotAlertAutoDismissedAlertDependency>
    for DependabotAlertAutoDismissedAlertDependency
{
    fn from(value: &DependabotAlertAutoDismissedAlertDependency) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DependabotAlertAutoDismissedAlertDependencyPackage {
    pub ecosystem: String,
    pub name: String,
}
impl From<&DependabotAlertAutoDismissedAlertDependencyPackage>
    for DependabotAlertAutoDismissedAlertDependencyPackage
{
    fn from(value: &DependabotAlertAutoDismissedAlertDependencyPackage) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DependabotAlertAutoDismissedAlertDependencyScope {
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "runtime")]
    Runtime,
}
impl From<&DependabotAlertAutoDismissedAlertDependencyScope>
    for DependabotAlertAutoDismissedAlertDependencyScope
{
    fn from(value: &DependabotAlertAutoDismissedAlertDependencyScope) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DependabotAlertAutoDismissedAlertDependencyScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Development => write!(f, "development"),
            Self::Runtime => write!(f, "runtime"),
        }
    }
}
impl std::str::FromStr for DependabotAlertAutoDismissedAlertDependencyScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "development" => Ok(Self::Development),
            "runtime" => Ok(Self::Runtime),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DependabotAlertAutoDismissedAlertDependencyScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DependabotAlertAutoDismissedAlertDependencyScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DependabotAlertAutoDismissedAlertDependencyScope {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DependabotAlertAutoDismissedAlertDismissedReason {
    #[serde(rename = "fix_started")]
    FixStarted,
    #[serde(rename = "inaccurate")]
    Inaccurate,
    #[serde(rename = "no_bandwidth")]
    NoBandwidth,
    #[serde(rename = "not_used")]
    NotUsed,
    #[serde(rename = "tolerable_risk")]
    TolerableRisk,
}
impl From<&DependabotAlertAutoDismissedAlertDismissedReason>
    for DependabotAlertAutoDismissedAlertDismissedReason
{
    fn from(value: &DependabotAlertAutoDismissedAlertDismissedReason) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DependabotAlertAutoDismissedAlertDismissedReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FixStarted => write!(f, "fix_started"),
            Self::Inaccurate => write!(f, "inaccurate"),
            Self::NoBandwidth => write!(f, "no_bandwidth"),
            Self::NotUsed => write!(f, "not_used"),
            Self::TolerableRisk => write!(f, "tolerable_risk"),
        }
    }
}
impl std::str::FromStr for DependabotAlertAutoDismissedAlertDismissedReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "fix_started" => Ok(Self::FixStarted),
            "inaccurate" => Ok(Self::Inaccurate),
            "no_bandwidth" => Ok(Self::NoBandwidth),
            "not_used" => Ok(Self::NotUsed),
            "tolerable_risk" => Ok(Self::TolerableRisk),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DependabotAlertAutoDismissedAlertDismissedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DependabotAlertAutoDismissedAlertDismissedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DependabotAlertAutoDismissedAlertDismissedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DependabotAlertAutoDismissedAlertSecurityAdvisory {
    pub cve_id: Option<String>,
    pub cvss: DependabotAlertAutoDismissedAlertSecurityAdvisoryCvss,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvss_severities: Option<RepositoryAdvisoryPublishedRepositoryAdvisoryCvssSeverities>,
    pub cwes: Vec<RepositoryAdvisoryPublishedRepositoryAdvisoryCwes>,
    pub description: String,
    pub ghsa_id: String,
    pub identifiers: Vec<RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiers>,
    pub published_at: String,
    pub references: Vec<DependabotAlertAutoDismissedAlertSecurityAdvisoryReferences>,
    pub severity: RepositoryAdvisoryPublishedRepositoryAdvisorySeverity,
    pub summary: String,
    pub updated_at: String,
    pub vulnerabilities: Vec<DependabotAlertAutoDismissedAlertSecurityAdvisoryVulnerabilities>,
    pub withdrawn_at: Option<String>,
}
impl From<&DependabotAlertAutoDismissedAlertSecurityAdvisory>
    for DependabotAlertAutoDismissedAlertSecurityAdvisory
{
    fn from(value: &DependabotAlertAutoDismissedAlertSecurityAdvisory) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DependabotAlertAutoDismissedAlertSecurityAdvisoryCvss {
    pub score: f64,
    pub vector_string: Option<String>,
}
impl From<&DependabotAlertAutoDismissedAlertSecurityAdvisoryCvss>
    for DependabotAlertAutoDismissedAlertSecurityAdvisoryCvss
{
    fn from(value: &DependabotAlertAutoDismissedAlertSecurityAdvisoryCvss) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DependabotAlertAutoDismissedAlertSecurityAdvisoryReferences {
    pub url: String,
}
impl From<&DependabotAlertAutoDismissedAlertSecurityAdvisoryReferences>
    for DependabotAlertAutoDismissedAlertSecurityAdvisoryReferences
{
    fn from(value: &DependabotAlertAutoDismissedAlertSecurityAdvisoryReferences) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DependabotAlertAutoDismissedAlertSecurityAdvisoryVulnerabilities {
    pub first_patched_version: Option<CheckRunRequestedActionRequestedAction>,
    pub package: DependabotAlertAutoDismissedAlertDependencyPackage,
    pub severity: RepositoryAdvisoryPublishedRepositoryAdvisorySeverity,
    pub vulnerable_version_range: String,
}
impl From<&DependabotAlertAutoDismissedAlertSecurityAdvisoryVulnerabilities>
    for DependabotAlertAutoDismissedAlertSecurityAdvisoryVulnerabilities
{
    fn from(value: &DependabotAlertAutoDismissedAlertSecurityAdvisoryVulnerabilities) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DependabotAlertAutoDismissedAlertState {
    #[serde(rename = "auto_dismissed")]
    AutoDismissed,
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "open")]
    Open,
}
impl From<&DependabotAlertAutoDismissedAlertState> for DependabotAlertAutoDismissedAlertState {
    fn from(value: &DependabotAlertAutoDismissedAlertState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DependabotAlertAutoDismissedAlertState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AutoDismissed => write!(f, "auto_dismissed"),
            Self::Dismissed => write!(f, "dismissed"),
            Self::Fixed => write!(f, "fixed"),
            Self::Open => write!(f, "open"),
        }
    }
}
impl std::str::FromStr for DependabotAlertAutoDismissedAlertState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "auto_dismissed" => Ok(Self::AutoDismissed),
            "dismissed" => Ok(Self::Dismissed),
            "fixed" => Ok(Self::Fixed),
            "open" => Ok(Self::Open),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DependabotAlertAutoDismissedAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DependabotAlertAutoDismissedAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DependabotAlertAutoDismissedAlertState {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum DeployKey {
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        key: DeployKeyCreatedKey,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        key: DeployKeyCreatedKey,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&DeployKey> for DeployKey {
    fn from(value: &DeployKey) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeployKeyCreatedKey {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_by: Option<String>,
    pub created_at: String,
    pub id: i64,
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_used: Option<String>,
    pub read_only: bool,
    pub title: String,
    pub url: String,
    pub verified: bool,
}
impl From<&DeployKeyCreatedKey> for DeployKeyCreatedKey {
    fn from(value: &DeployKeyCreatedKey) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Deployment {
    #[serde(rename = "created")]
    Created {
        deployment: DeploymentCreatedDeployment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
        workflow: Option<DeploymentStatusCreatedWorkflow>,
        workflow_run: Option<DeploymentCreatedWorkflowRun>,
    },
}
impl From<&Deployment> for Deployment {
    fn from(value: &Deployment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentCreatedDeployment {
    pub created_at: String,
    pub creator: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub description: Option<String>,
    pub environment: String,
    pub id: i64,
    pub node_id: String,
    pub original_environment: String,
    pub payload: DeploymentCreatedDeploymentPayload,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_environment: Option<bool>,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository_url: String,
    pub sha: String,
    pub statuses_url: String,
    pub task: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_environment: Option<bool>,
    pub updated_at: String,
    pub url: String,
}
impl From<&DeploymentCreatedDeployment> for DeploymentCreatedDeployment {
    fn from(value: &DeploymentCreatedDeployment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum DeploymentCreatedDeploymentPayload {
    Variant0(Untyped),
    Variant1(String),
}
impl From<&DeploymentCreatedDeploymentPayload> for DeploymentCreatedDeploymentPayload {
    fn from(value: &DeploymentCreatedDeploymentPayload) -> Self {
        value.clone()
    }
}
impl From<Untyped> for DeploymentCreatedDeploymentPayload {
    fn from(value: Untyped) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentCreatedWorkflowRun {
    pub actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    pub check_suite_id: i64,
    pub check_suite_node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_suite_url: Option<String>,
    pub conclusion: Option<DeploymentCreatedWorkflowRunConclusion>,
    pub created_at: String,
    pub display_title: String,
    pub event: String,
    pub head_branch: String,
    #[serde(default)]
    pub head_commit: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_repository: Option<DeploymentStatusCreatedWorkflowRunHeadRepository>,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jobs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub path: String,
    #[serde(default)]
    pub previous_attempt_url: (),
    pub pull_requests: Vec<DeploymentStatusCreatedWorkflowRunPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_workflows: Option<Vec<DeploymentStatusCreatedWorkflowRunReferencedWorkflows>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<DeploymentStatusCreatedWorkflowRunHeadRepository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_url: Option<String>,
    pub run_attempt: i64,
    pub run_number: i64,
    pub run_started_at: String,
    pub status: DeploymentStatusCreatedWorkflowRunStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triggering_actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub updated_at: String,
    pub url: String,
    pub workflow_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_url: Option<String>,
}
impl From<&DeploymentCreatedWorkflowRun> for DeploymentCreatedWorkflowRun {
    fn from(value: &DeploymentCreatedWorkflowRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeploymentCreatedWorkflowRunConclusion {
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "timed_out")]
    TimedOut,
}
impl From<&DeploymentCreatedWorkflowRunConclusion> for DeploymentCreatedWorkflowRunConclusion {
    fn from(value: &DeploymentCreatedWorkflowRunConclusion) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeploymentCreatedWorkflowRunConclusion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ActionRequired => write!(f, "action_required"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Failure => write!(f, "failure"),
            Self::Neutral => write!(f, "neutral"),
            Self::Stale => write!(f, "stale"),
            Self::Success => write!(f, "success"),
            Self::TimedOut => write!(f, "timed_out"),
        }
    }
}
impl std::str::FromStr for DeploymentCreatedWorkflowRunConclusion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "action_required" => Ok(Self::ActionRequired),
            "cancelled" => Ok(Self::Cancelled),
            "failure" => Ok(Self::Failure),
            "neutral" => Ok(Self::Neutral),
            "stale" => Ok(Self::Stale),
            "success" => Ok(Self::Success),
            "timed_out" => Ok(Self::TimedOut),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DeploymentCreatedWorkflowRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DeploymentCreatedWorkflowRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DeploymentCreatedWorkflowRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentProtectionRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<DeploymentProtectionRuleRequestedDeployment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_callback_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pull_requests: Vec<DeploymentProtectionRuleRequestedPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<Untyped>,
}
impl From<&DeploymentProtectionRule> for DeploymentProtectionRule {
    fn from(value: &DeploymentProtectionRule) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentProtectionRuleRequestedDeployment {
    pub created_at: String,
    pub creator: DiscussionTransferredChangesNewRepositoryOrganization,
    pub description: Option<String>,
    pub environment: String,
    pub id: i64,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_environment: Option<String>,
    pub payload: DeploymentProtectionRuleRequestedDeploymentPayload,
    #[serde(default)]
    pub performed_via_github_app: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_environment: Option<bool>,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository_url: String,
    pub sha: String,
    pub statuses_url: String,
    pub task: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_environment: Option<bool>,
    pub updated_at: String,
    pub url: String,
}
impl From<&DeploymentProtectionRuleRequestedDeployment>
    for DeploymentProtectionRuleRequestedDeployment
{
    fn from(value: &DeploymentProtectionRuleRequestedDeployment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum DeploymentProtectionRuleRequestedDeploymentPayload {
    Variant0(Untyped),
    Variant1(String),
}
impl From<&DeploymentProtectionRuleRequestedDeploymentPayload>
    for DeploymentProtectionRuleRequestedDeploymentPayload
{
    fn from(value: &DeploymentProtectionRuleRequestedDeploymentPayload) -> Self {
        value.clone()
    }
}
impl From<Untyped> for DeploymentProtectionRuleRequestedDeploymentPayload {
    fn from(value: Untyped) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentProtectionRuleRequestedPullRequests {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub additions: i64,
    pub assignee: DiscussionTransferredChangesNewRepositoryOrganization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<DiscussionTransferredChangesNewRepositoryOrganization>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<DeploymentProtectionRuleRequestedPullRequestsAutoMerge>,
    pub base: DeploymentProtectionRuleRequestedPullRequestsHead,
    pub body: Option<String>,
    pub changed_files: i64,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub commits: i64,
    pub commits_url: String,
    pub created_at: String,
    pub deletions: i64,
    pub diff_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub head: DeploymentProtectionRuleRequestedPullRequestsHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionAnsweredDiscussionLabels>,
    #[serde(rename = "_links")]
    pub links: DeploymentProtectionRuleRequestedPullRequestsLinks,
    pub locked: bool,
    pub maintainer_can_modify: bool,
    pub merge_commit_sha: Option<String>,
    pub mergeable: Option<bool>,
    pub mergeable_state: String,
    pub merged: bool,
    pub merged_at: Option<String>,
    pub merged_by: DiscussionTransferredChangesNewRepositoryOrganization,
    pub milestone: DeploymentProtectionRuleRequestedPullRequestsMilestone,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_reviewers: Option<Vec<DiscussionTransferredChangesNewRepositoryOrganization>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_teams: Option<Vec<DeploymentProtectionRuleRequestedPullRequestsRequestedTeams>>,
    pub review_comment_url: String,
    pub review_comments: i64,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: DiscussionTransferredChangesNewRepositoryOrganization,
}
impl From<&DeploymentProtectionRuleRequestedPullRequests>
    for DeploymentProtectionRuleRequestedPullRequests
{
    fn from(value: &DeploymentProtectionRuleRequestedPullRequests) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentProtectionRuleRequestedPullRequestsAutoMerge {
    pub commit_message: String,
    pub commit_title: String,
    pub enabled_by: DiscussionTransferredChangesNewRepositoryOrganization,
    pub merge_method: DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod,
}
impl From<&DeploymentProtectionRuleRequestedPullRequestsAutoMerge>
    for DeploymentProtectionRuleRequestedPullRequestsAutoMerge
{
    fn from(value: &DeploymentProtectionRuleRequestedPullRequestsAutoMerge) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod {
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "rebase")]
    Rebase,
    #[serde(rename = "squash")]
    Squash,
}
impl From<&DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod>
    for DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod
{
    fn from(value: &DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Merge => write!(f, "merge"),
            Self::Rebase => write!(f, "rebase"),
            Self::Squash => write!(f, "squash"),
        }
    }
}
impl std::str::FromStr for DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "merge" => Ok(Self::Merge),
            "rebase" => Ok(Self::Rebase),
            "squash" => Ok(Self::Squash),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentProtectionRuleRequestedPullRequestsHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: DeploymentProtectionRuleRequestedPullRequestsHeadRepo,
    pub sha: String,
    pub user: DiscussionTransferredChangesNewRepositoryOrganization,
}
impl From<&DeploymentProtectionRuleRequestedPullRequestsHead>
    for DeploymentProtectionRuleRequestedPullRequestsHead
{
    fn from(value: &DeploymentProtectionRuleRequestedPullRequestsHead) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentProtectionRuleRequestedPullRequestsHeadRepo {
    #[serde(default)]
    pub allow_auto_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_merge_commit: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_rebase_merge: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_squash_merge: bool,
    #[serde(default)]
    pub allow_update_branch: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anonymous_access_enabled: Option<bool>,
    pub archive_url: String,
    pub archived: bool,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: Option<String>,
    pub default_branch: String,
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deployments_url: String,
    pub description: Option<String>,
    pub disabled: bool,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    #[serde(default)]
    pub has_discussions: bool,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default)]
    pub is_template: bool,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub license: DiscussionTransferredChangesNewRepositoryLicense,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle>,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    pub owner: DiscussionTransferredChangesNewRepositoryOrganization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DiscussionTransferredChangesNewRepositoryPermissions>,
    pub private: bool,
    pub pulls_url: String,
    pub pushed_at: Option<String>,
    pub releases_url: String,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle>,
    pub ssh_url: String,
    pub stargazers_count: i64,
    pub stargazers_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_at: Option<String>,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temp_clone_token: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    pub trees_url: String,
    pub updated_at: Option<String>,
    pub url: String,
    #[serde(default)]
    pub use_squash_pr_title_as_default: bool,
    #[serde(
        default = "defaults::deployment_protection_rule_requested_pull_requests_head_repo_visibility"
    )]
    pub visibility: String,
    pub watchers: i64,
    pub watchers_count: i64,
    #[serde(default)]
    pub web_commit_signoff_required: bool,
}
impl From<&DeploymentProtectionRuleRequestedPullRequestsHeadRepo>
    for DeploymentProtectionRuleRequestedPullRequestsHeadRepo
{
    fn from(value: &DeploymentProtectionRuleRequestedPullRequestsHeadRepo) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentProtectionRuleRequestedPullRequestsLinks {
    pub comments: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub commits: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub html: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub issue: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub review_comment: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub review_comments: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    #[serde(rename = "self")]
    pub self_: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub statuses: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
}
impl From<&DeploymentProtectionRuleRequestedPullRequestsLinks>
    for DeploymentProtectionRuleRequestedPullRequestsLinks
{
    fn from(value: &DeploymentProtectionRuleRequestedPullRequestsLinks) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentProtectionRuleRequestedPullRequestsMilestone {
    pub closed_at: Option<String>,
    pub closed_issues: i64,
    pub created_at: String,
    pub creator: DiscussionTransferredChangesNewRepositoryOrganization,
    pub description: Option<String>,
    pub due_on: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub labels_url: String,
    pub node_id: String,
    pub number: i64,
    pub open_issues: i64,
    pub state: DeploymentProtectionRuleRequestedPullRequestsMilestoneState,
    pub title: String,
    pub updated_at: String,
    pub url: String,
}
impl From<&DeploymentProtectionRuleRequestedPullRequestsMilestone>
    for DeploymentProtectionRuleRequestedPullRequestsMilestone
{
    fn from(value: &DeploymentProtectionRuleRequestedPullRequestsMilestone) -> Self {
        value.clone()
    }
}
#[derive(
    Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Default,
)]
pub enum DeploymentProtectionRuleRequestedPullRequestsMilestoneState {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "open")]
    #[default]
    Open,
}
impl From<&DeploymentProtectionRuleRequestedPullRequestsMilestoneState>
    for DeploymentProtectionRuleRequestedPullRequestsMilestoneState
{
    fn from(value: &DeploymentProtectionRuleRequestedPullRequestsMilestoneState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeploymentProtectionRuleRequestedPullRequestsMilestoneState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Closed => write!(f, "closed"),
            Self::Open => write!(f, "open"),
        }
    }
}
impl std::str::FromStr for DeploymentProtectionRuleRequestedPullRequestsMilestoneState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "closed" => Ok(Self::Closed),
            "open" => Ok(Self::Open),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DeploymentProtectionRuleRequestedPullRequestsMilestoneState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for DeploymentProtectionRuleRequestedPullRequestsMilestoneState
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DeploymentProtectionRuleRequestedPullRequestsMilestoneState {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentProtectionRuleRequestedPullRequestsRequestedTeams {
    pub description: Option<String>,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ldap_dn: Option<String>,
    pub members_url: String,
    pub name: String,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<String>,
    pub permission: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    pub repositories_url: String,
    pub slug: String,
    pub url: String,
}
impl From<&DeploymentProtectionRuleRequestedPullRequestsRequestedTeams>
    for DeploymentProtectionRuleRequestedPullRequestsRequestedTeams
{
    fn from(value: &DeploymentProtectionRuleRequestedPullRequestsRequestedTeams) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeploymentProtectionRuleRequestedPullRequestsState {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "open")]
    Open,
}
impl From<&DeploymentProtectionRuleRequestedPullRequestsState>
    for DeploymentProtectionRuleRequestedPullRequestsState
{
    fn from(value: &DeploymentProtectionRuleRequestedPullRequestsState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeploymentProtectionRuleRequestedPullRequestsState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Closed => write!(f, "closed"),
            Self::Open => write!(f, "open"),
        }
    }
}
impl std::str::FromStr for DeploymentProtectionRuleRequestedPullRequestsState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "closed" => Ok(Self::Closed),
            "open" => Ok(Self::Open),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DeploymentProtectionRuleRequestedPullRequestsState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DeploymentProtectionRuleRequestedPullRequestsState {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DeploymentProtectionRuleRequestedPullRequestsState {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum DeploymentReview {
    #[serde(rename = "approved")]
    Approved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        approver: Option<DeploymentReviewApprovedApprover>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        organization: Untyped,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        reviewers: Vec<DeploymentReviewApprovedReviewers>,
        sender: Untyped,
        since: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        workflow_job_run: Option<DeploymentReviewApprovedWorkflowJobRun>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        workflow_job_runs: Vec<DeploymentReviewApprovedWorkflowJobRuns>,
        workflow_run: Option<DeploymentReviewApprovedWorkflowRun>,
    },
    #[serde(rename = "rejected")]
    Rejected {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        approver: Option<DeploymentReviewApprovedApprover>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        comment: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        organization: Untyped,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        reviewers: Vec<DeploymentReviewApprovedReviewers>,
        sender: Untyped,
        since: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        workflow_job_run: Option<DeploymentReviewApprovedWorkflowJobRun>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        workflow_job_runs: Vec<DeploymentReviewRejectedWorkflowJobRuns>,
        workflow_run: Option<DeploymentReviewRejectedWorkflowRun>,
    },
    #[serde(rename = "requested")]
    Requested {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        environment: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        organization: Untyped,
        repository: Untyped,
        requestor: Option<DeploymentStatusCreatedDeploymentCreator>,
        reviewers: Vec<DeploymentReviewRequestedReviewers>,
        sender: Untyped,
        since: String,
        workflow_job_run: DeploymentReviewApprovedWorkflowJobRuns,
        workflow_run: Option<DeploymentReviewRequestedWorkflowRun>,
    },
}
impl From<&DeploymentReview> for DeploymentReview {
    fn from(value: &DeploymentReview) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentReviewApprovedApprover {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_view_type: Option<String>,
}
impl From<&DeploymentReviewApprovedApprover> for DeploymentReviewApprovedApprover {
    fn from(value: &DeploymentReviewApprovedApprover) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentReviewApprovedReviewers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<DeploymentReviewApprovedReviewersReviewer>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl From<&DeploymentReviewApprovedReviewers> for DeploymentReviewApprovedReviewers {
    fn from(value: &DeploymentReviewApprovedReviewers) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentReviewApprovedReviewersReviewer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub id: i64,
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<DeploymentStatusCreatedDeploymentCreatorType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&DeploymentReviewApprovedReviewersReviewer>
    for DeploymentReviewApprovedReviewersReviewer
{
    fn from(value: &DeploymentReviewApprovedReviewersReviewer) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentReviewApprovedWorkflowJobRun {
    pub conclusion: (),
    pub created_at: String,
    pub environment: String,
    pub html_url: String,
    pub id: i64,
    pub name: (),
    pub status: String,
    pub updated_at: String,
}
impl From<&DeploymentReviewApprovedWorkflowJobRun> for DeploymentReviewApprovedWorkflowJobRun {
    fn from(value: &DeploymentReviewApprovedWorkflowJobRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentReviewApprovedWorkflowJobRuns {
    #[serde(default)]
    pub conclusion: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl From<&DeploymentReviewApprovedWorkflowJobRuns> for DeploymentReviewApprovedWorkflowJobRuns {
    fn from(value: &DeploymentReviewApprovedWorkflowJobRuns) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentReviewApprovedWorkflowRun {
    pub actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    pub check_suite_id: i64,
    pub check_suite_node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_suite_url: Option<String>,
    pub conclusion: Option<DeploymentCreatedWorkflowRunConclusion>,
    pub created_at: String,
    pub display_title: String,
    pub event: String,
    pub head_branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_commit: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_repository: Option<DeploymentReviewApprovedWorkflowRunHeadRepository>,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jobs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_attempt_url: Option<String>,
    pub pull_requests: Vec<DeploymentStatusCreatedWorkflowRunPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_workflows: Option<Vec<DeploymentStatusCreatedWorkflowRunReferencedWorkflows>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<DeploymentReviewApprovedWorkflowRunHeadRepository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_url: Option<String>,
    pub run_attempt: i64,
    pub run_number: i64,
    pub run_started_at: String,
    pub status: DeploymentStatusCreatedWorkflowRunStatus,
    pub triggering_actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub updated_at: String,
    pub url: String,
    pub workflow_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_url: Option<String>,
}
impl From<&DeploymentReviewApprovedWorkflowRun> for DeploymentReviewApprovedWorkflowRun {
    fn from(value: &DeploymentReviewApprovedWorkflowRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentReviewApprovedWorkflowRunHeadRepository {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignees_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blobs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branches_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaborators_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributors_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downloads_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fork: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_commits_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_refs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_tags_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_comment_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub languages_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestones_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<DeploymentReviewApprovedApprover>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulls_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub releases_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teams_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trees_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&DeploymentReviewApprovedWorkflowRunHeadRepository>
    for DeploymentReviewApprovedWorkflowRunHeadRepository
{
    fn from(value: &DeploymentReviewApprovedWorkflowRunHeadRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentReviewRejectedWorkflowJobRuns {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conclusion: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl From<&DeploymentReviewRejectedWorkflowJobRuns> for DeploymentReviewRejectedWorkflowJobRuns {
    fn from(value: &DeploymentReviewRejectedWorkflowJobRuns) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentReviewRejectedWorkflowRun {
    pub actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    pub check_suite_id: i64,
    pub check_suite_node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_suite_url: Option<String>,
    pub conclusion: Option<DeploymentCreatedWorkflowRunConclusion>,
    pub created_at: String,
    pub display_title: String,
    pub event: String,
    pub head_branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_commit: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_repository: Option<DeploymentReviewApprovedWorkflowRunHeadRepository>,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jobs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_attempt_url: Option<String>,
    pub pull_requests: Vec<DeploymentStatusCreatedWorkflowRunPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_workflows: Option<Vec<DeploymentStatusCreatedWorkflowRunReferencedWorkflows>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<DeploymentReviewApprovedWorkflowRunHeadRepository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_url: Option<String>,
    pub run_attempt: i64,
    pub run_number: i64,
    pub run_started_at: String,
    pub status: DeploymentReviewRejectedWorkflowRunStatus,
    pub triggering_actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub updated_at: String,
    pub url: String,
    pub workflow_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_url: Option<String>,
}
impl From<&DeploymentReviewRejectedWorkflowRun> for DeploymentReviewRejectedWorkflowRun {
    fn from(value: &DeploymentReviewRejectedWorkflowRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeploymentReviewRejectedWorkflowRunStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "waiting")]
    Waiting,
}
impl From<&DeploymentReviewRejectedWorkflowRunStatus>
    for DeploymentReviewRejectedWorkflowRunStatus
{
    fn from(value: &DeploymentReviewRejectedWorkflowRunStatus) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeploymentReviewRejectedWorkflowRunStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => write!(f, "completed"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Queued => write!(f, "queued"),
            Self::Requested => write!(f, "requested"),
            Self::Waiting => write!(f, "waiting"),
        }
    }
}
impl std::str::FromStr for DeploymentReviewRejectedWorkflowRunStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "in_progress" => Ok(Self::InProgress),
            "queued" => Ok(Self::Queued),
            "requested" => Ok(Self::Requested),
            "waiting" => Ok(Self::Waiting),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DeploymentReviewRejectedWorkflowRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DeploymentReviewRejectedWorkflowRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DeploymentReviewRejectedWorkflowRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentReviewRequestedReviewers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<DeploymentReviewRequestedReviewersType>,
}
impl From<&DeploymentReviewRequestedReviewers> for DeploymentReviewRequestedReviewers {
    fn from(value: &DeploymentReviewRequestedReviewers) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeploymentReviewRequestedReviewersType {
    Team,
    User,
}
impl From<&DeploymentReviewRequestedReviewersType> for DeploymentReviewRequestedReviewersType {
    fn from(value: &DeploymentReviewRequestedReviewersType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeploymentReviewRequestedReviewersType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Team => write!(f, "Team"),
            Self::User => write!(f, "User"),
        }
    }
}
impl std::str::FromStr for DeploymentReviewRequestedReviewersType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Team" => Ok(Self::Team),
            "User" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DeploymentReviewRequestedReviewersType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DeploymentReviewRequestedReviewersType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DeploymentReviewRequestedReviewersType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentReviewRequestedWorkflowRun {
    pub actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    pub check_suite_id: i64,
    pub check_suite_node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_suite_url: Option<String>,
    pub conclusion: Option<DeploymentCreatedWorkflowRunConclusion>,
    pub created_at: String,
    pub display_title: String,
    pub event: String,
    pub head_branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_commit: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_repository: Option<DeploymentReviewApprovedWorkflowRunHeadRepository>,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jobs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_attempt_url: Option<String>,
    pub pull_requests: Vec<DeploymentStatusCreatedWorkflowRunPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_workflows: Option<Vec<DeploymentStatusCreatedWorkflowRunReferencedWorkflows>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<DeploymentReviewApprovedWorkflowRunHeadRepository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_url: Option<String>,
    pub run_attempt: i64,
    pub run_number: i64,
    pub run_started_at: String,
    pub status: DeploymentStatusCreatedWorkflowRunStatus,
    pub triggering_actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub updated_at: String,
    pub url: String,
    pub workflow_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_url: Option<String>,
}
impl From<&DeploymentReviewRequestedWorkflowRun> for DeploymentReviewRequestedWorkflowRun {
    fn from(value: &DeploymentReviewRequestedWorkflowRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum DeploymentStatus {
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        check_run: Option<DeploymentStatusCreatedCheckRun>,
        deployment: DeploymentStatusCreatedDeployment,
        deployment_status: DeploymentStatusCreatedDeploymentStatus,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        workflow: Option<DeploymentStatusCreatedWorkflow>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        workflow_run: Option<DeploymentStatusCreatedWorkflowRun>,
    },
}
impl From<&DeploymentStatus> for DeploymentStatus {
    fn from(value: &DeploymentStatus) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedCheckRun {
    pub completed_at: Option<String>,
    pub conclusion: Option<DeploymentStatusCreatedCheckRunConclusion>,
    pub details_url: String,
    pub external_id: String,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub started_at: String,
    pub status: DeploymentStatusCreatedCheckRunStatus,
    pub url: String,
}
impl From<&DeploymentStatusCreatedCheckRun> for DeploymentStatusCreatedCheckRun {
    fn from(value: &DeploymentStatusCreatedCheckRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeploymentStatusCreatedCheckRunConclusion {
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "timed_out")]
    TimedOut,
}
impl From<&DeploymentStatusCreatedCheckRunConclusion>
    for DeploymentStatusCreatedCheckRunConclusion
{
    fn from(value: &DeploymentStatusCreatedCheckRunConclusion) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeploymentStatusCreatedCheckRunConclusion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ActionRequired => write!(f, "action_required"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Failure => write!(f, "failure"),
            Self::Neutral => write!(f, "neutral"),
            Self::Skipped => write!(f, "skipped"),
            Self::Stale => write!(f, "stale"),
            Self::Success => write!(f, "success"),
            Self::TimedOut => write!(f, "timed_out"),
        }
    }
}
impl std::str::FromStr for DeploymentStatusCreatedCheckRunConclusion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "action_required" => Ok(Self::ActionRequired),
            "cancelled" => Ok(Self::Cancelled),
            "failure" => Ok(Self::Failure),
            "neutral" => Ok(Self::Neutral),
            "skipped" => Ok(Self::Skipped),
            "stale" => Ok(Self::Stale),
            "success" => Ok(Self::Success),
            "timed_out" => Ok(Self::TimedOut),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DeploymentStatusCreatedCheckRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DeploymentStatusCreatedCheckRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DeploymentStatusCreatedCheckRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeploymentStatusCreatedCheckRunStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "waiting")]
    Waiting,
}
impl From<&DeploymentStatusCreatedCheckRunStatus> for DeploymentStatusCreatedCheckRunStatus {
    fn from(value: &DeploymentStatusCreatedCheckRunStatus) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeploymentStatusCreatedCheckRunStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => write!(f, "completed"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Pending => write!(f, "pending"),
            Self::Queued => write!(f, "queued"),
            Self::Waiting => write!(f, "waiting"),
        }
    }
}
impl std::str::FromStr for DeploymentStatusCreatedCheckRunStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "in_progress" => Ok(Self::InProgress),
            "pending" => Ok(Self::Pending),
            "queued" => Ok(Self::Queued),
            "waiting" => Ok(Self::Waiting),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DeploymentStatusCreatedCheckRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DeploymentStatusCreatedCheckRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DeploymentStatusCreatedCheckRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedDeployment {
    pub created_at: String,
    pub creator: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub description: Option<String>,
    pub environment: String,
    pub id: i64,
    pub node_id: String,
    pub original_environment: String,
    pub payload: Option<DeploymentStatusCreatedDeploymentPayload>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub production_environment: Option<bool>,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository_url: String,
    pub sha: String,
    pub statuses_url: String,
    pub task: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_environment: Option<bool>,
    pub updated_at: String,
    pub url: String,
}
impl From<&DeploymentStatusCreatedDeployment> for DeploymentStatusCreatedDeployment {
    fn from(value: &DeploymentStatusCreatedDeployment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedDeploymentCreator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub id: i64,
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<DeploymentStatusCreatedDeploymentCreatorType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_view_type: Option<String>,
}
impl From<&DeploymentStatusCreatedDeploymentCreator> for DeploymentStatusCreatedDeploymentCreator {
    fn from(value: &DeploymentStatusCreatedDeploymentCreator) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeploymentStatusCreatedDeploymentCreatorType {
    Bot,
    Organization,
    User,
}
impl From<&DeploymentStatusCreatedDeploymentCreatorType>
    for DeploymentStatusCreatedDeploymentCreatorType
{
    fn from(value: &DeploymentStatusCreatedDeploymentCreatorType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeploymentStatusCreatedDeploymentCreatorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bot => write!(f, "Bot"),
            Self::Organization => write!(f, "Organization"),
            Self::User => write!(f, "User"),
        }
    }
}
impl std::str::FromStr for DeploymentStatusCreatedDeploymentCreatorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Bot" => Ok(Self::Bot),
            "Organization" => Ok(Self::Organization),
            "User" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DeploymentStatusCreatedDeploymentCreatorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DeploymentStatusCreatedDeploymentCreatorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DeploymentStatusCreatedDeploymentCreatorType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum DeploymentStatusCreatedDeploymentPayload {
    Variant0(Untyped),
    Variant1(String),
}
impl From<&DeploymentStatusCreatedDeploymentPayload> for DeploymentStatusCreatedDeploymentPayload {
    fn from(value: &DeploymentStatusCreatedDeploymentPayload) -> Self {
        value.clone()
    }
}
impl From<Untyped> for DeploymentStatusCreatedDeploymentPayload {
    fn from(value: Untyped) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedDeploymentPerformedViaGithubApp {
    pub created_at: Option<String>,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    pub external_url: Option<String>,
    pub html_url: String,
    pub id: Option<i64>,
    pub name: String,
    pub node_id: String,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub updated_at: Option<String>,
}
impl From<&DeploymentStatusCreatedDeploymentPerformedViaGithubApp>
    for DeploymentStatusCreatedDeploymentPerformedViaGithubApp
{
    fn from(value: &DeploymentStatusCreatedDeploymentPerformedViaGithubApp) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administration:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_references:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discussions:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environments:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_administration:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_hooks:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_packages:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_plan:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_projects:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_secrets:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_self_hosted_runners:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_user_blocking:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_requests:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_hooks:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_projects:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning_alerts:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_events:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_scanning_alert:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_file:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_discussions:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vulnerability_alerts:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
}
impl From<&DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissions>
    for DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissions
{
    fn from(value: &DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissions) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl From<&DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>
    for DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions
{
    fn from(
        value: &DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions,
    ) -> Self {
        *value
    }
}
impl ::std::fmt::Display
    for DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Read => write!(f, "read"),
            Self::Write => write!(f, "write"),
        }
    }
}
impl std::str::FromStr
    for DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "read" => Ok(Self::Read),
            "write" => Ok(Self::Write),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedDeploymentStatus {
    pub created_at: String,
    pub creator: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub deployment_url: String,
    pub description: String,
    pub environment: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_url: Option<String>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubApp>,
    pub repository_url: String,
    pub state: String,
    pub target_url: String,
    pub updated_at: String,
    pub url: String,
}
impl From<&DeploymentStatusCreatedDeploymentStatus> for DeploymentStatusCreatedDeploymentStatus {
    fn from(value: &DeploymentStatusCreatedDeploymentStatus) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedWorkflow {
    pub badge_url: String,
    pub created_at: String,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub path: String,
    pub state: String,
    pub updated_at: String,
    pub url: String,
}
impl From<&DeploymentStatusCreatedWorkflow> for DeploymentStatusCreatedWorkflow {
    fn from(value: &DeploymentStatusCreatedWorkflow) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedWorkflowRun {
    pub actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    pub check_suite_id: i64,
    pub check_suite_node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_suite_url: Option<String>,
    pub conclusion: Option<DeploymentStatusCreatedWorkflowRunConclusion>,
    pub created_at: String,
    pub display_title: String,
    pub event: String,
    pub head_branch: String,
    #[serde(default)]
    pub head_commit: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_repository: Option<DeploymentStatusCreatedWorkflowRunHeadRepository>,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jobs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logs_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub path: String,
    #[serde(default)]
    pub previous_attempt_url: (),
    pub pull_requests: Vec<DeploymentStatusCreatedWorkflowRunPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_workflows: Option<Vec<DeploymentStatusCreatedWorkflowRunReferencedWorkflows>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<DeploymentStatusCreatedWorkflowRunHeadRepository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_url: Option<String>,
    pub run_attempt: i64,
    pub run_number: i64,
    pub run_started_at: String,
    pub status: DeploymentStatusCreatedWorkflowRunStatus,
    pub triggering_actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub updated_at: String,
    pub url: String,
    pub workflow_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_url: Option<String>,
}
impl From<&DeploymentStatusCreatedWorkflowRun> for DeploymentStatusCreatedWorkflowRun {
    fn from(value: &DeploymentStatusCreatedWorkflowRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeploymentStatusCreatedWorkflowRunConclusion {
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "startup_failure")]
    StartupFailure,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "timed_out")]
    TimedOut,
}
impl From<&DeploymentStatusCreatedWorkflowRunConclusion>
    for DeploymentStatusCreatedWorkflowRunConclusion
{
    fn from(value: &DeploymentStatusCreatedWorkflowRunConclusion) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeploymentStatusCreatedWorkflowRunConclusion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ActionRequired => write!(f, "action_required"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Failure => write!(f, "failure"),
            Self::Neutral => write!(f, "neutral"),
            Self::Stale => write!(f, "stale"),
            Self::StartupFailure => write!(f, "startup_failure"),
            Self::Success => write!(f, "success"),
            Self::TimedOut => write!(f, "timed_out"),
        }
    }
}
impl std::str::FromStr for DeploymentStatusCreatedWorkflowRunConclusion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "action_required" => Ok(Self::ActionRequired),
            "cancelled" => Ok(Self::Cancelled),
            "failure" => Ok(Self::Failure),
            "neutral" => Ok(Self::Neutral),
            "stale" => Ok(Self::Stale),
            "startup_failure" => Ok(Self::StartupFailure),
            "success" => Ok(Self::Success),
            "timed_out" => Ok(Self::TimedOut),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DeploymentStatusCreatedWorkflowRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DeploymentStatusCreatedWorkflowRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DeploymentStatusCreatedWorkflowRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedWorkflowRunHeadRepository {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignees_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blobs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branches_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaborators_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributors_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments_url: Option<String>,
    #[serde(default)]
    pub description: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downloads_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fork: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_commits_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_refs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_tags_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_comment_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub languages_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestones_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<DeploymentStatusCreatedWorkflowRunHeadRepositoryOwner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulls_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub releases_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teams_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trees_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&DeploymentStatusCreatedWorkflowRunHeadRepository>
    for DeploymentStatusCreatedWorkflowRunHeadRepository
{
    fn from(value: &DeploymentStatusCreatedWorkflowRunHeadRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedWorkflowRunHeadRepositoryOwner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&DeploymentStatusCreatedWorkflowRunHeadRepositoryOwner>
    for DeploymentStatusCreatedWorkflowRunHeadRepositoryOwner
{
    fn from(value: &DeploymentStatusCreatedWorkflowRunHeadRepositoryOwner) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedWorkflowRunPullRequests {
    pub base: DeploymentStatusCreatedWorkflowRunPullRequestsBase,
    pub head: DeploymentStatusCreatedWorkflowRunPullRequestsBase,
    pub id: i64,
    pub number: i64,
    pub url: String,
}
impl From<&DeploymentStatusCreatedWorkflowRunPullRequests>
    for DeploymentStatusCreatedWorkflowRunPullRequests
{
    fn from(value: &DeploymentStatusCreatedWorkflowRunPullRequests) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedWorkflowRunPullRequestsBase {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: DeploymentStatusCreatedWorkflowRunPullRequestsBaseRepo,
    pub sha: String,
}
impl From<&DeploymentStatusCreatedWorkflowRunPullRequestsBase>
    for DeploymentStatusCreatedWorkflowRunPullRequestsBase
{
    fn from(value: &DeploymentStatusCreatedWorkflowRunPullRequestsBase) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedWorkflowRunPullRequestsBaseRepo {
    pub id: i64,
    pub name: String,
    pub url: String,
}
impl From<&DeploymentStatusCreatedWorkflowRunPullRequestsBaseRepo>
    for DeploymentStatusCreatedWorkflowRunPullRequestsBaseRepo
{
    fn from(value: &DeploymentStatusCreatedWorkflowRunPullRequestsBaseRepo) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DeploymentStatusCreatedWorkflowRunReferencedWorkflows {
    pub path: String,
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    pub sha: String,
}
impl From<&DeploymentStatusCreatedWorkflowRunReferencedWorkflows>
    for DeploymentStatusCreatedWorkflowRunReferencedWorkflows
{
    fn from(value: &DeploymentStatusCreatedWorkflowRunReferencedWorkflows) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DeploymentStatusCreatedWorkflowRunStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "waiting")]
    Waiting,
}
impl From<&DeploymentStatusCreatedWorkflowRunStatus> for DeploymentStatusCreatedWorkflowRunStatus {
    fn from(value: &DeploymentStatusCreatedWorkflowRunStatus) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DeploymentStatusCreatedWorkflowRunStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => write!(f, "completed"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Pending => write!(f, "pending"),
            Self::Queued => write!(f, "queued"),
            Self::Requested => write!(f, "requested"),
            Self::Waiting => write!(f, "waiting"),
        }
    }
}
impl std::str::FromStr for DeploymentStatusCreatedWorkflowRunStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "in_progress" => Ok(Self::InProgress),
            "pending" => Ok(Self::Pending),
            "queued" => Ok(Self::Queued),
            "requested" => Ok(Self::Requested),
            "waiting" => Ok(Self::Waiting),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DeploymentStatusCreatedWorkflowRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DeploymentStatusCreatedWorkflowRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DeploymentStatusCreatedWorkflowRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Discussion {
    Answered {
        action: String,
        answer: DiscussionAnsweredAnswer,
        discussion: Box<DiscussionAnsweredDiscussion>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    CategoryChanged {
        action: String,
        changes: DiscussionCategoryChangedChanges,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Closed {
        action: String,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Created {
        action: String,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Deleted {
        action: String,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Edited {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        changes: Option<DiscussionEditedChanges>,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Labeled {
        action: String,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        label: DiscussionLabeledLabel,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Locked {
        action: String,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Pinned {
        action: String,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Reopened {
        action: String,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Transferred {
        action: String,
        changes: DiscussionTransferredChanges,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Unanswered(DiscussionUnanswered),
    Unlabeled {
        action: String,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        label: DiscussionLabeledLabel,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Unlocked {
        action: String,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    Unpinned {
        action: String,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&Discussion> for Discussion {
    fn from(value: &Discussion) -> Self {
        value.clone()
    }
}
impl From<DiscussionUnanswered> for Discussion {
    fn from(value: DiscussionUnanswered) -> Self {
        Self::Unanswered(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionAnsweredAnswer {
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: String,
    pub child_comment_count: i64,
    pub created_at: String,
    pub discussion_id: i64,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub parent_id: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<DiscussionAnsweredAnswerReactions>,
    pub repository_url: String,
    pub updated_at: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&DiscussionAnsweredAnswer> for DiscussionAnsweredAnswer {
    fn from(value: &DiscussionAnsweredAnswer) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiscussionAnsweredAnswerAuthorAssociation {
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
impl From<&DiscussionAnsweredAnswerAuthorAssociation>
    for DiscussionAnsweredAnswerAuthorAssociation
{
    fn from(value: &DiscussionAnsweredAnswerAuthorAssociation) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DiscussionAnsweredAnswerAuthorAssociation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Collaborator => write!(f, "COLLABORATOR"),
            Self::Contributor => write!(f, "CONTRIBUTOR"),
            Self::FirstTimer => write!(f, "FIRST_TIMER"),
            Self::FirstTimeContributor => write!(f, "FIRST_TIME_CONTRIBUTOR"),
            Self::Mannequin => write!(f, "MANNEQUIN"),
            Self::Member => write!(f, "MEMBER"),
            Self::None => write!(f, "NONE"),
            Self::Owner => write!(f, "OWNER"),
        }
    }
}
impl std::str::FromStr for DiscussionAnsweredAnswerAuthorAssociation {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "COLLABORATOR" => Ok(Self::Collaborator),
            "CONTRIBUTOR" => Ok(Self::Contributor),
            "FIRST_TIMER" => Ok(Self::FirstTimer),
            "FIRST_TIME_CONTRIBUTOR" => Ok(Self::FirstTimeContributor),
            "MANNEQUIN" => Ok(Self::Mannequin),
            "MEMBER" => Ok(Self::Member),
            "NONE" => Ok(Self::None),
            "OWNER" => Ok(Self::Owner),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DiscussionAnsweredAnswerAuthorAssociation {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DiscussionAnsweredAnswerAuthorAssociation {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DiscussionAnsweredAnswerAuthorAssociation {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionAnsweredAnswerReactions {
    pub confused: i64,
    pub eyes: i64,
    pub heart: i64,
    pub hooray: i64,
    pub laugh: i64,
    #[serde(rename = "-1")]
    pub minus1: i64,
    #[serde(rename = "+1")]
    pub plus1: i64,
    pub rocket: i64,
    pub total_count: i64,
    pub url: String,
}
impl From<&DiscussionAnsweredAnswerReactions> for DiscussionAnsweredAnswerReactions {
    fn from(value: &DiscussionAnsweredAnswerReactions) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionAnsweredDiscussion {
    pub active_lock_reason: Option<String>,
    pub answer_chosen_at: Option<String>,
    pub answer_chosen_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub answer_html_url: Option<String>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: String,
    pub category: DiscussionAnsweredDiscussionCategory,
    pub comments: i64,
    pub created_at: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<DiscussionAnsweredDiscussionLabels>,
    pub locked: bool,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<DiscussionAnsweredAnswerReactions>,
    pub repository_url: String,
    pub state: DiscussionAnsweredDiscussionState,
    pub state_reason: Option<DiscussionAnsweredDiscussionStateReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&DiscussionAnsweredDiscussion> for DiscussionAnsweredDiscussion {
    fn from(value: &DiscussionAnsweredDiscussion) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionAnsweredDiscussionCategory {
    pub created_at: String,
    pub description: String,
    pub emoji: String,
    pub id: i64,
    pub is_answerable: bool,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    pub repository_id: i64,
    pub slug: String,
    pub updated_at: String,
}
impl From<&DiscussionAnsweredDiscussionCategory> for DiscussionAnsweredDiscussionCategory {
    fn from(value: &DiscussionAnsweredDiscussionCategory) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionAnsweredDiscussionLabels {
    pub color: String,
    pub default: bool,
    pub description: Option<String>,
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub url: String,
}
impl From<&DiscussionAnsweredDiscussionLabels> for DiscussionAnsweredDiscussionLabels {
    fn from(value: &DiscussionAnsweredDiscussionLabels) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiscussionAnsweredDiscussionState {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "converting")]
    Converting,
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "transferring")]
    Transferring,
}
impl From<&DiscussionAnsweredDiscussionState> for DiscussionAnsweredDiscussionState {
    fn from(value: &DiscussionAnsweredDiscussionState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DiscussionAnsweredDiscussionState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Closed => write!(f, "closed"),
            Self::Converting => write!(f, "converting"),
            Self::Locked => write!(f, "locked"),
            Self::Open => write!(f, "open"),
            Self::Transferring => write!(f, "transferring"),
        }
    }
}
impl std::str::FromStr for DiscussionAnsweredDiscussionState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "closed" => Ok(Self::Closed),
            "converting" => Ok(Self::Converting),
            "locked" => Ok(Self::Locked),
            "open" => Ok(Self::Open),
            "transferring" => Ok(Self::Transferring),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DiscussionAnsweredDiscussionState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DiscussionAnsweredDiscussionState {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DiscussionAnsweredDiscussionState {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiscussionAnsweredDiscussionStateReason {
    #[serde(rename = "duplicate")]
    Duplicate,
    #[serde(rename = "outdated")]
    Outdated,
    #[serde(rename = "reopened")]
    Reopened,
    #[serde(rename = "resolved")]
    Resolved,
}
impl From<&DiscussionAnsweredDiscussionStateReason> for DiscussionAnsweredDiscussionStateReason {
    fn from(value: &DiscussionAnsweredDiscussionStateReason) -> Self {
        *value
    }
}
impl ::std::fmt::Display for DiscussionAnsweredDiscussionStateReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Duplicate => write!(f, "duplicate"),
            Self::Outdated => write!(f, "outdated"),
            Self::Reopened => write!(f, "reopened"),
            Self::Resolved => write!(f, "resolved"),
        }
    }
}
impl std::str::FromStr for DiscussionAnsweredDiscussionStateReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "duplicate" => Ok(Self::Duplicate),
            "outdated" => Ok(Self::Outdated),
            "reopened" => Ok(Self::Reopened),
            "resolved" => Ok(Self::Resolved),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for DiscussionAnsweredDiscussionStateReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DiscussionAnsweredDiscussionStateReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DiscussionAnsweredDiscussionStateReason {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionCategoryChangedChanges {
    pub category: DiscussionCategoryChangedChangesCategory,
}
impl From<&DiscussionCategoryChangedChanges> for DiscussionCategoryChangedChanges {
    fn from(value: &DiscussionCategoryChangedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionCategoryChangedChangesCategory {
    pub from: DiscussionAnsweredDiscussionCategory,
}
impl From<&DiscussionCategoryChangedChangesCategory> for DiscussionCategoryChangedChangesCategory {
    fn from(value: &DiscussionCategoryChangedChangesCategory) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum DiscussionComment {
    #[serde(rename = "created")]
    Created {
        comment: DiscussionCommentCreatedComment,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        comment: DiscussionCommentCreatedComment,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: IssueCommentEditedChanges,
        comment: DiscussionCommentCreatedComment,
        discussion: DiscussionAnsweredDiscussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&DiscussionComment> for DiscussionComment {
    fn from(value: &DiscussionComment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionCommentCreatedComment {
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: String,
    pub child_comment_count: i64,
    pub created_at: String,
    pub discussion_id: i64,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub parent_id: Option<i64>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub repository_url: String,
    pub updated_at: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&DiscussionCommentCreatedComment> for DiscussionCommentCreatedComment {
    fn from(value: &DiscussionCommentCreatedComment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<DiscussionEditedChangesBody>,
}
impl From<&DiscussionEditedChanges> for DiscussionEditedChanges {
    fn from(value: &DiscussionEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionEditedChangesBody {
    pub from: String,
}
impl From<&DiscussionEditedChangesBody> for DiscussionEditedChangesBody {
    fn from(value: &DiscussionEditedChangesBody) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionLabeledLabel {
    pub color: String,
    pub default: bool,
    pub description: Option<String>,
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub url: String,
}
impl From<&DiscussionLabeledLabel> for DiscussionLabeledLabel {
    fn from(value: &DiscussionLabeledLabel) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionTransferredChanges {
    pub new_discussion: DiscussionAnsweredDiscussion,
    pub new_repository: DiscussionTransferredChangesNewRepository,
}
impl From<&DiscussionTransferredChanges> for DiscussionTransferredChanges {
    fn from(value: &DiscussionTransferredChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionTransferredChangesNewRepository {
    #[serde(default)]
    pub allow_auto_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_merge_commit: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_rebase_merge: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_squash_merge: bool,
    #[serde(default)]
    pub allow_update_branch: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anonymous_access_enabled: Option<bool>,
    pub archive_url: String,
    pub archived: bool,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<Untyped>,
    pub default_branch: String,
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deployments_url: String,
    pub description: Option<String>,
    pub disabled: bool,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    #[serde(default)]
    pub has_discussions: bool,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default)]
    pub is_template: bool,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub license: DiscussionTransferredChangesNewRepositoryLicense,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle>,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i64>,
    pub node_id: String,
    pub notifications_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<DiscussionTransferredChangesNewRepositoryOrganization>,
    pub owner: DiscussionTransferredChangesNewRepositoryOrganization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DiscussionTransferredChangesNewRepositoryPermissions>,
    pub private: bool,
    pub pulls_url: String,
    pub pushed_at: Option<String>,
    pub releases_url: String,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle>,
    pub ssh_url: String,
    pub stargazers_count: i64,
    pub stargazers_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_at: Option<String>,
    pub statuses_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i64>,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temp_clone_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_repository: Option<DiscussionTransferredChangesNewRepositoryTemplateRepository>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    pub trees_url: String,
    pub updated_at: Option<String>,
    pub url: String,
    #[serde(default)]
    pub use_squash_pr_title_as_default: bool,
    #[serde(default = "defaults::discussion_transferred_changes_new_repository_visibility")]
    pub visibility: String,
    pub watchers: i64,
    pub watchers_count: i64,
    #[serde(default)]
    pub web_commit_signoff_required: bool,
}
impl From<&DiscussionTransferredChangesNewRepository>
    for DiscussionTransferredChangesNewRepository
{
    fn from(value: &DiscussionTransferredChangesNewRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionTransferredChangesNewRepositoryLicense {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub key: String,
    pub name: String,
    pub node_id: String,
    pub spdx_id: Option<String>,
    pub url: Option<String>,
}
impl From<&DiscussionTransferredChangesNewRepositoryLicense>
    for DiscussionTransferredChangesNewRepositoryLicense
{
    fn from(value: &DiscussionTransferredChangesNewRepositoryLicense) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionTransferredChangesNewRepositoryOrganization {
    pub avatar_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub node_id: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_at: Option<String>,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_view_type: Option<String>,
}
impl From<&DiscussionTransferredChangesNewRepositoryOrganization>
    for DiscussionTransferredChangesNewRepositoryOrganization
{
    fn from(value: &DiscussionTransferredChangesNewRepositoryOrganization) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionTransferredChangesNewRepositoryPermissions {
    pub admin: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintain: Option<bool>,
    pub pull: bool,
    pub push: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triage: Option<bool>,
}
impl From<&DiscussionTransferredChangesNewRepositoryPermissions>
    for DiscussionTransferredChangesNewRepositoryPermissions
{
    fn from(value: &DiscussionTransferredChangesNewRepositoryPermissions) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionTransferredChangesNewRepositoryTemplateRepository {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_auto_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignees_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blobs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branches_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clone_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collaborators_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributors_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downloads_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fork: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forks_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_commits_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_refs_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_tags_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_downloads: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_issues: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_pages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_comment_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub languages_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestones_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_issues_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryOwner>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pulls_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pushed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub releases_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscribers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svn_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teams_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temp_clone_token: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub topics: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trees_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_squash_pr_title_as_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watchers_count: Option<i64>,
}
impl From<&DiscussionTransferredChangesNewRepositoryTemplateRepository>
    for DiscussionTransferredChangesNewRepositoryTemplateRepository
{
    fn from(value: &DiscussionTransferredChangesNewRepositoryTemplateRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage {
    #[serde(rename = "BLANK")]
    Blank,
    #[serde(rename = "PR_BODY")]
    PrBody,
    #[serde(rename = "PR_TITLE")]
    PrTitle,
}
impl From<&DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage
{
    fn from(
        value: &DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage,
    ) -> Self {
        *value
    }
}
impl ::std::fmt::Display
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Blank => write!(f, "BLANK"),
            Self::PrBody => write!(f, "PR_BODY"),
            Self::PrTitle => write!(f, "PR_TITLE"),
        }
    }
}
impl std::str::FromStr
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "BLANK" => Ok(Self::Blank),
            "PR_BODY" => Ok(Self::PrBody),
            "PR_TITLE" => Ok(Self::PrTitle),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle {
    #[serde(rename = "MERGE_MESSAGE")]
    MergeMessage,
    #[serde(rename = "PR_TITLE")]
    PrTitle,
}
impl From<&DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle
{
    fn from(
        value: &DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle,
    ) -> Self {
        *value
    }
}
impl ::std::fmt::Display
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MergeMessage => write!(f, "MERGE_MESSAGE"),
            Self::PrTitle => write!(f, "PR_TITLE"),
        }
    }
}
impl std::str::FromStr
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "MERGE_MESSAGE" => Ok(Self::MergeMessage),
            "PR_TITLE" => Ok(Self::PrTitle),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionTransferredChangesNewRepositoryTemplateRepositoryOwner {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&DiscussionTransferredChangesNewRepositoryTemplateRepositoryOwner>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryOwner
{
    fn from(value: &DiscussionTransferredChangesNewRepositoryTemplateRepositoryOwner) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionTransferredChangesNewRepositoryTemplateRepositoryPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triage: Option<bool>,
}
impl From<&DiscussionTransferredChangesNewRepositoryTemplateRepositoryPermissions>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositoryPermissions
{
    fn from(
        value: &DiscussionTransferredChangesNewRepositoryTemplateRepositoryPermissions,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage {
    #[serde(rename = "BLANK")]
    Blank,
    #[serde(rename = "COMMIT_MESSAGES")]
    CommitMessages,
    #[serde(rename = "PR_BODY")]
    PrBody,
}
impl From<&DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage
{
    fn from(
        value: &DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage,
    ) -> Self {
        *value
    }
}
impl ::std::fmt::Display
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Blank => write!(f, "BLANK"),
            Self::CommitMessages => write!(f, "COMMIT_MESSAGES"),
            Self::PrBody => write!(f, "PR_BODY"),
        }
    }
}
impl std::str::FromStr
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "BLANK" => Ok(Self::Blank),
            "COMMIT_MESSAGES" => Ok(Self::CommitMessages),
            "PR_BODY" => Ok(Self::PrBody),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle {
    #[serde(rename = "COMMIT_OR_PR_TITLE")]
    CommitOrPrTitle,
    #[serde(rename = "PR_TITLE")]
    PrTitle,
}
impl From<&DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle
{
    fn from(
        value: &DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle,
    ) -> Self {
        *value
    }
}
impl ::std::fmt::Display
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CommitOrPrTitle => write!(f, "COMMIT_OR_PR_TITLE"),
            Self::PrTitle => write!(f, "PR_TITLE"),
        }
    }
}
impl std::str::FromStr
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "COMMIT_OR_PR_TITLE" => Ok(Self::CommitOrPrTitle),
            "PR_TITLE" => Ok(Self::PrTitle),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct DiscussionUnanswered {
    pub action: String,
    pub discussion: DiscussionAnsweredDiscussion,
    pub old_answer: DiscussionAnsweredAnswer,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub repository: Untyped,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<Untyped>,
}
impl From<&DiscussionUnanswered> for DiscussionUnanswered {
    fn from(value: &DiscussionUnanswered) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Fork {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    pub forkee: ForkDefaultForkee,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub repository: Untyped,
    pub sender: Untyped,
}
impl From<&Fork> for Fork {
    fn from(value: &Fork) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ForkDefaultForkee {
    #[serde(default)]
    pub allow_auto_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_merge_commit: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_rebase_merge: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_squash_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    pub archive_url: String,
    pub archived: bool,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: ForkDefaultForkeeCreatedAt,
    pub default_branch: String,
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deployments_url: String,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub license: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoLicense>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoPermissions>,
    pub private: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    pub pulls_url: String,
    pub pushed_at: Option<ForkDefaultForkeePushedAt>,
    pub releases_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    pub size: i64,
    pub ssh_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers: Option<i64>,
    pub stargazers_count: i64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub topics: Vec<String>,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    pub visibility: PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility,
    pub watchers: i64,
    pub watchers_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_commit_signoff_required: Option<bool>,
}
impl From<&ForkDefaultForkee> for ForkDefaultForkee {
    fn from(value: &ForkDefaultForkee) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ForkDefaultForkeeCreatedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&ForkDefaultForkeeCreatedAt> for ForkDefaultForkeeCreatedAt {
    fn from(value: &ForkDefaultForkeeCreatedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ForkDefaultForkeeCreatedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for ForkDefaultForkeeCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ForkDefaultForkeeCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ForkDefaultForkeeCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for ForkDefaultForkeeCreatedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for ForkDefaultForkeeCreatedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ForkDefaultForkeePushedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&ForkDefaultForkeePushedAt> for ForkDefaultForkeePushedAt {
    fn from(value: &ForkDefaultForkeePushedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ForkDefaultForkeePushedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for ForkDefaultForkeePushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ForkDefaultForkeePushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ForkDefaultForkeePushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for ForkDefaultForkeePushedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for ForkDefaultForkeePushedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct GithubAppAuthorization(pub GithubAppAuthorizationRevoked);
impl ::std::ops::Deref for GithubAppAuthorization {
    type Target = GithubAppAuthorizationRevoked;
    fn deref(&self) -> &GithubAppAuthorizationRevoked {
        &self.0
    }
}
impl From<GithubAppAuthorization> for GithubAppAuthorizationRevoked {
    fn from(value: GithubAppAuthorization) -> Self {
        value.0
    }
}
impl From<&GithubAppAuthorization> for GithubAppAuthorization {
    fn from(value: &GithubAppAuthorization) -> Self {
        value.clone()
    }
}
impl From<GithubAppAuthorizationRevoked> for GithubAppAuthorization {
    fn from(value: GithubAppAuthorizationRevoked) -> Self {
        Self(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct GithubAppAuthorizationRevoked {
    pub action: String,
    pub sender: Untyped,
}
impl From<&GithubAppAuthorizationRevoked> for GithubAppAuthorizationRevoked {
    fn from(value: &GithubAppAuthorizationRevoked) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Gollum {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub pages: Vec<GollumDefaultPages>,
    pub repository: Untyped,
    pub sender: Untyped,
}
impl From<&Gollum> for Gollum {
    fn from(value: &Gollum) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct GollumDefaultPages {
    pub action: GollumDefaultPagesAction,
    pub html_url: String,
    pub page_name: String,
    pub sha: String,
    pub summary: Option<String>,
    pub title: String,
}
impl From<&GollumDefaultPages> for GollumDefaultPages {
    fn from(value: &GollumDefaultPages) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GollumDefaultPagesAction {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "edited")]
    Edited,
}
impl From<&GollumDefaultPagesAction> for GollumDefaultPagesAction {
    fn from(value: &GollumDefaultPagesAction) -> Self {
        *value
    }
}
impl ::std::fmt::Display for GollumDefaultPagesAction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Created => write!(f, "created"),
            Self::Edited => write!(f, "edited"),
        }
    }
}
impl std::str::FromStr for GollumDefaultPagesAction {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "created" => Ok(Self::Created),
            "edited" => Ok(Self::Edited),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for GollumDefaultPagesAction {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for GollumDefaultPagesAction {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for GollumDefaultPagesAction {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Installation {
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        installation: InstallationAttribute,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        repositories: Vec<InstallationCreatedRepositories>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        requester: Option<Box<DeploymentStatusCreatedDeploymentCreator>>,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        installation: InstallationAttribute,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        repositories: Vec<InstallationCreatedRepositories>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default)]
        requester: (),
        sender: Untyped,
    },
    #[serde(rename = "new_permissions_accepted")]
    NewPermissionsAccepted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        installation: InstallationAttribute,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        repositories: Vec<InstallationCreatedRepositories>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default)]
        requester: (),
        sender: Untyped,
    },
    #[serde(rename = "suspend")]
    Suspend {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        installation: InstallationAttribute,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        repositories: Vec<InstallationCreatedRepositories>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default)]
        requester: (),
        sender: Untyped,
    },
    #[serde(rename = "unsuspend")]
    Unsuspend {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        installation: InstallationAttribute,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        repositories: Vec<InstallationCreatedRepositories>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default)]
        requester: (),
        sender: Untyped,
    },
}
impl From<&Installation> for Installation {
    fn from(value: &Installation) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct InstallationAttribute {
    pub id: i64,
}
impl From<&InstallationAttribute> for InstallationAttribute {
    fn from(value: &InstallationAttribute) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct InstallationCreatedRepositories {
    pub full_name: String,
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub private: bool,
}
impl From<&InstallationCreatedRepositories> for InstallationCreatedRepositories {
    fn from(value: &InstallationCreatedRepositories) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum InstallationRepositories {
    #[serde(rename = "added")]
    Added {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        installation: InstallationAttribute,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repositories_added: Vec<InstallationCreatedRepositories>,
        repositories_removed: Vec<InstallationCreatedRepositories>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        repository_selection: InstallationRepositoriesAddedRepositorySelection,
        requester: Option<DeploymentStatusCreatedDeploymentCreator>,
        sender: Untyped,
    },
    #[serde(rename = "removed")]
    Removed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        installation: InstallationAttribute,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repositories_added: Vec<InstallationCreatedRepositories>,
        repositories_removed: Vec<InstallationCreatedRepositories>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        repository_selection: InstallationRepositoriesAddedRepositorySelection,
        requester: Option<DeploymentStatusCreatedDeploymentCreator>,
        sender: Untyped,
    },
}
impl From<&InstallationRepositories> for InstallationRepositories {
    fn from(value: &InstallationRepositories) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum InstallationRepositoriesAddedRepositorySelection {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "selected")]
    Selected,
}
impl From<&InstallationRepositoriesAddedRepositorySelection>
    for InstallationRepositoriesAddedRepositorySelection
{
    fn from(value: &InstallationRepositoriesAddedRepositorySelection) -> Self {
        *value
    }
}
impl ::std::fmt::Display for InstallationRepositoriesAddedRepositorySelection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => write!(f, "all"),
            Self::Selected => write!(f, "selected"),
        }
    }
}
impl std::str::FromStr for InstallationRepositoriesAddedRepositorySelection {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "all" => Ok(Self::All),
            "selected" => Ok(Self::Selected),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for InstallationRepositoriesAddedRepositorySelection {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InstallationRepositoriesAddedRepositorySelection {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InstallationRepositoriesAddedRepositorySelection {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum InstallationTarget {
    #[serde(rename = "renamed")]
    Renamed {
        account: InstallationTargetRenamedAccount,
        changes: InstallationTargetRenamedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        installation: InstallationAttribute,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
        target_type: String,
    },
}
impl From<&InstallationTarget> for InstallationTarget {
    fn from(value: &InstallationTarget) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct InstallationTargetRenamedAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    pub avatar_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    pub description: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_organization_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_repository_projects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks_url: Option<String>,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_gists: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_members_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_repos: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_view_type: Option<String>,
    #[serde(default)]
    pub website_url: (),
}
impl From<&InstallationTargetRenamedAccount> for InstallationTargetRenamedAccount {
    fn from(value: &InstallationTargetRenamedAccount) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct InstallationTargetRenamedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<DiscussionEditedChangesBody>,
}
impl From<&InstallationTargetRenamedChanges> for InstallationTargetRenamedChanges {
    fn from(value: &InstallationTargetRenamedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum IssueComment {
    #[serde(rename = "created")]
    Created {
        comment: IssueCommentCreatedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssueCommentCreatedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        comment: IssueCommentDeletedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssueCommentCreatedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: IssueCommentEditedChanges,
        comment: IssueCommentDeletedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssueCommentCreatedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&IssueComment> for IssueComment {
    fn from(value: &IssueComment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssueCommentCreatedComment {
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: String,
    pub created_at: String,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub node_id: String,
    pub performed_via_github_app: (),
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&IssueCommentCreatedComment> for IssueCommentCreatedComment {
    fn from(value: &IssueCommentCreatedComment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssueCommentCreatedIssue {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    pub assignee: Option<MilestoneClosedMilestoneCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    pub labels: Vec<DiscussionLabeledLabel>,
    pub labels_url: String,
    pub locked: bool,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<IssueCommentCreatedIssuePullRequest>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub repository_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&IssueCommentCreatedIssue> for IssueCommentCreatedIssue {
    fn from(value: &IssueCommentCreatedIssue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IssueCommentCreatedIssueActiveLockReason {
    #[serde(rename = "off-topic")]
    OffTopic,
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "spam")]
    Spam,
    #[serde(rename = "too heated")]
    TooHeated,
}
impl From<&IssueCommentCreatedIssueActiveLockReason> for IssueCommentCreatedIssueActiveLockReason {
    fn from(value: &IssueCommentCreatedIssueActiveLockReason) -> Self {
        *value
    }
}
impl ::std::fmt::Display for IssueCommentCreatedIssueActiveLockReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::OffTopic => write!(f, "off-topic"),
            Self::Resolved => write!(f, "resolved"),
            Self::Spam => write!(f, "spam"),
            Self::TooHeated => write!(f, "too heated"),
        }
    }
}
impl std::str::FromStr for IssueCommentCreatedIssueActiveLockReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "off-topic" => Ok(Self::OffTopic),
            "resolved" => Ok(Self::Resolved),
            "spam" => Ok(Self::Spam),
            "too heated" => Ok(Self::TooHeated),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for IssueCommentCreatedIssueActiveLockReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IssueCommentCreatedIssueActiveLockReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IssueCommentCreatedIssueActiveLockReason {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssueCommentCreatedIssuePullRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&IssueCommentCreatedIssuePullRequest> for IssueCommentCreatedIssuePullRequest {
    fn from(value: &IssueCommentCreatedIssuePullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssueCommentDeletedComment {
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: String,
    pub created_at: String,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub node_id: String,
    pub performed_via_github_app: Option<IssueCommentDeletedCommentPerformedViaGithubApp>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&IssueCommentDeletedComment> for IssueCommentDeletedComment {
    fn from(value: &IssueCommentDeletedComment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssueCommentDeletedCommentPerformedViaGithubApp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    pub created_at: String,
    pub description: Option<String>,
    pub events: Vec<String>,
    pub external_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installations_count: Option<i64>,
    pub name: String,
    pub node_id: String,
    pub owner: DiscussionTransferredChangesNewRepositoryOrganization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pem: Option<String>,
    pub permissions: IssueCommentDeletedCommentPerformedViaGithubAppPermissions,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
}
impl From<&IssueCommentDeletedCommentPerformedViaGithubApp>
    for IssueCommentDeletedCommentPerformedViaGithubApp
{
    fn from(value: &IssueCommentDeletedCommentPerformedViaGithubApp) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssueCommentDeletedCommentPerformedViaGithubAppPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
}
impl From<&IssueCommentDeletedCommentPerformedViaGithubAppPermissions>
    for IssueCommentDeletedCommentPerformedViaGithubAppPermissions
{
    fn from(value: &IssueCommentDeletedCommentPerformedViaGithubAppPermissions) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssueCommentEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<DiscussionEditedChangesBody>,
}
impl From<&IssueCommentEditedChanges> for IssueCommentEditedChanges {
    fn from(value: &IssueCommentEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Issues {
    #[serde(rename = "assigned")]
    Assigned {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesAssignedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "closed")]
    Closed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesClosedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesDeletedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "demilestoned")]
    Demilestoned {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesDemilestonedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        milestone: Option<MilestoneClosedMilestone>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: DiscussionEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesAssignedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<DiscussionLabeledLabel>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "labeled")]
    Labeled {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesAssignedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<DiscussionLabeledLabel>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "locked")]
    Locked {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesLockedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "milestoned")]
    Milestoned {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesMilestonedIssue,
        milestone: MilestoneClosedMilestone,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "opened")]
    Opened {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        changes: Option<IssuesOpenedChanges>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesOpenedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "pinned")]
    Pinned {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesDeletedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "reopened")]
    Reopened {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesReopenedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "transferred")]
    Transferred {
        changes: IssuesTransferredChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesDeletedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "unassigned")]
    Unassigned {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        assignee: Option<MilestoneClosedMilestoneCreator>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesAssignedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "unlabeled")]
    Unlabeled {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesAssignedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<DiscussionLabeledLabel>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "unlocked")]
    Unlocked {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesUnlockedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "unpinned")]
    Unpinned {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        issue: IssuesDeletedIssue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&Issues> for Issues {
    fn from(value: &Issues) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesAssignedIssue {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<MilestoneClosedMilestoneCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<DiscussionLabeledLabel>,
    pub labels_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<IssuesAssignedIssuePerformedViaGithubApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<IssueCommentCreatedIssuePullRequest>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub repository_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DeploymentProtectionRuleRequestedPullRequestsState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&IssuesAssignedIssue> for IssuesAssignedIssue {
    fn from(value: &IssuesAssignedIssue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesAssignedIssuePerformedViaGithubApp {
    pub created_at: Option<String>,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    pub external_url: Option<String>,
    pub html_url: String,
    pub id: Option<i64>,
    pub name: String,
    pub node_id: String,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<IssuesAssignedIssuePerformedViaGithubAppPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub updated_at: Option<String>,
}
impl From<&IssuesAssignedIssuePerformedViaGithubApp> for IssuesAssignedIssuePerformedViaGithubApp {
    fn from(value: &IssuesAssignedIssuePerformedViaGithubApp) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesAssignedIssuePerformedViaGithubAppPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administration:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_references:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discussions:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environments:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_administration:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_hooks:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_packages:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_plan:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_projects: Option<MemberAddedChangesPermissionTo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_secrets:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_self_hosted_runners:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_user_blocking:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_requests:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_hooks:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_projects:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning_alerts:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_events:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_scanning_alert:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_file:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_discussions:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vulnerability_alerts:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
}
impl From<&IssuesAssignedIssuePerformedViaGithubAppPermissions>
    for IssuesAssignedIssuePerformedViaGithubAppPermissions
{
    fn from(value: &IssuesAssignedIssuePerformedViaGithubAppPermissions) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesClosedIssue {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Untyped>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Option<Untyped>>,
    pub labels_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<IssueCommentCreatedIssuePullRequest>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub repository_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&IssuesClosedIssue> for IssuesClosedIssue {
    fn from(value: &IssuesClosedIssue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesDeletedIssue {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<DiscussionLabeledLabel>,
    pub labels_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    pub milestone: Option<MilestoneCreatedMilestone>,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<IssueCommentCreatedIssuePullRequest>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub repository_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DeploymentProtectionRuleRequestedPullRequestsState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&IssuesDeletedIssue> for IssuesDeletedIssue {
    fn from(value: &IssuesDeletedIssue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesDemilestonedIssue {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<IssuesDemilestonedIssueAssignee>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Option<Untyped>>,
    pub labels_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<IssuesAssignedIssuePerformedViaGithubApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<IssueCommentCreatedIssuePullRequest>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub repository_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DeploymentProtectionRuleRequestedPullRequestsState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&IssuesDemilestonedIssue> for IssuesDemilestonedIssue {
    fn from(value: &IssuesDemilestonedIssue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesDemilestonedIssueAssignee {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub id: i64,
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<MilestoneClosedMilestoneCreatorType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&IssuesDemilestonedIssueAssignee> for IssuesDemilestonedIssueAssignee {
    fn from(value: &IssuesDemilestonedIssueAssignee) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesLockedIssue {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Option<Untyped>>,
    pub labels_url: String,
    pub locked: bool,
    pub milestone: Option<MilestoneCreatedMilestone>,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<IssueCommentCreatedIssuePullRequest>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub repository_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DeploymentProtectionRuleRequestedPullRequestsState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&IssuesLockedIssue> for IssuesLockedIssue {
    fn from(value: &IssuesLockedIssue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesMilestonedIssue {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<DeploymentReviewApprovedReviewersReviewer>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Option<Untyped>>,
    pub labels_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<IssuesAssignedIssuePerformedViaGithubApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<IssueCommentCreatedIssuePullRequest>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub repository_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DeploymentProtectionRuleRequestedPullRequestsState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&IssuesMilestonedIssue> for IssuesMilestonedIssue {
    fn from(value: &IssuesMilestonedIssue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesOpenedChanges {
    pub old_issue: Option<IssuesDeletedIssue>,
    pub old_repository: IssuesOpenedChangesOldRepository,
}
impl From<&IssuesOpenedChanges> for IssuesOpenedChanges {
    fn from(value: &IssuesOpenedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesOpenedChangesOldRepository {
    #[serde(default)]
    pub allow_auto_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_merge_commit: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_rebase_merge: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_squash_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    pub archive_url: String,
    pub archived: bool,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: IssuesOpenedChangesOldRepositoryCreatedAt,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<Untyped>,
    pub default_branch: String,
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deployments_url: String,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_discussions: Option<bool>,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub license: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoLicense>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoPermissions>,
    pub private: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    pub pulls_url: String,
    pub pushed_at: Option<IssuesOpenedChangesOldRepositoryPushedAt>,
    pub releases_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    pub size: i64,
    pub ssh_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers: Option<i64>,
    pub stargazers_count: i64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub topics: Vec<String>,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    pub visibility: PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility,
    pub watchers: i64,
    pub watchers_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_commit_signoff_required: Option<bool>,
}
impl From<&IssuesOpenedChangesOldRepository> for IssuesOpenedChangesOldRepository {
    fn from(value: &IssuesOpenedChangesOldRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum IssuesOpenedChangesOldRepositoryCreatedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&IssuesOpenedChangesOldRepositoryCreatedAt>
    for IssuesOpenedChangesOldRepositoryCreatedAt
{
    fn from(value: &IssuesOpenedChangesOldRepositoryCreatedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IssuesOpenedChangesOldRepositoryCreatedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for IssuesOpenedChangesOldRepositoryCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IssuesOpenedChangesOldRepositoryCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IssuesOpenedChangesOldRepositoryCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for IssuesOpenedChangesOldRepositoryCreatedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for IssuesOpenedChangesOldRepositoryCreatedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum IssuesOpenedChangesOldRepositoryPushedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&IssuesOpenedChangesOldRepositoryPushedAt> for IssuesOpenedChangesOldRepositoryPushedAt {
    fn from(value: &IssuesOpenedChangesOldRepositoryPushedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IssuesOpenedChangesOldRepositoryPushedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for IssuesOpenedChangesOldRepositoryPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IssuesOpenedChangesOldRepositoryPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IssuesOpenedChangesOldRepositoryPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for IssuesOpenedChangesOldRepositoryPushedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for IssuesOpenedChangesOldRepositoryPushedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesOpenedIssue {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<DiscussionLabeledLabel>,
    pub labels_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<IssuesAssignedIssuePerformedViaGithubApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<IssueCommentCreatedIssuePullRequest>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub repository_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DeploymentProtectionRuleRequestedPullRequestsState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&IssuesOpenedIssue> for IssuesOpenedIssue {
    fn from(value: &IssuesOpenedIssue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesReopenedIssue {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<DeploymentReviewApprovedReviewersReviewer>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Option<Untyped>>,
    pub labels_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<IssuesReopenedIssuePerformedViaGithubApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<IssueCommentCreatedIssuePullRequest>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub repository_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&IssuesReopenedIssue> for IssuesReopenedIssue {
    fn from(value: &IssuesReopenedIssue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesReopenedIssuePerformedViaGithubApp {
    pub created_at: Option<String>,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    pub external_url: Option<String>,
    pub html_url: String,
    pub id: Option<i64>,
    pub name: String,
    pub node_id: String,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<IssuesReopenedIssuePerformedViaGithubAppPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub updated_at: Option<String>,
}
impl From<&IssuesReopenedIssuePerformedViaGithubApp> for IssuesReopenedIssuePerformedViaGithubApp {
    fn from(value: &IssuesReopenedIssuePerformedViaGithubApp) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesReopenedIssuePerformedViaGithubAppPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administration:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_references:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discussions:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environments:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keys: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_administration:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_hooks:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_packages:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_plan:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_projects: Option<MemberAddedChangesPermissionTo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_secrets:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_self_hosted_runners:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_user_blocking:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_requests:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_hooks:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_projects: Option<MemberAddedChangesPermissionTo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_scanning_alerts:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_events:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_scanning_alert:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_file:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_discussions:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vulnerability_alerts:
        Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions>,
}
impl From<&IssuesReopenedIssuePerformedViaGithubAppPermissions>
    for IssuesReopenedIssuePerformedViaGithubAppPermissions
{
    fn from(value: &IssuesReopenedIssuePerformedViaGithubAppPermissions) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesTransferredChanges {
    pub new_issue: IssuesDeletedIssue,
    pub new_repository: IssuesTransferredChangesNewRepository,
}
impl From<&IssuesTransferredChanges> for IssuesTransferredChanges {
    fn from(value: &IssuesTransferredChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesTransferredChangesNewRepository {
    #[serde(default)]
    pub allow_auto_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_merge_commit: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_rebase_merge: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_squash_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    pub archive_url: String,
    pub archived: bool,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: IssuesTransferredChangesNewRepositoryCreatedAt,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<Untyped>,
    pub default_branch: String,
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deployments_url: String,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub has_discussions: bool,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub license: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoLicense>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoPermissions>,
    pub private: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    pub pulls_url: String,
    pub pushed_at: Option<IssuesTransferredChangesNewRepositoryPushedAt>,
    pub releases_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    pub size: i64,
    pub ssh_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers: Option<i64>,
    pub stargazers_count: i64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub topics: Vec<String>,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    pub visibility: PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility,
    pub watchers: i64,
    pub watchers_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_commit_signoff_required: Option<bool>,
}
impl From<&IssuesTransferredChangesNewRepository> for IssuesTransferredChangesNewRepository {
    fn from(value: &IssuesTransferredChangesNewRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum IssuesTransferredChangesNewRepositoryCreatedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&IssuesTransferredChangesNewRepositoryCreatedAt>
    for IssuesTransferredChangesNewRepositoryCreatedAt
{
    fn from(value: &IssuesTransferredChangesNewRepositoryCreatedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IssuesTransferredChangesNewRepositoryCreatedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for IssuesTransferredChangesNewRepositoryCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IssuesTransferredChangesNewRepositoryCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IssuesTransferredChangesNewRepositoryCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for IssuesTransferredChangesNewRepositoryCreatedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for IssuesTransferredChangesNewRepositoryCreatedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum IssuesTransferredChangesNewRepositoryPushedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&IssuesTransferredChangesNewRepositoryPushedAt>
    for IssuesTransferredChangesNewRepositoryPushedAt
{
    fn from(value: &IssuesTransferredChangesNewRepositoryPushedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for IssuesTransferredChangesNewRepositoryPushedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for IssuesTransferredChangesNewRepositoryPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IssuesTransferredChangesNewRepositoryPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IssuesTransferredChangesNewRepositoryPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for IssuesTransferredChangesNewRepositoryPushedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for IssuesTransferredChangesNewRepositoryPushedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct IssuesUnlockedIssue {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Option<Untyped>>,
    pub labels_url: String,
    pub locked: bool,
    pub milestone: Option<MilestoneCreatedMilestone>,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<DeploymentStatusCreatedDeploymentPerformedViaGithubApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<IssueCommentCreatedIssuePullRequest>,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub repository_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DeploymentProtectionRuleRequestedPullRequestsState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&IssuesUnlockedIssue> for IssuesUnlockedIssue {
    fn from(value: &IssuesUnlockedIssue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Label {
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        label: DiscussionLabeledLabel,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        label: DiscussionLabeledLabel,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        changes: Option<LabelEditedChanges>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        label: DiscussionLabeledLabel,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&Label> for Label {
    fn from(value: &Label) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct LabelEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<DiscussionEditedChangesBody>,
}
impl From<&LabelEditedChanges> for LabelEditedChanges {
    fn from(value: &LabelEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Member {
    #[serde(rename = "added")]
    Added {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        changes: Option<MemberAddedChanges>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        member: Option<DeploymentStatusCreatedDeploymentCreator>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: MemberEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        member: Option<DeploymentStatusCreatedDeploymentCreator>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "removed")]
    Removed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        member: Option<DeploymentStatusCreatedDeploymentCreator>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&Member> for Member {
    fn from(value: &Member) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MemberAddedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<MemberAddedChangesPermission>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<MemberAddedChangesRoleName>,
}
impl From<&MemberAddedChanges> for MemberAddedChanges {
    fn from(value: &MemberAddedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MemberAddedChangesPermission {
    pub to: MemberAddedChangesPermissionTo,
}
impl From<&MemberAddedChangesPermission> for MemberAddedChangesPermission {
    fn from(value: &MemberAddedChangesPermission) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MemberAddedChangesPermissionTo {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl From<&MemberAddedChangesPermissionTo> for MemberAddedChangesPermissionTo {
    fn from(value: &MemberAddedChangesPermissionTo) -> Self {
        *value
    }
}
impl ::std::fmt::Display for MemberAddedChangesPermissionTo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Admin => write!(f, "admin"),
            Self::Read => write!(f, "read"),
            Self::Write => write!(f, "write"),
        }
    }
}
impl std::str::FromStr for MemberAddedChangesPermissionTo {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "admin" => Ok(Self::Admin),
            "read" => Ok(Self::Read),
            "write" => Ok(Self::Write),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MemberAddedChangesPermissionTo {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MemberAddedChangesPermissionTo {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MemberAddedChangesPermissionTo {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MemberAddedChangesRoleName {
    pub to: String,
}
impl From<&MemberAddedChangesRoleName> for MemberAddedChangesRoleName {
    fn from(value: &MemberAddedChangesRoleName) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MemberEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_permission: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<MemberEditedChangesPermission>,
}
impl From<&MemberEditedChanges> for MemberEditedChanges {
    fn from(value: &MemberEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MemberEditedChangesPermission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}
impl From<&MemberEditedChangesPermission> for MemberEditedChangesPermission {
    fn from(value: &MemberEditedChangesPermission) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Membership {
    #[serde(rename = "added")]
    Added {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        member: Option<DeploymentStatusCreatedDeploymentCreator>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        scope: String,
        sender: Option<Untyped>,
        team: TeamAddDefaultTeam,
    },
    #[serde(rename = "removed")]
    Removed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        member: Option<DeploymentStatusCreatedDeploymentCreator>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        scope: MembershipRemovedScope,
        sender: Option<Untyped>,
        team: TeamAddDefaultTeam,
    },
}
impl From<&Membership> for Membership {
    fn from(value: &Membership) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MembershipRemovedScope {
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "team")]
    Team,
}
impl From<&MembershipRemovedScope> for MembershipRemovedScope {
    fn from(value: &MembershipRemovedScope) -> Self {
        *value
    }
}
impl ::std::fmt::Display for MembershipRemovedScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Organization => write!(f, "organization"),
            Self::Team => write!(f, "team"),
        }
    }
}
impl std::str::FromStr for MembershipRemovedScope {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "organization" => Ok(Self::Organization),
            "team" => Ok(Self::Team),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MembershipRemovedScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MembershipRemovedScope {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MembershipRemovedScope {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum MergeGroup {
    #[serde(rename = "checks_requested")]
    ChecksRequested {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        merge_group: MergeGroupChecksRequestedMergeGroup,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "destroyed")]
    Destroyed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        merge_group: MergeGroupChecksRequestedMergeGroup,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reason: Option<MergeGroupDestroyedReason>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
}
impl From<&MergeGroup> for MergeGroup {
    fn from(value: &MergeGroup) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MergeGroupChecksRequestedMergeGroup {
    pub base_ref: String,
    pub base_sha: String,
    pub head_commit: MergeGroupChecksRequestedMergeGroupHeadCommit,
    pub head_ref: String,
    pub head_sha: String,
}
impl From<&MergeGroupChecksRequestedMergeGroup> for MergeGroupChecksRequestedMergeGroup {
    fn from(value: &MergeGroupChecksRequestedMergeGroup) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MergeGroupChecksRequestedMergeGroupHeadCommit {
    pub author: Option<MergeGroupChecksRequestedMergeGroupHeadCommitAuthor>,
    pub committer: Option<MergeGroupChecksRequestedMergeGroupHeadCommitAuthor>,
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub tree_id: String,
}
impl From<&MergeGroupChecksRequestedMergeGroupHeadCommit>
    for MergeGroupChecksRequestedMergeGroupHeadCommit
{
    fn from(value: &MergeGroupChecksRequestedMergeGroupHeadCommit) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MergeGroupChecksRequestedMergeGroupHeadCommitAuthor {
    pub email: String,
    pub name: String,
}
impl From<&MergeGroupChecksRequestedMergeGroupHeadCommitAuthor>
    for MergeGroupChecksRequestedMergeGroupHeadCommitAuthor
{
    fn from(value: &MergeGroupChecksRequestedMergeGroupHeadCommitAuthor) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MergeGroupDestroyedReason {
    #[serde(rename = "dequeued")]
    Dequeued,
    #[serde(rename = "invalidated")]
    Invalidated,
    #[serde(rename = "merged")]
    Merged,
}
impl From<&MergeGroupDestroyedReason> for MergeGroupDestroyedReason {
    fn from(value: &MergeGroupDestroyedReason) -> Self {
        *value
    }
}
impl ::std::fmt::Display for MergeGroupDestroyedReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Dequeued => write!(f, "dequeued"),
            Self::Invalidated => write!(f, "invalidated"),
            Self::Merged => write!(f, "merged"),
        }
    }
}
impl std::str::FromStr for MergeGroupDestroyedReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "dequeued" => Ok(Self::Dequeued),
            "invalidated" => Ok(Self::Invalidated),
            "merged" => Ok(Self::Merged),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MergeGroupDestroyedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MergeGroupDestroyedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MergeGroupDestroyedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Meta {
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        hook: MetaDeletedHook,
        hook_id: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
}
impl From<&Meta> for Meta {
    fn from(value: &Meta) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MetaDeletedHook {
    pub active: bool,
    pub config: MetaDeletedHookConfig,
    pub created_at: String,
    pub events: Vec<String>,
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub updated_at: String,
}
impl From<&MetaDeletedHook> for MetaDeletedHook {
    fn from(value: &MetaDeletedHook) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MetaDeletedHookConfig {
    pub content_type: MetaDeletedHookConfigContentType,
    pub insecure_ssl: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    pub url: String,
}
impl From<&MetaDeletedHookConfig> for MetaDeletedHookConfig {
    fn from(value: &MetaDeletedHookConfig) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MetaDeletedHookConfigContentType {
    #[serde(rename = "form")]
    Form,
    #[serde(rename = "json")]
    Json,
}
impl From<&MetaDeletedHookConfigContentType> for MetaDeletedHookConfigContentType {
    fn from(value: &MetaDeletedHookConfigContentType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for MetaDeletedHookConfigContentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Form => write!(f, "form"),
            Self::Json => write!(f, "json"),
        }
    }
}
impl std::str::FromStr for MetaDeletedHookConfigContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "form" => Ok(Self::Form),
            "json" => Ok(Self::Json),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MetaDeletedHookConfigContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MetaDeletedHookConfigContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MetaDeletedHookConfigContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Milestone {
    #[serde(rename = "closed")]
    Closed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        milestone: MilestoneClosedMilestone,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        milestone: MilestoneCreatedMilestone,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        milestone: MilestoneClosedMilestone,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: MilestoneEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        milestone: MilestoneClosedMilestone,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "opened")]
    Opened {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        milestone: MilestoneCreatedMilestone,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&Milestone> for Milestone {
    fn from(value: &Milestone) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MilestoneClosedMilestone {
    pub closed_at: Option<String>,
    pub closed_issues: i64,
    pub created_at: String,
    pub creator: Option<MilestoneClosedMilestoneCreator>,
    pub description: Option<String>,
    pub due_on: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub labels_url: String,
    pub node_id: String,
    pub number: i64,
    pub open_issues: i64,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub title: String,
    pub updated_at: String,
    pub url: String,
}
impl From<&MilestoneClosedMilestone> for MilestoneClosedMilestone {
    fn from(value: &MilestoneClosedMilestone) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MilestoneClosedMilestoneCreator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub id: i64,
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<MilestoneClosedMilestoneCreatorType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_view_type: Option<String>,
}
impl From<&MilestoneClosedMilestoneCreator> for MilestoneClosedMilestoneCreator {
    fn from(value: &MilestoneClosedMilestoneCreator) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum MilestoneClosedMilestoneCreatorType {
    Bot,
    Mannequin,
    Organization,
    User,
}
impl From<&MilestoneClosedMilestoneCreatorType> for MilestoneClosedMilestoneCreatorType {
    fn from(value: &MilestoneClosedMilestoneCreatorType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for MilestoneClosedMilestoneCreatorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bot => write!(f, "Bot"),
            Self::Mannequin => write!(f, "Mannequin"),
            Self::Organization => write!(f, "Organization"),
            Self::User => write!(f, "User"),
        }
    }
}
impl std::str::FromStr for MilestoneClosedMilestoneCreatorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Bot" => Ok(Self::Bot),
            "Mannequin" => Ok(Self::Mannequin),
            "Organization" => Ok(Self::Organization),
            "User" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for MilestoneClosedMilestoneCreatorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MilestoneClosedMilestoneCreatorType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MilestoneClosedMilestoneCreatorType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MilestoneCreatedMilestone {
    pub closed_at: Option<String>,
    pub closed_issues: i64,
    pub created_at: String,
    pub creator: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub description: Option<String>,
    pub due_on: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub labels_url: String,
    pub node_id: String,
    pub number: i64,
    pub open_issues: i64,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub title: String,
    pub updated_at: String,
    pub url: String,
}
impl From<&MilestoneCreatedMilestone> for MilestoneCreatedMilestone {
    fn from(value: &MilestoneCreatedMilestone) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct MilestoneEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due_on: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<DiscussionEditedChangesBody>,
}
impl From<&MilestoneEditedChanges> for MilestoneEditedChanges {
    fn from(value: &MilestoneEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum OrgBlock {
    #[serde(rename = "blocked")]
    Blocked {
        blocked_user: Option<DeploymentStatusCreatedDeploymentCreator>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "unblocked")]
    Unblocked {
        blocked_user: Option<DeploymentStatusCreatedDeploymentCreator>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
}
impl From<&OrgBlock> for OrgBlock {
    fn from(value: &OrgBlock) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Organization {
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        membership: Option<OrganizationDeletedMembership>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "member_added")]
    MemberAdded {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        membership: OrganizationDeletedMembership,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "member_invited")]
    MemberInvited {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        invitation: Box<OrganizationMemberInvitedInvitation>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        user: Option<DeploymentStatusCreatedDeploymentCreator>,
    },
    #[serde(rename = "member_removed")]
    MemberRemoved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        membership: OrganizationDeletedMembership,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "renamed")]
    Renamed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        changes: Option<OrganizationRenamedChanges>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        membership: Option<OrganizationDeletedMembership>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
}
impl From<&Organization> for Organization {
    fn from(value: &Organization) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationDeletedMembership {
    pub organization_url: String,
    pub role: String,
    pub state: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&OrganizationDeletedMembership> for OrganizationDeletedMembership {
    fn from(value: &OrganizationDeletedMembership) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationMemberInvitedInvitation {
    pub created_at: String,
    pub email: Option<String>,
    pub failed_at: Option<String>,
    pub failed_reason: Option<String>,
    pub id: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invitation_source: Option<String>,
    pub invitation_teams_url: String,
    pub inviter: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub login: Option<String>,
    pub node_id: String,
    pub role: String,
    pub team_count: f64,
}
impl From<&OrganizationMemberInvitedInvitation> for OrganizationMemberInvitedInvitation {
    fn from(value: &OrganizationMemberInvitedInvitation) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct OrganizationRenamedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<DiscussionEditedChangesBody>,
}
impl From<&OrganizationRenamedChanges> for OrganizationRenamedChanges {
    fn from(value: &OrganizationRenamedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Package {
    #[serde(rename = "published")]
    Published {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        package: PackagePublishedPackage,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "updated")]
    Updated {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        package: Box<PackageUpdatedPackage>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&Package> for Package {
    fn from(value: &Package) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackage {
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub ecosystem: String,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub namespace: String,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub package_type: String,
    pub package_version: Option<PackagePublishedPackagePackageVersion>,
    pub registry: Option<PackagePublishedPackageRegistry>,
    pub updated_at: Option<String>,
}
impl From<&PackagePublishedPackage> for PackagePublishedPackage {
    fn from(value: &PackagePublishedPackage) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackagePackageVersion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<PackagePublishedPackagePackageVersionBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_metadata: Option<PackagePublishedPackagePackageVersionContainerMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub docker_metadata: Vec<PackagePublishedPackagePackageVersionDockerMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub html_url: String,
    pub id: i64,
    pub installation_command: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    pub metadata: Vec<Untyped>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npm_metadata: Option<PackagePublishedPackagePackageVersionNpmMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nuget_metadata: Option<Vec<PackagePublishedPackagePackageVersionNugetMetadata>>,
    pub package_files: Vec<PackagePublishedPackagePackageVersionPackageFiles>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<PackagePublishedPackagePackageVersionRelease>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rubygems_metadata: Vec<PackagePublishedPackagePackageVersionRubygemsMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_commitish: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_oid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub version: String,
}
impl From<&PackagePublishedPackagePackageVersion> for PackagePublishedPackagePackageVersion {
    fn from(value: &PackagePublishedPackagePackageVersion) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PackagePublishedPackagePackageVersionBody {
    Variant0(Untyped),
    Variant1(String),
}
impl From<&PackagePublishedPackagePackageVersionBody>
    for PackagePublishedPackagePackageVersionBody
{
    fn from(value: &PackagePublishedPackagePackageVersionBody) -> Self {
        value.clone()
    }
}
impl From<Untyped> for PackagePublishedPackagePackageVersionBody {
    fn from(value: Untyped) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackagePackageVersionContainerMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<PackagePublishedPackagePackageVersionContainerMetadataTag>,
}
impl From<&PackagePublishedPackagePackageVersionContainerMetadata>
    for PackagePublishedPackagePackageVersionContainerMetadata
{
    fn from(value: &PackagePublishedPackagePackageVersionContainerMetadata) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackagePackageVersionContainerMetadataTag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl From<&PackagePublishedPackagePackageVersionContainerMetadataTag>
    for PackagePublishedPackagePackageVersionContainerMetadataTag
{
    fn from(value: &PackagePublishedPackagePackageVersionContainerMetadataTag) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackagePackageVersionDockerMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}
impl From<&PackagePublishedPackagePackageVersionDockerMetadata>
    for PackagePublishedPackagePackageVersionDockerMetadata
{
    fn from(value: &PackagePublishedPackagePackageVersionDockerMetadata) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackagePackageVersionNpmMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bin: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bugs: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_oid: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributors: Vec<Untyped>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cpu: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_by_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directories: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dist: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engines: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_head: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_shrinkwrap: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation_command: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub maintainers: Vec<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npm_user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npm_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional_dependencies: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub os: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_dependencies: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_via_actions: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl From<&PackagePublishedPackagePackageVersionNpmMetadata>
    for PackagePublishedPackagePackageVersionNpmMetadata
{
    fn from(value: &PackagePublishedPackagePackageVersionNpmMetadata) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackagePackageVersionNugetMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PackagePublishedPackagePackageVersionNugetMetadataId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<PackagePublishedPackagePackageVersionNugetMetadataValue>,
}
impl From<&PackagePublishedPackagePackageVersionNugetMetadata>
    for PackagePublishedPackagePackageVersionNugetMetadata
{
    fn from(value: &PackagePublishedPackagePackageVersionNugetMetadata) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PackagePublishedPackagePackageVersionNugetMetadataId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PackagePublishedPackagePackageVersionNugetMetadataId>
    for PackagePublishedPackagePackageVersionNugetMetadataId
{
    fn from(value: &PackagePublishedPackagePackageVersionNugetMetadataId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PackagePublishedPackagePackageVersionNugetMetadataId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for PackagePublishedPackagePackageVersionNugetMetadataId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PackagePublishedPackagePackageVersionNugetMetadataId {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PackagePublishedPackagePackageVersionNugetMetadataId {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for PackagePublishedPackagePackageVersionNugetMetadataId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for PackagePublishedPackagePackageVersionNugetMetadataId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PackagePublishedPackagePackageVersionNugetMetadataValue {
    Variant0(bool),
    Variant1(String),
    Variant2(i64),
    Variant3(Untyped),
}
impl From<&PackagePublishedPackagePackageVersionNugetMetadataValue>
    for PackagePublishedPackagePackageVersionNugetMetadataValue
{
    fn from(value: &PackagePublishedPackagePackageVersionNugetMetadataValue) -> Self {
        value.clone()
    }
}
impl From<bool> for PackagePublishedPackagePackageVersionNugetMetadataValue {
    fn from(value: bool) -> Self {
        Self::Variant0(value)
    }
}
impl From<i64> for PackagePublishedPackagePackageVersionNugetMetadataValue {
    fn from(value: i64) -> Self {
        Self::Variant2(value)
    }
}
impl From<Untyped> for PackagePublishedPackagePackageVersionNugetMetadataValue {
    fn from(value: Untyped) -> Self {
        Self::Variant3(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackagePackageVersionPackageFiles {
    pub content_type: String,
    pub created_at: String,
    pub download_url: String,
    pub id: i64,
    pub md5: Option<String>,
    pub name: String,
    pub sha1: Option<String>,
    pub sha256: Option<String>,
    pub size: i64,
    pub state: Option<String>,
    pub updated_at: String,
}
impl From<&PackagePublishedPackagePackageVersionPackageFiles>
    for PackagePublishedPackagePackageVersionPackageFiles
{
    fn from(value: &PackagePublishedPackagePackageVersionPackageFiles) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackagePackageVersionRelease {
    pub author: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub created_at: String,
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: Option<String>,
    pub prerelease: bool,
    pub published_at: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub url: String,
}
impl From<&PackagePublishedPackagePackageVersionRelease>
    for PackagePublishedPackagePackageVersionRelease
{
    fn from(value: &PackagePublishedPackagePackageVersionRelease) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackagePackageVersionRubygemsMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_oid: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_info: Option<PackagePublishedPackagePackageVersionRubygemsMetadataVersionInfo>,
}
impl From<&PackagePublishedPackagePackageVersionRubygemsMetadata>
    for PackagePublishedPackagePackageVersionRubygemsMetadata
{
    fn from(value: &PackagePublishedPackagePackageVersionRubygemsMetadata) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackagePackageVersionRubygemsMetadataVersionInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl From<&PackagePublishedPackagePackageVersionRubygemsMetadataVersionInfo>
    for PackagePublishedPackagePackageVersionRubygemsMetadataVersionInfo
{
    fn from(value: &PackagePublishedPackagePackageVersionRubygemsMetadataVersionInfo) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackagePublishedPackageRegistry {
    pub about_url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub vendor: String,
}
impl From<&PackagePublishedPackageRegistry> for PackagePublishedPackageRegistry {
    fn from(value: &PackagePublishedPackageRegistry) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackageUpdatedPackage {
    pub created_at: String,
    pub description: Option<String>,
    pub ecosystem: String,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub namespace: String,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub package_type: String,
    pub package_version: PackageUpdatedPackagePackageVersion,
    pub registry: Option<PackagePublishedPackageRegistry>,
    pub updated_at: String,
}
impl From<&PackageUpdatedPackage> for PackageUpdatedPackage {
    fn from(value: &PackageUpdatedPackage) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackageUpdatedPackagePackageVersion {
    pub author: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub body: String,
    pub body_html: String,
    pub created_at: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub docker_metadata: Vec<PackagePublishedPackagePackageVersionDockerMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub html_url: String,
    pub id: i64,
    pub installation_command: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    pub metadata: Vec<Untyped>,
    pub name: String,
    pub package_files: Vec<PackageUpdatedPackagePackageVersionPackageFiles>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<PackageUpdatedPackagePackageVersionRelease>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rubygems_metadata: Vec<PackagePublishedPackagePackageVersionRubygemsMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    pub target_commitish: String,
    pub target_oid: String,
    pub updated_at: String,
    pub version: String,
}
impl From<&PackageUpdatedPackagePackageVersion> for PackageUpdatedPackagePackageVersion {
    fn from(value: &PackageUpdatedPackagePackageVersion) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackageUpdatedPackagePackageVersionPackageFiles {
    pub content_type: String,
    pub created_at: String,
    pub download_url: String,
    pub id: i64,
    pub md5: Option<String>,
    pub name: String,
    pub sha1: Option<String>,
    pub sha256: String,
    pub size: i64,
    pub state: String,
    pub updated_at: String,
}
impl From<&PackageUpdatedPackagePackageVersionPackageFiles>
    for PackageUpdatedPackagePackageVersionPackageFiles
{
    fn from(value: &PackageUpdatedPackagePackageVersionPackageFiles) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PackageUpdatedPackagePackageVersionRelease {
    pub author: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub created_at: String,
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub prerelease: bool,
    pub published_at: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub url: String,
}
impl From<&PackageUpdatedPackagePackageVersionRelease>
    for PackageUpdatedPackagePackageVersionRelease
{
    fn from(value: &PackageUpdatedPackagePackageVersionRelease) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PageBuild {
    pub build: PageBuildDefaultBuild,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub repository: Untyped,
    pub sender: Untyped,
}
impl From<&PageBuild> for PageBuild {
    fn from(value: &PageBuild) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PageBuildDefaultBuild {
    pub commit: Option<String>,
    pub created_at: String,
    pub duration: i64,
    pub error: PageBuildDefaultBuildError,
    pub pusher: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub status: String,
    pub updated_at: String,
    pub url: String,
}
impl From<&PageBuildDefaultBuild> for PageBuildDefaultBuild {
    fn from(value: &PageBuildDefaultBuild) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PageBuildDefaultBuildError {
    pub message: Option<String>,
}
impl From<&PageBuildDefaultBuildError> for PageBuildDefaultBuildError {
    fn from(value: &PageBuildDefaultBuildError) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum PersonalAccessTokenRequest {
    #[serde(rename = "approved")]
    Approved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        installation: InstallationAttribute,
        organization: Untyped,
        personal_access_token_request: PersonalAccessTokenRequestApprovedPersonalAccessTokenRequest,
        sender: Untyped,
    },
    #[serde(rename = "cancelled")]
    Cancelled {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        installation: InstallationAttribute,
        organization: Untyped,
        personal_access_token_request: PersonalAccessTokenRequestApprovedPersonalAccessTokenRequest,
        sender: Untyped,
    },
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        organization: Untyped,
        personal_access_token_request: PersonalAccessTokenRequestApprovedPersonalAccessTokenRequest,
        sender: Untyped,
    },
    #[serde(rename = "denied")]
    Denied {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        installation: InstallationAttribute,
        organization: Untyped,
        personal_access_token_request: PersonalAccessTokenRequestApprovedPersonalAccessTokenRequest,
        sender: Untyped,
    },
}
impl From<&PersonalAccessTokenRequest> for PersonalAccessTokenRequest {
    fn from(value: &PersonalAccessTokenRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PersonalAccessTokenRequestApprovedPersonalAccessTokenRequest {
    pub created_at: String,
    pub id: i64,
    pub owner: DiscussionTransferredChangesNewRepositoryOrganization,
    pub permissions_added:
        PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestPermissionsAdded,
    pub permissions_result:
        PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestPermissionsAdded,
    pub permissions_upgraded:
        PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestPermissionsAdded,
    pub repositories: Option<Vec<InstallationCreatedRepositories>>,
    pub repository_count: Option<i64>,
    pub repository_selection:
        PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestRepositorySelection,
    pub token_expired: bool,
    pub token_expires_at: Option<String>,
    pub token_last_used_at: Option<String>,
}
impl From<&PersonalAccessTokenRequestApprovedPersonalAccessTokenRequest>
    for PersonalAccessTokenRequestApprovedPersonalAccessTokenRequest
{
    fn from(value: &PersonalAccessTokenRequestApprovedPersonalAccessTokenRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestPermissionsAdded {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Untyped>,
}
impl From<&PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestPermissionsAdded>
    for PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestPermissionsAdded
{
    fn from(
        value: &PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestPermissionsAdded,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestRepositorySelection {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "subset")]
    Subset,
}
impl From<&PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestRepositorySelection>
    for PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestRepositorySelection
{
    fn from(
        value: &PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestRepositorySelection,
    ) -> Self {
        *value
    }
}
impl ::std::fmt::Display
    for PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestRepositorySelection
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => write!(f, "all"),
            Self::None => write!(f, "none"),
            Self::Subset => write!(f, "subset"),
        }
    }
}
impl std::str::FromStr
    for PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestRepositorySelection
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            "subset" => Ok(Self::Subset),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestRepositorySelection
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestRepositorySelection
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for PersonalAccessTokenRequestApprovedPersonalAccessTokenRequestRepositorySelection
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Ping(pub PingDefault);
impl ::std::ops::Deref for Ping {
    type Target = PingDefault;
    fn deref(&self) -> &PingDefault {
        &self.0
    }
}
impl From<Ping> for PingDefault {
    fn from(value: Ping) -> Self {
        value.0
    }
}
impl From<&Ping> for Ping {
    fn from(value: &Ping) -> Self {
        value.clone()
    }
}
impl From<PingDefault> for Ping {
    fn from(value: PingDefault) -> Self {
        Self(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PingDefault {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hook: Option<PingDefaultHook>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hook_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zen: Option<String>,
}
impl From<&PingDefault> for PingDefault {
    fn from(value: &PingDefault) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PingDefaultHook {
    pub active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    pub config: PingDefaultHookConfig,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deliveries_url: Option<String>,
    pub events: Vec<String>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_response: Option<PingDefaultHookLastResponse>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ping_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_url: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&PingDefaultHook> for PingDefaultHook {
    fn from(value: &PingDefaultHook) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PingDefaultHookConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure_ssl: Option<PingDefaultHookConfigInsecureSsl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&PingDefaultHookConfig> for PingDefaultHookConfig {
    fn from(value: &PingDefaultHookConfig) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PingDefaultHookConfigInsecureSsl {
    Variant0(f64),
    Variant1(String),
}
impl From<&PingDefaultHookConfigInsecureSsl> for PingDefaultHookConfigInsecureSsl {
    fn from(value: &PingDefaultHookConfigInsecureSsl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PingDefaultHookConfigInsecureSsl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for PingDefaultHookConfigInsecureSsl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PingDefaultHookConfigInsecureSsl {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PingDefaultHookConfigInsecureSsl {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for PingDefaultHookConfigInsecureSsl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<f64> for PingDefaultHookConfigInsecureSsl {
    fn from(value: f64) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PingDefaultHookLastResponse {
    pub code: Option<i64>,
    pub message: Option<String>,
    pub status: Option<String>,
}
impl From<&PingDefaultHookLastResponse> for PingDefaultHookLastResponse {
    fn from(value: &PingDefaultHookLastResponse) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Project {
    #[serde(rename = "closed")]
    Closed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project: ProjectClosedProject,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project: ProjectClosedProject,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project: ProjectClosedProject,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "edited")]
    Edited {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        changes: Option<ProjectEditedChanges>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project: ProjectClosedProject,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "reopened")]
    Reopened {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project: ProjectClosedProject,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
}
impl From<&Project> for Project {
    fn from(value: &Project) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum ProjectCard {
    #[serde(rename = "converted")]
    Converted {
        changes: ProjectCardConvertedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project_card: ProjectCardConvertedProjectCard,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project_card: ProjectCardConvertedProjectCard,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project_card: ProjectCardDeletedProjectCard,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: ProjectCardEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project_card: ProjectCardConvertedProjectCard,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "moved")]
    Moved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        changes: Option<ProjectCardMovedChanges>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project_card: ProjectCardMovedProjectCard,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
}
impl From<&ProjectCard> for ProjectCard {
    fn from(value: &ProjectCard) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectCardConvertedChanges {
    pub note: DiscussionEditedChangesBody,
}
impl From<&ProjectCardConvertedChanges> for ProjectCardConvertedChanges {
    fn from(value: &ProjectCardConvertedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectCardConvertedProjectCard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_id: Option<i64>,
    pub archived: bool,
    pub column_id: i64,
    pub column_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    pub created_at: String,
    pub creator: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub id: i64,
    pub node_id: String,
    pub note: Option<String>,
    pub project_url: String,
    pub updated_at: String,
    pub url: String,
}
impl From<&ProjectCardConvertedProjectCard> for ProjectCardConvertedProjectCard {
    fn from(value: &ProjectCardConvertedProjectCard) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectCardDeletedProjectCard {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_id: Option<i64>,
    pub archived: bool,
    pub column_id: Option<i64>,
    pub column_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    pub created_at: String,
    pub creator: Option<MilestoneClosedMilestoneCreator>,
    pub id: i64,
    pub node_id: String,
    pub note: Option<String>,
    pub project_url: String,
    pub updated_at: String,
    pub url: String,
}
impl From<&ProjectCardDeletedProjectCard> for ProjectCardDeletedProjectCard {
    fn from(value: &ProjectCardDeletedProjectCard) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectCardEditedChanges {
    pub note: RepositoryEditedChangesDescription,
}
impl From<&ProjectCardEditedChanges> for ProjectCardEditedChanges {
    fn from(value: &ProjectCardEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectCardMovedChanges {
    pub column_id: ProjectCardMovedChangesColumnId,
}
impl From<&ProjectCardMovedChanges> for ProjectCardMovedChanges {
    fn from(value: &ProjectCardMovedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectCardMovedChangesColumnId {
    pub from: i64,
}
impl From<&ProjectCardMovedChangesColumnId> for ProjectCardMovedChangesColumnId {
    fn from(value: &ProjectCardMovedChangesColumnId) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectCardMovedProjectCard {
    pub after_id: Option<f64>,
    pub archived: bool,
    pub column_id: i64,
    pub column_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    pub created_at: String,
    pub creator: Option<MilestoneClosedMilestoneCreator>,
    pub id: i64,
    pub node_id: String,
    pub note: Option<String>,
    pub project_url: String,
    pub updated_at: String,
    pub url: String,
}
impl From<&ProjectCardMovedProjectCard> for ProjectCardMovedProjectCard {
    fn from(value: &ProjectCardMovedProjectCard) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectClosedProject {
    pub body: Option<String>,
    pub columns_url: String,
    pub created_at: String,
    pub creator: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub number: i64,
    pub owner_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub updated_at: String,
    pub url: String,
}
impl From<&ProjectClosedProject> for ProjectClosedProject {
    fn from(value: &ProjectClosedProject) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum ProjectColumn {
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project_column: ProjectColumnCreatedProjectColumn,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project_column: ProjectColumnCreatedProjectColumn,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: RepositoryRenamedChangesRepository,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project_column: ProjectColumnCreatedProjectColumn,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "moved")]
    Moved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        project_column: ProjectColumnCreatedProjectColumn,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
}
impl From<&ProjectColumn> for ProjectColumn {
    fn from(value: &ProjectColumn) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectColumnCreatedProjectColumn {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_id: Option<i64>,
    pub cards_url: String,
    pub created_at: String,
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub project_url: String,
    pub updated_at: String,
    pub url: String,
}
impl From<&ProjectColumnCreatedProjectColumn> for ProjectColumnCreatedProjectColumn {
    fn from(value: &ProjectColumnCreatedProjectColumn) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ProjectEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<DiscussionEditedChangesBody>,
}
impl From<&ProjectEditedChanges> for ProjectEditedChanges {
    fn from(value: &ProjectEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Public {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub repository: Untyped,
    pub sender: Untyped,
}
impl From<&Public> for Public {
    fn from(value: &Public) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PullRequest {
    Assigned {
        action: String,
        assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestAssignedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    AutoMergeDisabled {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestAutoMergeDisabledPullRequest,
        reason: String,
        repository: Untyped,
        sender: Untyped,
    },
    AutoMergeEnabled {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestAutoMergeEnabledPullRequest,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        reason: Option<String>,
        repository: Untyped,
        sender: Untyped,
    },
    Closed {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestClosedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    ConvertedToDraft {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestClosedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    Demilestoned(PullRequestDemilestoned),
    Dequeued {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestAutoMergeEnabledPullRequest,
        reason: PullRequestDequeuedReason,
        repository: Untyped,
        sender: Untyped,
    },
    Edited {
        action: String,
        changes: PullRequestEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestClosedPullRequest,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    Enqueued {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestAutoMergeEnabledPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    Labeled {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<DiscussionLabeledLabel>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestLabeledPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    Locked {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestLockedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    Milestoned(PullRequestMilestoned),
    Opened {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestClosedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    ReadyForReview {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestClosedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    Reopened {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestClosedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    ReviewRequestRemoved {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestReviewRequestRemovedPullRequest,
        repository: Untyped,
        requested_reviewer: Option<DeploymentStatusCreatedDeploymentCreator>,
        requested_team: PullRequestReviewCommentCreatedPullRequestRequestedTeams,
        sender: Untyped,
    },
    ReviewRequested {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestReviewRequestedPullRequest,
        repository: Untyped,
        requested_reviewer: Option<MilestoneClosedMilestoneCreator>,
        requested_team: PullRequestReviewCommentCreatedPullRequestRequestedTeams,
        sender: Untyped,
    },
    Synchronize {
        action: String,
        after: String,
        before: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestReviewRequestedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    Unassigned {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        assignee: Option<MilestoneClosedMilestoneCreator>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestUnassignedPullRequest,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    Unlabeled {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<DiscussionLabeledLabel>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestAssignedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    Unlocked {
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestUnlockedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&PullRequest> for PullRequest {
    fn from(value: &PullRequest) -> Self {
        value.clone()
    }
}
impl From<PullRequestDemilestoned> for PullRequest {
    fn from(value: PullRequestDemilestoned) -> Self {
        Self::Demilestoned(value)
    }
}
impl From<PullRequestMilestoned> for PullRequest {
    fn from(value: PullRequestMilestoned) -> Self {
        Self::Milestoned(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestAssignedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    pub assignee: Option<MilestoneClosedMilestoneCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<i64>,
    pub closed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    pub comments_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<i64>,
    pub commits_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewSubmittedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    pub merge_commit_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<i64>,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestAssignedPullRequest> for PullRequestAssignedPullRequest {
    fn from(value: &PullRequestAssignedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestAutoMergeDisabledPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestAutoMergeDisabledPullRequestBase,
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<i64>,
    pub closed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    pub comments_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<i64>,
    pub commits_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewCommentCreatedPullRequestBase,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    pub merge_commit_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub milestone: Option<MilestoneCreatedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<i64>,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestAutoMergeDisabledPullRequest> for PullRequestAutoMergeDisabledPullRequest {
    fn from(value: &PullRequestAutoMergeDisabledPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestAutoMergeDisabledPullRequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: PullRequestAutoMergeDisabledPullRequestBaseRepo,
    pub sha: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestAutoMergeDisabledPullRequestBase>
    for PullRequestAutoMergeDisabledPullRequestBase
{
    fn from(value: &PullRequestAutoMergeDisabledPullRequestBase) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestAutoMergeDisabledPullRequestBaseRepo {
    #[serde(default)]
    pub allow_auto_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_merge_commit: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_rebase_merge: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_squash_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    pub archive_url: String,
    pub archived: bool,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: PullRequestAutoMergeDisabledPullRequestBaseRepoCreatedAt,
    pub default_branch: String,
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deployments_url: String,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub has_discussions: bool,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub license: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoLicense>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle>,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoPermissions>,
    pub private: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    pub pulls_url: String,
    pub pushed_at: Option<PullRequestAutoMergeDisabledPullRequestBaseRepoPushedAt>,
    pub releases_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle>,
    pub ssh_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers: Option<i64>,
    pub stargazers_count: i64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub topics: Vec<String>,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    #[serde(default)]
    pub use_squash_pr_title_as_default: bool,
    pub visibility: PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility,
    pub watchers: i64,
    pub watchers_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_commit_signoff_required: Option<bool>,
}
impl From<&PullRequestAutoMergeDisabledPullRequestBaseRepo>
    for PullRequestAutoMergeDisabledPullRequestBaseRepo
{
    fn from(value: &PullRequestAutoMergeDisabledPullRequestBaseRepo) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PullRequestAutoMergeDisabledPullRequestBaseRepoCreatedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&PullRequestAutoMergeDisabledPullRequestBaseRepoCreatedAt>
    for PullRequestAutoMergeDisabledPullRequestBaseRepoCreatedAt
{
    fn from(value: &PullRequestAutoMergeDisabledPullRequestBaseRepoCreatedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PullRequestAutoMergeDisabledPullRequestBaseRepoCreatedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestAutoMergeDisabledPullRequestBaseRepoCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PullRequestAutoMergeDisabledPullRequestBaseRepoCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestAutoMergeDisabledPullRequestBaseRepoCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for PullRequestAutoMergeDisabledPullRequestBaseRepoCreatedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for PullRequestAutoMergeDisabledPullRequestBaseRepoCreatedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PullRequestAutoMergeDisabledPullRequestBaseRepoPushedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&PullRequestAutoMergeDisabledPullRequestBaseRepoPushedAt>
    for PullRequestAutoMergeDisabledPullRequestBaseRepoPushedAt
{
    fn from(value: &PullRequestAutoMergeDisabledPullRequestBaseRepoPushedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PullRequestAutoMergeDisabledPullRequestBaseRepoPushedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestAutoMergeDisabledPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PullRequestAutoMergeDisabledPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestAutoMergeDisabledPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for PullRequestAutoMergeDisabledPullRequestBaseRepoPushedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for PullRequestAutoMergeDisabledPullRequestBaseRepoPushedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestAutoMergeEnabledPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<i64>,
    pub closed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    pub comments_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<i64>,
    pub commits_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewCommentCreatedPullRequestBase,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    pub merge_commit_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<i64>,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestAutoMergeEnabledPullRequest> for PullRequestAutoMergeEnabledPullRequest {
    fn from(value: &PullRequestAutoMergeEnabledPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestClosedPullRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub additions: i64,
    #[serde(default)]
    pub allow_auto_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    pub assignee: DiscussionTransferredChangesNewRepositoryOrganization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<DiscussionTransferredChangesNewRepositoryOrganization>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<DeploymentProtectionRuleRequestedPullRequestsAutoMerge>,
    pub base: DeploymentProtectionRuleRequestedPullRequestsHead,
    pub body: Option<String>,
    pub changed_files: i64,
    pub closed_at: Option<String>,
    pub comments: i64,
    pub comments_url: String,
    pub commits: i64,
    pub commits_url: String,
    pub created_at: String,
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deletions: i64,
    pub diff_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub head: DeploymentProtectionRuleRequestedPullRequestsHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionAnsweredDiscussionLabels>,
    #[serde(rename = "_links")]
    pub links: DeploymentProtectionRuleRequestedPullRequestsLinks,
    pub locked: bool,
    pub maintainer_can_modify: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage>,
    pub merge_commit_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle>,
    pub mergeable: Option<bool>,
    pub mergeable_state: String,
    pub merged: bool,
    pub merged_at: Option<String>,
    pub merged_by: DiscussionTransferredChangesNewRepositoryOrganization,
    pub milestone: DeploymentProtectionRuleRequestedPullRequestsMilestone,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_reviewers: Option<Vec<DiscussionTransferredChangesNewRepositoryOrganization>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_teams: Option<Vec<DeploymentProtectionRuleRequestedPullRequestsRequestedTeams>>,
    pub review_comment_url: String,
    pub review_comments: i64,
    pub review_comments_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle>,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    #[serde(default)]
    pub use_squash_pr_title_as_default: bool,
    pub user: DiscussionTransferredChangesNewRepositoryOrganization,
}
impl From<&PullRequestClosedPullRequest> for PullRequestClosedPullRequest {
    fn from(value: &PullRequestClosedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestDemilestoned {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<DeploymentProtectionRuleRequestedPullRequestsMilestone>,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub pull_request: PullRequestDemilestonedPullRequest,
    pub repository: Untyped,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<Untyped>,
}
impl From<&PullRequestDemilestoned> for PullRequestDemilestoned {
    fn from(value: &PullRequestDemilestoned) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestDemilestonedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    pub assignee: Option<MilestoneClosedMilestoneCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<i64>,
    pub closed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    pub comments_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<i64>,
    pub commits_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewCommentCreatedPullRequestBase,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    pub merge_commit_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<MilestoneClosedMilestoneCreator>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<i64>,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestDemilestonedPullRequest> for PullRequestDemilestonedPullRequest {
    fn from(value: &PullRequestDemilestonedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PullRequestDequeuedReason {
    #[serde(rename = "ALREADY_MERGED")]
    AlreadyMerged,
    #[serde(rename = "BRANCH_PROTECTIONS")]
    BranchProtections,
    #[serde(rename = "CI_FAILURE")]
    CiFailure,
    #[serde(rename = "CI_TIMEOUT")]
    CiTimeout,
    #[serde(rename = "GIT_TREE_INVALID")]
    GitTreeInvalid,
    #[serde(rename = "INVALID_MERGE_COMMIT")]
    InvalidMergeCommit,
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "MERGE")]
    Merge,
    #[serde(rename = "MERGE_CONFLICT")]
    MergeConflict,
    #[serde(rename = "QUEUE_CLEARED")]
    QueueCleared,
    #[serde(rename = "ROLL_BACK")]
    RollBack,
    #[serde(rename = "UNKNOWN_REMOVAL_REASON")]
    UnknownRemovalReason,
}
impl From<&PullRequestDequeuedReason> for PullRequestDequeuedReason {
    fn from(value: &PullRequestDequeuedReason) -> Self {
        *value
    }
}
impl ::std::fmt::Display for PullRequestDequeuedReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AlreadyMerged => write!(f, "ALREADY_MERGED"),
            Self::BranchProtections => write!(f, "BRANCH_PROTECTIONS"),
            Self::CiFailure => write!(f, "CI_FAILURE"),
            Self::CiTimeout => write!(f, "CI_TIMEOUT"),
            Self::GitTreeInvalid => write!(f, "GIT_TREE_INVALID"),
            Self::InvalidMergeCommit => write!(f, "INVALID_MERGE_COMMIT"),
            Self::Manual => write!(f, "MANUAL"),
            Self::Merge => write!(f, "MERGE"),
            Self::MergeConflict => write!(f, "MERGE_CONFLICT"),
            Self::QueueCleared => write!(f, "QUEUE_CLEARED"),
            Self::RollBack => write!(f, "ROLL_BACK"),
            Self::UnknownRemovalReason => write!(f, "UNKNOWN_REMOVAL_REASON"),
        }
    }
}
impl std::str::FromStr for PullRequestDequeuedReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ALREADY_MERGED" => Ok(Self::AlreadyMerged),
            "BRANCH_PROTECTIONS" => Ok(Self::BranchProtections),
            "CI_FAILURE" => Ok(Self::CiFailure),
            "CI_TIMEOUT" => Ok(Self::CiTimeout),
            "GIT_TREE_INVALID" => Ok(Self::GitTreeInvalid),
            "INVALID_MERGE_COMMIT" => Ok(Self::InvalidMergeCommit),
            "MANUAL" => Ok(Self::Manual),
            "MERGE" => Ok(Self::Merge),
            "MERGE_CONFLICT" => Ok(Self::MergeConflict),
            "QUEUE_CLEARED" => Ok(Self::QueueCleared),
            "ROLL_BACK" => Ok(Self::RollBack),
            "UNKNOWN_REMOVAL_REASON" => Ok(Self::UnknownRemovalReason),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestDequeuedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PullRequestDequeuedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestDequeuedReason {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<PullRequestEditedChangesBase>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<DiscussionEditedChangesBody>,
}
impl From<&PullRequestEditedChanges> for PullRequestEditedChanges {
    fn from(value: &PullRequestEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestEditedChangesBase {
    #[serde(rename = "ref")]
    pub ref_: DiscussionEditedChangesBody,
    pub sha: DiscussionEditedChangesBody,
}
impl From<&PullRequestEditedChangesBase> for PullRequestEditedChangesBase {
    fn from(value: &PullRequestEditedChangesBase) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestLabeledPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    pub assignee: Option<MilestoneClosedMilestoneCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<i64>,
    pub closed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    pub comments_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<i64>,
    pub commits_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewSubmittedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    pub merge_commit_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<MilestoneClosedMilestoneCreator>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<i64>,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestLabeledPullRequest> for PullRequestLabeledPullRequest {
    fn from(value: &PullRequestLabeledPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestLockedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<i64>,
    pub closed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    pub comments_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<i64>,
    pub commits_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewSubmittedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    pub merge_commit_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<MilestoneClosedMilestoneCreator>,
    pub milestone: Option<MilestoneCreatedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<i64>,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestLockedPullRequest> for PullRequestLockedPullRequest {
    fn from(value: &PullRequestLockedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestMilestoned {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<DeploymentProtectionRuleRequestedPullRequestsMilestone>,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub pull_request: PullRequestDemilestonedPullRequest,
    pub repository: Untyped,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<Untyped>,
}
impl From<&PullRequestMilestoned> for PullRequestMilestoned {
    fn from(value: &PullRequestMilestoned) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum PullRequestReview {
    #[serde(rename = "dismissed")]
    Dismissed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestReviewDismissedPullRequest,
        repository: Untyped,
        review: PullRequestReviewDismissedReview,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: IssueCommentEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestReviewEditedPullRequest,
        repository: Untyped,
        review: PullRequestReviewEditedReview,
        sender: Untyped,
    },
    #[serde(rename = "submitted")]
    Submitted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestReviewSubmittedPullRequest,
        repository: Untyped,
        review: PullRequestReviewEditedReview,
        sender: Untyped,
    },
}
impl From<&PullRequestReview> for PullRequestReview {
    fn from(value: &PullRequestReview) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum PullRequestReviewComment {
    #[serde(rename = "created")]
    Created {
        comment: PullRequestReviewCommentCreatedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestReviewCommentCreatedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        comment: PullRequestReviewCommentDeletedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestReviewCommentDeletedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: IssueCommentEditedChanges,
        comment: PullRequestReviewCommentDeletedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestReviewCommentEditedPullRequest,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&PullRequestReviewComment> for PullRequestReviewComment {
    fn from(value: &PullRequestReviewComment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedComment {
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: String,
    pub commit_id: String,
    pub created_at: String,
    pub diff_hunk: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<i64>,
    pub line: Option<i64>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedCommentLinks,
    pub node_id: String,
    pub original_commit_id: String,
    pub original_line: Option<i64>,
    pub original_position: i64,
    pub original_start_line: Option<i64>,
    pub path: String,
    pub position: Option<i64>,
    pub pull_request_review_id: Option<i64>,
    pub pull_request_url: String,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub side: PullRequestReviewCommentCreatedCommentSide,
    pub start_line: Option<i64>,
    pub start_side: Option<PullRequestReviewCommentCreatedCommentStartSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<PullRequestReviewCommentCreatedCommentSubjectType>,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewCommentCreatedComment> for PullRequestReviewCommentCreatedComment {
    fn from(value: &PullRequestReviewCommentCreatedComment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedCommentLinks {
    pub html: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub pull_request: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    #[serde(rename = "self")]
    pub self_: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
}
impl From<&PullRequestReviewCommentCreatedCommentLinks>
    for PullRequestReviewCommentCreatedCommentLinks
{
    fn from(value: &PullRequestReviewCommentCreatedCommentLinks) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PullRequestReviewCommentCreatedCommentSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}
impl From<&PullRequestReviewCommentCreatedCommentSide>
    for PullRequestReviewCommentCreatedCommentSide
{
    fn from(value: &PullRequestReviewCommentCreatedCommentSide) -> Self {
        *value
    }
}
impl ::std::fmt::Display for PullRequestReviewCommentCreatedCommentSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Left => write!(f, "LEFT"),
            Self::Right => write!(f, "RIGHT"),
        }
    }
}
impl std::str::FromStr for PullRequestReviewCommentCreatedCommentSide {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "LEFT" => Ok(Self::Left),
            "RIGHT" => Ok(Self::Right),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestReviewCommentCreatedCommentSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PullRequestReviewCommentCreatedCommentSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestReviewCommentCreatedCommentSide {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(
    Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Default,
)]
pub enum PullRequestReviewCommentCreatedCommentStartSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    #[default]
    Right,
}
impl From<&PullRequestReviewCommentCreatedCommentStartSide>
    for PullRequestReviewCommentCreatedCommentStartSide
{
    fn from(value: &PullRequestReviewCommentCreatedCommentStartSide) -> Self {
        *value
    }
}
impl ::std::fmt::Display for PullRequestReviewCommentCreatedCommentStartSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Left => write!(f, "LEFT"),
            Self::Right => write!(f, "RIGHT"),
        }
    }
}
impl std::str::FromStr for PullRequestReviewCommentCreatedCommentStartSide {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "LEFT" => Ok(Self::Left),
            "RIGHT" => Ok(Self::Right),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestReviewCommentCreatedCommentStartSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PullRequestReviewCommentCreatedCommentStartSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestReviewCommentCreatedCommentStartSide {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PullRequestReviewCommentCreatedCommentSubjectType {
    #[serde(rename = "file")]
    File,
    #[serde(rename = "line")]
    Line,
}
impl From<&PullRequestReviewCommentCreatedCommentSubjectType>
    for PullRequestReviewCommentCreatedCommentSubjectType
{
    fn from(value: &PullRequestReviewCommentCreatedCommentSubjectType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for PullRequestReviewCommentCreatedCommentSubjectType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::File => write!(f, "file"),
            Self::Line => write!(f, "line"),
        }
    }
}
impl std::str::FromStr for PullRequestReviewCommentCreatedCommentSubjectType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "file" => Ok(Self::File),
            "line" => Ok(Self::Line),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestReviewCommentCreatedCommentSubjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PullRequestReviewCommentCreatedCommentSubjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestReviewCommentCreatedCommentSubjectType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    pub assignee: Option<MilestoneClosedMilestoneCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub head: PullRequestReviewCommentCreatedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestReviewCommentCreatedPullRequest>
    for PullRequestReviewCommentCreatedPullRequest
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedPullRequestAutoMerge {
    pub commit_message: Option<String>,
    pub commit_title: Option<String>,
    pub enabled_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub merge_method: DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod,
}
impl From<&PullRequestReviewCommentCreatedPullRequestAutoMerge>
    for PullRequestReviewCommentCreatedPullRequestAutoMerge
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestAutoMerge) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedPullRequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: PullRequestReviewCommentCreatedPullRequestBaseRepo,
    pub sha: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewCommentCreatedPullRequestBase>
    for PullRequestReviewCommentCreatedPullRequestBase
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestBase) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedPullRequestBaseRepo {
    #[serde(default)]
    pub allow_auto_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_merge_commit: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_rebase_merge: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_squash_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    pub archive_url: String,
    pub archived: bool,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: PullRequestReviewCommentCreatedPullRequestBaseRepoCreatedAt,
    pub default_branch: String,
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deployments_url: String,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub has_discussions: bool,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub license: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoLicense>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositoryMergeCommitTitle>,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoPermissions>,
    pub private: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    pub pulls_url: String,
    pub pushed_at: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoPushedAt>,
    pub releases_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    pub size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_message:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub squash_merge_commit_title:
        Option<DiscussionTransferredChangesNewRepositoryTemplateRepositorySquashMergeCommitTitle>,
    pub ssh_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers: Option<i64>,
    pub stargazers_count: i64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub topics: Vec<String>,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    #[serde(default)]
    pub use_squash_pr_title_as_default: bool,
    pub visibility: PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility,
    pub watchers: i64,
    pub watchers_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_commit_signoff_required: Option<bool>,
}
impl From<&PullRequestReviewCommentCreatedPullRequestBaseRepo>
    for PullRequestReviewCommentCreatedPullRequestBaseRepo
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestBaseRepo) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PullRequestReviewCommentCreatedPullRequestBaseRepoCreatedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&PullRequestReviewCommentCreatedPullRequestBaseRepoCreatedAt>
    for PullRequestReviewCommentCreatedPullRequestBaseRepoCreatedAt
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestBaseRepoCreatedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PullRequestReviewCommentCreatedPullRequestBaseRepoCreatedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestReviewCommentCreatedPullRequestBaseRepoCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for PullRequestReviewCommentCreatedPullRequestBaseRepoCreatedAt
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestReviewCommentCreatedPullRequestBaseRepoCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for PullRequestReviewCommentCreatedPullRequestBaseRepoCreatedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for PullRequestReviewCommentCreatedPullRequestBaseRepoCreatedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedPullRequestBaseRepoLicense {
    pub key: String,
    pub name: String,
    pub node_id: String,
    pub spdx_id: String,
    pub url: Option<String>,
}
impl From<&PullRequestReviewCommentCreatedPullRequestBaseRepoLicense>
    for PullRequestReviewCommentCreatedPullRequestBaseRepoLicense
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestBaseRepoLicense) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedPullRequestBaseRepoPermissions {
    pub admin: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintain: Option<bool>,
    pub pull: bool,
    pub push: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triage: Option<bool>,
}
impl From<&PullRequestReviewCommentCreatedPullRequestBaseRepoPermissions>
    for PullRequestReviewCommentCreatedPullRequestBaseRepoPermissions
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestBaseRepoPermissions) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PullRequestReviewCommentCreatedPullRequestBaseRepoPushedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&PullRequestReviewCommentCreatedPullRequestBaseRepoPushedAt>
    for PullRequestReviewCommentCreatedPullRequestBaseRepoPushedAt
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestBaseRepoPushedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PullRequestReviewCommentCreatedPullRequestBaseRepoPushedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestReviewCommentCreatedPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PullRequestReviewCommentCreatedPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestReviewCommentCreatedPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for PullRequestReviewCommentCreatedPullRequestBaseRepoPushedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for PullRequestReviewCommentCreatedPullRequestBaseRepoPushedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
}
impl From<&PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility>
    for PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility) -> Self {
        *value
    }
}
impl ::std::fmt::Display for PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Internal => write!(f, "internal"),
            Self::Private => write!(f, "private"),
            Self::Public => write!(f, "public"),
        }
    }
}
impl std::str::FromStr for PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "internal" => Ok(Self::Internal),
            "private" => Ok(Self::Private),
            "public" => Ok(Self::Public),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedPullRequestHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Option<PullRequestReviewCommentCreatedPullRequestBaseRepo>,
    pub sha: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewCommentCreatedPullRequestHead>
    for PullRequestReviewCommentCreatedPullRequestHead
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestHead) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedPullRequestLinks {
    pub comments: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub commits: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub html: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub issue: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub review_comment: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub review_comments: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    #[serde(rename = "self")]
    pub self_: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub statuses: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
}
impl From<&PullRequestReviewCommentCreatedPullRequestLinks>
    for PullRequestReviewCommentCreatedPullRequestLinks
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestLinks) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedPullRequestRequestedTeams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_url: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<PullRequestReviewCommentCreatedPullRequestRequestedTeamsParent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<TeamAddDefaultTeamParentPrivacy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&PullRequestReviewCommentCreatedPullRequestRequestedTeams>
    for PullRequestReviewCommentCreatedPullRequestRequestedTeams
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestRequestedTeams) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentCreatedPullRequestRequestedTeamsParent {
    pub description: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub members_url: String,
    pub name: String,
    pub node_id: String,
    pub permission: String,
    pub privacy: TeamAddDefaultTeamParentPrivacy,
    pub repositories_url: String,
    pub slug: String,
    pub url: String,
}
impl From<&PullRequestReviewCommentCreatedPullRequestRequestedTeamsParent>
    for PullRequestReviewCommentCreatedPullRequestRequestedTeamsParent
{
    fn from(value: &PullRequestReviewCommentCreatedPullRequestRequestedTeamsParent) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentDeletedComment {
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: String,
    pub commit_id: String,
    pub created_at: String,
    pub diff_hunk: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<i64>,
    pub line: Option<i64>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedCommentLinks,
    pub node_id: String,
    pub original_commit_id: String,
    pub original_line: i64,
    pub original_position: i64,
    pub original_start_line: Option<i64>,
    pub path: String,
    pub position: Option<i64>,
    pub pull_request_review_id: Option<i64>,
    pub pull_request_url: String,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub side: PullRequestReviewCommentCreatedCommentSide,
    pub start_line: Option<i64>,
    pub start_side: Option<PullRequestReviewCommentCreatedCommentStartSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<PullRequestReviewCommentCreatedCommentSubjectType>,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewCommentDeletedComment> for PullRequestReviewCommentDeletedComment {
    fn from(value: &PullRequestReviewCommentDeletedComment) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentDeletedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub head: PullRequestReviewCommentCreatedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<MilestoneCreatedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestReviewCommentDeletedPullRequest>
    for PullRequestReviewCommentDeletedPullRequest
{
    fn from(value: &PullRequestReviewCommentDeletedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentEditedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub head: PullRequestReviewCommentCreatedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<PullRequestReviewCommentEditedPullRequestUser>,
}
impl From<&PullRequestReviewCommentEditedPullRequest>
    for PullRequestReviewCommentEditedPullRequest
{
    fn from(value: &PullRequestReviewCommentEditedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewCommentEditedPullRequestUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub id: i64,
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<MilestoneClosedMilestoneCreatorType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_view_type: Option<String>,
}
impl From<&PullRequestReviewCommentEditedPullRequestUser>
    for PullRequestReviewCommentEditedPullRequestUser
{
    fn from(value: &PullRequestReviewCommentEditedPullRequestUser) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewDismissedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    pub assignee: Option<MilestoneClosedMilestoneCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewCommentCreatedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<MilestoneCreatedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestReviewDismissedPullRequest> for PullRequestReviewDismissedPullRequest {
    fn from(value: &PullRequestReviewDismissedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewDismissedReview {
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub commit_id: String,
    pub html_url: String,
    pub id: i64,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewDismissedReviewLinks,
    pub node_id: String,
    pub pull_request_url: String,
    pub state: PullRequestReviewDismissedReviewState,
    pub submitted_at: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestReviewDismissedReview> for PullRequestReviewDismissedReview {
    fn from(value: &PullRequestReviewDismissedReview) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewDismissedReviewLinks {
    pub html: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
    pub pull_request: RepositoryRulesetCreatedRepositoryRulesetLinksSelf,
}
impl From<&PullRequestReviewDismissedReviewLinks> for PullRequestReviewDismissedReviewLinks {
    fn from(value: &PullRequestReviewDismissedReviewLinks) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PullRequestReviewDismissedReviewState {
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "changes_requested")]
    ChangesRequested,
    #[serde(rename = "dismissed")]
    Dismissed,
}
impl From<&PullRequestReviewDismissedReviewState> for PullRequestReviewDismissedReviewState {
    fn from(value: &PullRequestReviewDismissedReviewState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for PullRequestReviewDismissedReviewState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Approved => write!(f, "approved"),
            Self::ChangesRequested => write!(f, "changes_requested"),
            Self::Dismissed => write!(f, "dismissed"),
        }
    }
}
impl std::str::FromStr for PullRequestReviewDismissedReviewState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "approved" => Ok(Self::Approved),
            "changes_requested" => Ok(Self::ChangesRequested),
            "dismissed" => Ok(Self::Dismissed),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestReviewDismissedReviewState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PullRequestReviewDismissedReviewState {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestReviewDismissedReviewState {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewEditedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    pub assignee: Option<MilestoneClosedMilestoneCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewEditedPullRequestBase,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewEditedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestReviewEditedPullRequest> for PullRequestReviewEditedPullRequest {
    fn from(value: &PullRequestReviewEditedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewEditedPullRequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: PullRequestReviewEditedPullRequestBaseRepo,
    pub sha: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewEditedPullRequestBase> for PullRequestReviewEditedPullRequestBase {
    fn from(value: &PullRequestReviewEditedPullRequestBase) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewEditedPullRequestBaseRepo {
    #[serde(default)]
    pub allow_auto_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_merge_commit: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_rebase_merge: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_squash_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    pub archive_url: String,
    pub archived: bool,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: PullRequestReviewEditedPullRequestBaseRepoCreatedAt,
    pub default_branch: String,
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deployments_url: String,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub license: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoLicense>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoPermissions>,
    pub private: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    pub pulls_url: String,
    pub pushed_at: Option<PullRequestReviewEditedPullRequestBaseRepoPushedAt>,
    pub releases_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    pub size: i64,
    pub ssh_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers: Option<i64>,
    pub stargazers_count: i64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub topics: Vec<String>,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    pub visibility: PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility,
    pub watchers: i64,
    pub watchers_count: i64,
}
impl From<&PullRequestReviewEditedPullRequestBaseRepo>
    for PullRequestReviewEditedPullRequestBaseRepo
{
    fn from(value: &PullRequestReviewEditedPullRequestBaseRepo) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PullRequestReviewEditedPullRequestBaseRepoCreatedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&PullRequestReviewEditedPullRequestBaseRepoCreatedAt>
    for PullRequestReviewEditedPullRequestBaseRepoCreatedAt
{
    fn from(value: &PullRequestReviewEditedPullRequestBaseRepoCreatedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PullRequestReviewEditedPullRequestBaseRepoCreatedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestReviewEditedPullRequestBaseRepoCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PullRequestReviewEditedPullRequestBaseRepoCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestReviewEditedPullRequestBaseRepoCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for PullRequestReviewEditedPullRequestBaseRepoCreatedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for PullRequestReviewEditedPullRequestBaseRepoCreatedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PullRequestReviewEditedPullRequestBaseRepoPushedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&PullRequestReviewEditedPullRequestBaseRepoPushedAt>
    for PullRequestReviewEditedPullRequestBaseRepoPushedAt
{
    fn from(value: &PullRequestReviewEditedPullRequestBaseRepoPushedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PullRequestReviewEditedPullRequestBaseRepoPushedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestReviewEditedPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PullRequestReviewEditedPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestReviewEditedPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for PullRequestReviewEditedPullRequestBaseRepoPushedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for PullRequestReviewEditedPullRequestBaseRepoPushedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewEditedPullRequestHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Option<PullRequestReviewEditedPullRequestBaseRepo>,
    pub sha: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewEditedPullRequestHead> for PullRequestReviewEditedPullRequestHead {
    fn from(value: &PullRequestReviewEditedPullRequestHead) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewEditedReview {
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: Option<String>,
    pub commit_id: String,
    pub html_url: String,
    pub id: i64,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewDismissedReviewLinks,
    pub node_id: String,
    pub pull_request_url: String,
    pub state: String,
    pub submitted_at: Option<String>,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewEditedReview> for PullRequestReviewEditedReview {
    fn from(value: &PullRequestReviewEditedReview) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewRequestRemovedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<i64>,
    pub closed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    pub comments_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<i64>,
    pub commits_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewCommentCreatedPullRequestBase,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    pub merge_commit_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub milestone: Option<MilestoneCreatedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<i64>,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewRequestRemovedPullRequest>
    for PullRequestReviewRequestRemovedPullRequest
{
    fn from(value: &PullRequestReviewRequestRemovedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewRequestedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    pub assignee: Option<MilestoneClosedMilestoneCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<i64>,
    pub closed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    pub comments_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<i64>,
    pub commits_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewCommentCreatedPullRequestBase,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    pub merge_commit_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<i64>,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestReviewRequestedPullRequest> for PullRequestReviewRequestedPullRequest {
    fn from(value: &PullRequestReviewRequestedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewSubmittedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    pub assignee: Option<MilestoneClosedMilestoneCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewSubmittedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestReviewSubmittedPullRequest> for PullRequestReviewSubmittedPullRequest {
    fn from(value: &PullRequestReviewSubmittedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewSubmittedPullRequestHead {
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Option<PullRequestReviewCommentCreatedPullRequestBaseRepo>,
    pub sha: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewSubmittedPullRequestHead>
    for PullRequestReviewSubmittedPullRequestHead
{
    fn from(value: &PullRequestReviewSubmittedPullRequestHead) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum PullRequestReviewThread {
    #[serde(rename = "resolved")]
    Resolved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestReviewThreadResolvedPullRequest,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
        thread: PullRequestReviewThreadResolvedThread,
    },
    #[serde(rename = "unresolved")]
    Unresolved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        pull_request: PullRequestReviewThreadUnresolvedPullRequest,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
        thread: PullRequestReviewThreadUnresolvedThread,
    },
}
impl From<&PullRequestReviewThread> for PullRequestReviewThread {
    fn from(value: &PullRequestReviewThread) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewThreadResolvedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestReviewThreadResolvedPullRequestBase,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewThreadResolvedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<MilestoneCreatedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestReviewThreadResolvedPullRequest>
    for PullRequestReviewThreadResolvedPullRequest
{
    fn from(value: &PullRequestReviewThreadResolvedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewThreadResolvedPullRequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: PullRequestReviewThreadResolvedPullRequestBaseRepo,
    pub sha: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewThreadResolvedPullRequestBase>
    for PullRequestReviewThreadResolvedPullRequestBase
{
    fn from(value: &PullRequestReviewThreadResolvedPullRequestBase) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewThreadResolvedPullRequestBaseRepo {
    #[serde(default)]
    pub allow_auto_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_merge_commit: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_rebase_merge: bool,
    #[serde(default = "defaults::default_bool::<true>")]
    pub allow_squash_merge: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    pub archive_url: String,
    pub archived: bool,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: PullRequestReviewThreadResolvedPullRequestBaseRepoCreatedAt,
    pub default_branch: String,
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deployments_url: String,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub has_discussions: bool,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub license: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoLicense>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<PullRequestReviewCommentCreatedPullRequestBaseRepoPermissions>,
    pub private: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    pub pulls_url: String,
    pub pushed_at: Option<PullRequestReviewThreadResolvedPullRequestBaseRepoPushedAt>,
    pub releases_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    pub size: i64,
    pub ssh_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers: Option<i64>,
    pub stargazers_count: i64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub topics: Vec<String>,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    pub visibility: PullRequestReviewCommentCreatedPullRequestBaseRepoVisibility,
    pub watchers: i64,
    pub watchers_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_commit_signoff_required: Option<bool>,
}
impl From<&PullRequestReviewThreadResolvedPullRequestBaseRepo>
    for PullRequestReviewThreadResolvedPullRequestBaseRepo
{
    fn from(value: &PullRequestReviewThreadResolvedPullRequestBaseRepo) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PullRequestReviewThreadResolvedPullRequestBaseRepoCreatedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&PullRequestReviewThreadResolvedPullRequestBaseRepoCreatedAt>
    for PullRequestReviewThreadResolvedPullRequestBaseRepoCreatedAt
{
    fn from(value: &PullRequestReviewThreadResolvedPullRequestBaseRepoCreatedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PullRequestReviewThreadResolvedPullRequestBaseRepoCreatedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestReviewThreadResolvedPullRequestBaseRepoCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for PullRequestReviewThreadResolvedPullRequestBaseRepoCreatedAt
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestReviewThreadResolvedPullRequestBaseRepoCreatedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for PullRequestReviewThreadResolvedPullRequestBaseRepoCreatedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for PullRequestReviewThreadResolvedPullRequestBaseRepoCreatedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum PullRequestReviewThreadResolvedPullRequestBaseRepoPushedAt {
    Variant0(String),
    Variant1(i64),
}
impl From<&PullRequestReviewThreadResolvedPullRequestBaseRepoPushedAt>
    for PullRequestReviewThreadResolvedPullRequestBaseRepoPushedAt
{
    fn from(value: &PullRequestReviewThreadResolvedPullRequestBaseRepoPushedAt) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PullRequestReviewThreadResolvedPullRequestBaseRepoPushedAt {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl std::convert::TryFrom<&str> for PullRequestReviewThreadResolvedPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PullRequestReviewThreadResolvedPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PullRequestReviewThreadResolvedPullRequestBaseRepoPushedAt {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for PullRequestReviewThreadResolvedPullRequestBaseRepoPushedAt {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Variant0(x) => x.fmt(f),
            Self::Variant1(x) => x.fmt(f),
        }
    }
}
impl From<i64> for PullRequestReviewThreadResolvedPullRequestBaseRepoPushedAt {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewThreadResolvedPullRequestHead {
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Option<PullRequestReviewThreadResolvedPullRequestBaseRepo>,
    pub sha: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewThreadResolvedPullRequestHead>
    for PullRequestReviewThreadResolvedPullRequestHead
{
    fn from(value: &PullRequestReviewThreadResolvedPullRequestHead) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewThreadResolvedThread {
    pub comments: Vec<PullRequestReviewThreadResolvedThreadComments>,
    pub node_id: String,
}
impl From<&PullRequestReviewThreadResolvedThread> for PullRequestReviewThreadResolvedThread {
    fn from(value: &PullRequestReviewThreadResolvedThread) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewThreadResolvedThreadComments {
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub body: String,
    pub commit_id: String,
    pub created_at: String,
    pub diff_hunk: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<i64>,
    pub line: Option<i64>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedCommentLinks,
    pub node_id: String,
    pub original_commit_id: String,
    pub original_line: Option<i64>,
    pub original_position: i64,
    pub original_start_line: Option<i64>,
    pub path: String,
    pub position: Option<i64>,
    pub pull_request_review_id: Option<i64>,
    pub pull_request_url: String,
    pub reactions: DiscussionAnsweredAnswerReactions,
    pub side: PullRequestReviewCommentCreatedCommentSide,
    pub start_line: Option<i64>,
    pub start_side: Option<PullRequestReviewCommentCreatedCommentStartSide>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_type: Option<PullRequestReviewCommentCreatedCommentSubjectType>,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestReviewThreadResolvedThreadComments>
    for PullRequestReviewThreadResolvedThreadComments
{
    fn from(value: &PullRequestReviewThreadResolvedThreadComments) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewThreadUnresolvedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewThreadUnresolvedPullRequestAutoMerge>,
    pub base: PullRequestReviewThreadResolvedPullRequestBase,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewThreadResolvedPullRequestBase,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<MilestoneCreatedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestReviewThreadUnresolvedPullRequest>
    for PullRequestReviewThreadUnresolvedPullRequest
{
    fn from(value: &PullRequestReviewThreadUnresolvedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewThreadUnresolvedPullRequestAutoMerge {
    pub commit_message: Option<String>,
    pub commit_title: String,
    pub enabled_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub merge_method: DeploymentProtectionRuleRequestedPullRequestsAutoMergeMergeMethod,
}
impl From<&PullRequestReviewThreadUnresolvedPullRequestAutoMerge>
    for PullRequestReviewThreadUnresolvedPullRequestAutoMerge
{
    fn from(value: &PullRequestReviewThreadUnresolvedPullRequestAutoMerge) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestReviewThreadUnresolvedThread {
    pub comments: Vec<PullRequestReviewCommentDeletedComment>,
    pub node_id: String,
}
impl From<&PullRequestReviewThreadUnresolvedThread> for PullRequestReviewThreadUnresolvedThread {
    fn from(value: &PullRequestReviewThreadUnresolvedThread) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestUnassignedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    pub assignee: Option<MilestoneClosedMilestoneCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewCommentCreatedPullRequestAutoMerge>,
    pub base: PullRequestUnassignedPullRequestBase,
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<i64>,
    pub closed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    pub comments_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<i64>,
    pub commits_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewSubmittedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    pub merge_commit_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<MilestoneClosedMilestoneCreator>,
    pub milestone: Option<MilestoneClosedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<i64>,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<MilestoneClosedMilestoneCreator>,
}
impl From<&PullRequestUnassignedPullRequest> for PullRequestUnassignedPullRequest {
    fn from(value: &PullRequestUnassignedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestUnassignedPullRequestBase {
    pub label: Option<String>,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: PullRequestReviewCommentCreatedPullRequestBaseRepo,
    pub sha: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestUnassignedPullRequestBase> for PullRequestUnassignedPullRequestBase {
    fn from(value: &PullRequestUnassignedPullRequestBase) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PullRequestUnlockedPullRequest {
    pub active_lock_reason: Option<IssueCommentCreatedIssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    pub assignee: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub assignees: Vec<Option<Untyped>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    pub auto_merge: Option<PullRequestReviewThreadUnresolvedPullRequestAutoMerge>,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changed_files: Option<i64>,
    pub closed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<i64>,
    pub comments_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commits: Option<i64>,
    pub commits_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    pub diff_url: String,
    pub draft: bool,
    pub head: PullRequestReviewCommentCreatedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<DiscussionLabeledLabel>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintainer_can_modify: Option<bool>,
    pub merge_commit_sha: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mergeable_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    pub merged_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub milestone: Option<MilestoneCreatedMilestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebaseable: Option<bool>,
    pub requested_reviewers: Vec<::serde_json::Value>,
    pub requested_teams: Vec<PullRequestReviewCommentCreatedPullRequestRequestedTeams>,
    pub review_comment_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<i64>,
    pub review_comments_url: String,
    pub state: DeploymentProtectionRuleRequestedPullRequestsState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&PullRequestUnlockedPullRequest> for PullRequestUnlockedPullRequest {
    fn from(value: &PullRequestUnlockedPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Push {
    pub after: String,
    pub base_ref: Option<String>,
    pub before: String,
    pub commits: Vec<PushDefaultCommits>,
    pub compare: String,
    pub created: bool,
    pub deleted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    pub forced: bool,
    pub head_commit: Option<PushDefaultCommits>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub pusher: PushDefaultCommitsAuthor,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Untyped,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<Untyped>,
}
impl From<&Push> for Push {
    fn from(value: &Push) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PushDefaultCommits {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<String>,
    pub author: PushDefaultCommitsAuthor,
    pub committer: PushDefaultCommitsAuthor,
    pub distinct: bool,
    pub id: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    pub timestamp: String,
    pub tree_id: String,
    pub url: String,
}
impl From<&PushDefaultCommits> for PushDefaultCommits {
    fn from(value: &PushDefaultCommits) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct PushDefaultCommitsAuthor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    pub email: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
impl From<&PushDefaultCommitsAuthor> for PushDefaultCommitsAuthor {
    fn from(value: &PushDefaultCommitsAuthor) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum RegistryPackage {
    #[serde(rename = "published")]
    Published {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        registry_package: RegistryPackagePublishedRegistryPackage,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
    #[serde(rename = "updated")]
    Updated {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        registry_package: RegistryPackageUpdatedRegistryPackage,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
    },
}
impl From<&RegistryPackage> for RegistryPackage {
    fn from(value: &RegistryPackage) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RegistryPackagePublishedRegistryPackage {
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub ecosystem: String,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub namespace: String,
    pub owner: DeploymentReviewApprovedApprover,
    pub package_type: String,
    pub package_version: Option<RegistryPackagePublishedRegistryPackagePackageVersion>,
    pub registry: Option<PackagePublishedPackageRegistry>,
    pub updated_at: Option<String>,
}
impl From<&RegistryPackagePublishedRegistryPackage> for RegistryPackagePublishedRegistryPackage {
    fn from(value: &RegistryPackagePublishedRegistryPackage) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RegistryPackagePublishedRegistryPackagePackageVersion {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<DeploymentReviewApprovedApprover>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<RegistryPackagePublishedRegistryPackagePackageVersionBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_metadata: Option<PackagePublishedPackagePackageVersionContainerMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub docker_metadata: Vec<PackagePublishedPackagePackageVersionDockerMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub html_url: String,
    pub id: i64,
    pub installation_command: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    pub metadata: Vec<Untyped>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npm_metadata: Option<RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nuget_metadata:
        Option<Vec<RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadata>>,
    pub package_files: Vec<PackagePublishedPackagePackageVersionPackageFiles>,
    pub package_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<RegistryPackagePublishedRegistryPackagePackageVersionRelease>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rubygems_metadata: Vec<PackagePublishedPackagePackageVersionRubygemsMetadata>,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_commitish: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_oid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub version: String,
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersion>
    for RegistryPackagePublishedRegistryPackagePackageVersion
{
    fn from(value: &RegistryPackagePublishedRegistryPackagePackageVersion) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RegistryPackagePublishedRegistryPackagePackageVersionBody {
    Variant0(Untyped),
    Variant1(String),
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersionBody>
    for RegistryPackagePublishedRegistryPackagePackageVersionBody
{
    fn from(value: &RegistryPackagePublishedRegistryPackagePackageVersionBody) -> Self {
        value.clone()
    }
}
impl From<Untyped> for RegistryPackagePublishedRegistryPackagePackageVersionBody {
    fn from(value: Untyped) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataAuthor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bin: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bugs: Option<RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataBugs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_oid: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributors: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cpu: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_by_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dev_dependencies: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directories:
        Option<RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDirectories>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dist: Option<RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDist>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engines: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_head: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_shrinkwrap: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation_command: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub maintainers: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub man: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npm_user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub npm_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional_dependencies: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub os: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_dependencies: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_via_actions: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository:
        Option<RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataRepository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scripts: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadata>
    for RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadata
{
    fn from(value: &RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadata) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataAuthor {
    Variant0(Untyped),
    Variant1(String),
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataAuthor>
    for RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataAuthor
{
    fn from(
        value: &RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataAuthor,
    ) -> Self {
        value.clone()
    }
}
impl From<Untyped> for RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataAuthor {
    fn from(value: Untyped) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataBugs {
    Variant0(Untyped),
    Variant1(String),
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataBugs>
    for RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataBugs
{
    fn from(value: &RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataBugs) -> Self {
        value.clone()
    }
}
impl From<Untyped> for RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataBugs {
    fn from(value: Untyped) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDirectories {
    Variant0(Untyped),
    Variant1(String),
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDirectories>
    for RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDirectories
{
    fn from(
        value: &RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDirectories,
    ) -> Self {
        value.clone()
    }
}
impl From<Untyped> for RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDirectories {
    fn from(value: Untyped) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDist {
    Variant0(Untyped),
    Variant1(String),
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDist>
    for RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDist
{
    fn from(value: &RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDist) -> Self {
        value.clone()
    }
}
impl From<Untyped> for RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataDist {
    fn from(value: Untyped) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataRepository {
    Variant0(Untyped),
    Variant1(String),
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataRepository>
    for RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataRepository
{
    fn from(
        value: &RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataRepository,
    ) -> Self {
        value.clone()
    }
}
impl From<Untyped> for RegistryPackagePublishedRegistryPackagePackageVersionNpmMetadataRepository {
    fn from(value: Untyped) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataValue>,
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadata>
    for RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadata
{
    fn from(value: &RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadata) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataId {
    Variant0(String),
    Variant1(i64),
    Variant2(Untyped),
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataId>
    for RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataId
{
    fn from(value: &RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataId) -> Self {
        value.clone()
    }
}
impl From<i64> for RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
impl From<Untyped> for RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataId {
    fn from(value: Untyped) -> Self {
        Self::Variant2(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataValue {
    Variant0(bool),
    Variant1(String),
    Variant2(i64),
    Variant3(Untyped),
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataValue>
    for RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataValue
{
    fn from(
        value: &RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataValue,
    ) -> Self {
        value.clone()
    }
}
impl From<bool> for RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataValue {
    fn from(value: bool) -> Self {
        Self::Variant0(value)
    }
}
impl From<i64> for RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataValue {
    fn from(value: i64) -> Self {
        Self::Variant2(value)
    }
}
impl From<Untyped> for RegistryPackagePublishedRegistryPackagePackageVersionNugetMetadataValue {
    fn from(value: Untyped) -> Self {
        Self::Variant3(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RegistryPackagePublishedRegistryPackagePackageVersionRelease {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<DeploymentReviewApprovedApprover>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_commitish: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&RegistryPackagePublishedRegistryPackagePackageVersionRelease>
    for RegistryPackagePublishedRegistryPackagePackageVersionRelease
{
    fn from(value: &RegistryPackagePublishedRegistryPackagePackageVersionRelease) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RegistryPackageUpdatedRegistryPackage {
    pub created_at: String,
    pub description: (),
    pub ecosystem: String,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub namespace: String,
    pub owner: DeploymentReviewApprovedApprover,
    pub package_type: String,
    pub package_version: RegistryPackageUpdatedRegistryPackagePackageVersion,
    pub registry: Option<Untyped>,
    pub updated_at: String,
}
impl From<&RegistryPackageUpdatedRegistryPackage> for RegistryPackageUpdatedRegistryPackage {
    fn from(value: &RegistryPackageUpdatedRegistryPackage) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RegistryPackageUpdatedRegistryPackagePackageVersion {
    pub author: DeploymentReviewApprovedApprover,
    pub body: String,
    pub body_html: String,
    pub created_at: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub docker_metadata: Vec<Option<Untyped>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub html_url: String,
    pub id: i64,
    pub installation_command: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    pub metadata: Vec<Untyped>,
    pub name: String,
    pub package_files: Vec<PackageUpdatedPackagePackageVersionPackageFiles>,
    pub package_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prerelease: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release: Option<RegistryPackageUpdatedRegistryPackagePackageVersionRelease>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rubygems_metadata: Vec<PackagePublishedPackagePackageVersionRubygemsMetadata>,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    pub target_commitish: String,
    pub target_oid: String,
    pub updated_at: String,
    pub version: String,
}
impl From<&RegistryPackageUpdatedRegistryPackagePackageVersion>
    for RegistryPackageUpdatedRegistryPackagePackageVersion
{
    fn from(value: &RegistryPackageUpdatedRegistryPackagePackageVersion) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RegistryPackageUpdatedRegistryPackagePackageVersionRelease {
    pub author: DeploymentReviewApprovedApprover,
    pub created_at: String,
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub prerelease: bool,
    pub published_at: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub url: String,
}
impl From<&RegistryPackageUpdatedRegistryPackagePackageVersionRelease>
    for RegistryPackageUpdatedRegistryPackagePackageVersionRelease
{
    fn from(value: &RegistryPackageUpdatedRegistryPackagePackageVersionRelease) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Release {
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        release: ReleaseCreatedRelease,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        release: ReleaseCreatedRelease,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: ReleaseEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        release: ReleaseCreatedRelease,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "prereleased")]
    Prereleased {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        release: ReleasePrereleasedRelease,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "published")]
    Published {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        release: ReleasePublishedRelease,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "released")]
    Released {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        release: ReleaseCreatedRelease,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "unpublished")]
    Unpublished {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        release: ReleasePublishedRelease,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
}
impl From<&Release> for Release {
    fn from(value: &Release) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ReleaseCreatedRelease {
    pub assets: Vec<ReleaseCreatedReleaseAssets>,
    pub assets_url: String,
    pub author: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub body: Option<String>,
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discussion_url: Option<String>,
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: Option<String>,
    pub node_id: String,
    pub prerelease: bool,
    pub published_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<DiscussionAnsweredAnswerReactions>,
    pub tag_name: String,
    pub tarball_url: Option<String>,
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: Option<String>,
}
impl From<&ReleaseCreatedRelease> for ReleaseCreatedRelease {
    fn from(value: &ReleaseCreatedRelease) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ReleaseCreatedReleaseAssets {
    pub browser_download_url: String,
    pub content_type: String,
    pub created_at: String,
    pub download_count: i64,
    pub id: i64,
    pub label: Option<String>,
    pub name: String,
    pub node_id: String,
    pub size: i64,
    pub state: String,
    pub updated_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploader: Option<DeploymentReviewApprovedReviewersReviewer>,
    pub url: String,
}
impl From<&ReleaseCreatedReleaseAssets> for ReleaseCreatedReleaseAssets {
    fn from(value: &ReleaseCreatedReleaseAssets) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ReleaseEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub make_latest: Option<ReleaseEditedChangesMakeLatest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<DiscussionEditedChangesBody>,
}
impl From<&ReleaseEditedChanges> for ReleaseEditedChanges {
    fn from(value: &ReleaseEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ReleaseEditedChangesMakeLatest {
    pub to: bool,
}
impl From<&ReleaseEditedChangesMakeLatest> for ReleaseEditedChangesMakeLatest {
    fn from(value: &ReleaseEditedChangesMakeLatest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ReleasePrereleasedRelease {
    pub assets: Vec<Option<Untyped>>,
    pub assets_url: String,
    pub author: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub body: Option<String>,
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discussion_url: Option<String>,
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: Option<String>,
    pub node_id: String,
    pub prerelease: bool,
    pub published_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<DiscussionAnsweredAnswerReactions>,
    pub tag_name: String,
    pub tarball_url: Option<String>,
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: Option<String>,
}
impl From<&ReleasePrereleasedRelease> for ReleasePrereleasedRelease {
    fn from(value: &ReleasePrereleasedRelease) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ReleasePublishedRelease {
    pub assets: Vec<Option<Untyped>>,
    pub assets_url: String,
    pub author: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub body: Option<String>,
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discussion_url: Option<String>,
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: Option<String>,
    pub node_id: String,
    pub prerelease: bool,
    pub published_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<DiscussionAnsweredAnswerReactions>,
    pub tag_name: String,
    pub tarball_url: Option<String>,
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: Option<String>,
}
impl From<&ReleasePublishedRelease> for ReleasePublishedRelease {
    fn from(value: &ReleasePublishedRelease) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Repository {
    #[serde(rename = "archived")]
    Archived {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: RepositoryEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "privatized")]
    Privatized {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "publicized")]
    Publicized {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "renamed")]
    Renamed {
        changes: RepositoryRenamedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "transferred")]
    Transferred {
        changes: Box<RepositoryTransferredChanges>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
    #[serde(rename = "unarchived")]
    Unarchived {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&Repository> for Repository {
    fn from(value: &Repository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum RepositoryAdvisory {
    #[serde(rename = "published")]
    Published {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        repository_advisory: RepositoryAdvisoryPublishedRepositoryAdvisory,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "reported")]
    Reported {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        repository_advisory: RepositoryAdvisoryPublishedRepositoryAdvisory,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
}
impl From<&RepositoryAdvisory> for RepositoryAdvisory {
    fn from(value: &RepositoryAdvisory) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisory {
    pub author: Option<DiscussionTransferredChangesNewRepositoryOrganization>,
    pub closed_at: Option<String>,
    pub collaborating_teams:
        Option<Vec<RepositoryAdvisoryPublishedRepositoryAdvisoryCollaboratingTeams>>,
    pub collaborating_users: Option<Vec<DiscussionTransferredChangesNewRepositoryOrganization>>,
    pub created_at: Option<String>,
    pub credits: Option<Vec<RepositoryAdvisoryPublishedRepositoryAdvisoryCredits>>,
    pub credits_detailed: Option<Vec<RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailed>>,
    pub cve_id: Option<String>,
    pub cvss: Option<RepositoryAdvisoryPublishedRepositoryAdvisoryCvss>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvss_severities: Option<RepositoryAdvisoryPublishedRepositoryAdvisoryCvssSeverities>,
    pub cwe_ids: Option<Vec<String>>,
    pub cwes: Option<Vec<RepositoryAdvisoryPublishedRepositoryAdvisoryCwes>>,
    pub description: Option<String>,
    pub ghsa_id: String,
    pub html_url: String,
    pub identifiers: Vec<RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiers>,
    pub private_fork: Option<RepositoryAdvisoryPublishedRepositoryAdvisoryPrivateFork>,
    pub published_at: Option<String>,
    pub publisher: Option<DiscussionTransferredChangesNewRepositoryOrganization>,
    pub severity: Option<RepositoryAdvisoryPublishedRepositoryAdvisorySeverity>,
    pub state: RepositoryAdvisoryPublishedRepositoryAdvisoryState,
    pub submission: Option<RepositoryAdvisoryPublishedRepositoryAdvisorySubmission>,
    pub summary: String,
    pub updated_at: Option<String>,
    pub url: String,
    pub vulnerabilities: Option<Vec<RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilities>>,
    pub withdrawn_at: Option<String>,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisory>
    for RepositoryAdvisoryPublishedRepositoryAdvisory
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisory) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisoryCollaboratingTeams {
    pub description: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub members_url: String,
    pub name: String,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<String>,
    pub parent: DeploymentProtectionRuleRequestedPullRequestsRequestedTeams,
    pub permission: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions:
        Option<RepositoryAdvisoryPublishedRepositoryAdvisoryCollaboratingTeamsPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<String>,
    pub repositories_url: String,
    pub slug: String,
    pub url: String,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryCollaboratingTeams>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCollaboratingTeams
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryCollaboratingTeams) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisoryCollaboratingTeamsPermissions {
    pub admin: bool,
    pub maintain: bool,
    pub pull: bool,
    pub push: bool,
    pub triage: bool,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryCollaboratingTeamsPermissions>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCollaboratingTeamsPermissions
{
    fn from(
        value: &RepositoryAdvisoryPublishedRepositoryAdvisoryCollaboratingTeamsPermissions,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisoryCredits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsType>,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryCredits>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCredits
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryCredits) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailed {
    pub state: RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailedState,
    #[serde(rename = "type")]
    pub type_: RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsType,
    pub user: DiscussionTransferredChangesNewRepositoryOrganization,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailed>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailed
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailed) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailedState {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "declined")]
    Declined,
    #[serde(rename = "pending")]
    Pending,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailedState>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailedState
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailedState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailedState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Accepted => write!(f, "accepted"),
            Self::Declined => write!(f, "declined"),
            Self::Pending => write!(f, "pending"),
        }
    }
}
impl std::str::FromStr for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailedState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "accepted" => Ok(Self::Accepted),
            "declined" => Ok(Self::Declined),
            "pending" => Ok(Self::Pending),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailedState
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailedState
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsDetailedState
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsType {
    #[serde(rename = "analyst")]
    Analyst,
    #[serde(rename = "coordinator")]
    Coordinator,
    #[serde(rename = "finder")]
    Finder,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "remediation_developer")]
    RemediationDeveloper,
    #[serde(rename = "remediation_reviewer")]
    RemediationReviewer,
    #[serde(rename = "remediation_verifier")]
    RemediationVerifier,
    #[serde(rename = "reporter")]
    Reporter,
    #[serde(rename = "sponsor")]
    Sponsor,
    #[serde(rename = "tool")]
    Tool,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsType>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsType
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Analyst => write!(f, "analyst"),
            Self::Coordinator => write!(f, "coordinator"),
            Self::Finder => write!(f, "finder"),
            Self::Other => write!(f, "other"),
            Self::RemediationDeveloper => write!(f, "remediation_developer"),
            Self::RemediationReviewer => write!(f, "remediation_reviewer"),
            Self::RemediationVerifier => write!(f, "remediation_verifier"),
            Self::Reporter => write!(f, "reporter"),
            Self::Sponsor => write!(f, "sponsor"),
            Self::Tool => write!(f, "tool"),
        }
    }
}
impl std::str::FromStr for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "analyst" => Ok(Self::Analyst),
            "coordinator" => Ok(Self::Coordinator),
            "finder" => Ok(Self::Finder),
            "other" => Ok(Self::Other),
            "remediation_developer" => Ok(Self::RemediationDeveloper),
            "remediation_reviewer" => Ok(Self::RemediationReviewer),
            "remediation_verifier" => Ok(Self::RemediationVerifier),
            "reporter" => Ok(Self::Reporter),
            "sponsor" => Ok(Self::Sponsor),
            "tool" => Ok(Self::Tool),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryAdvisoryPublishedRepositoryAdvisoryCreditsType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisoryCvss {
    pub score: Option<f64>,
    pub vector_string: Option<String>,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryCvss>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCvss
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryCvss) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisoryCvssSeverities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvss_v3: Option<RepositoryAdvisoryPublishedRepositoryAdvisoryCvss>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvss_v4: Option<RepositoryAdvisoryPublishedRepositoryAdvisoryCvss>,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryCvssSeverities>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCvssSeverities
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryCvssSeverities) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisoryCwes {
    pub cwe_id: String,
    pub name: String,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryCwes>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryCwes
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryCwes) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiers {
    #[serde(rename = "type")]
    pub type_: RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiersType,
    pub value: String,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiers>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiers
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiers) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiersType {
    #[serde(rename = "CVE")]
    Cve,
    #[serde(rename = "GHSA")]
    Ghsa,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiersType>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiersType
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiersType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiersType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cve => write!(f, "CVE"),
            Self::Ghsa => write!(f, "GHSA"),
        }
    }
}
impl std::str::FromStr for RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiersType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "CVE" => Ok(Self::Cve),
            "GHSA" => Ok(Self::Ghsa),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiersType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiersType
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryIdentifiersType
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisoryPrivateFork {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub description: Option<String>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub owner: DiscussionTransferredChangesNewRepositoryOrganization,
    pub private: bool,
    pub pulls_url: String,
    pub releases_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub url: String,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryPrivateFork>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryPrivateFork
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryPrivateFork) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryAdvisoryPublishedRepositoryAdvisorySeverity {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisorySeverity>
    for RepositoryAdvisoryPublishedRepositoryAdvisorySeverity
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisorySeverity) -> Self {
        *value
    }
}
impl ::std::fmt::Display for RepositoryAdvisoryPublishedRepositoryAdvisorySeverity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Critical => write!(f, "critical"),
            Self::High => write!(f, "high"),
            Self::Low => write!(f, "low"),
            Self::Medium => write!(f, "medium"),
        }
    }
}
impl std::str::FromStr for RepositoryAdvisoryPublishedRepositoryAdvisorySeverity {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "critical" => Ok(Self::Critical),
            "high" => Ok(Self::High),
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RepositoryAdvisoryPublishedRepositoryAdvisorySeverity {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryAdvisoryPublishedRepositoryAdvisorySeverity {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryAdvisoryPublishedRepositoryAdvisorySeverity {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryAdvisoryPublishedRepositoryAdvisoryState {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "triage")]
    Triage,
    #[serde(rename = "withdrawn")]
    Withdrawn,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryState>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryState
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for RepositoryAdvisoryPublishedRepositoryAdvisoryState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Closed => write!(f, "closed"),
            Self::Draft => write!(f, "draft"),
            Self::Published => write!(f, "published"),
            Self::Triage => write!(f, "triage"),
            Self::Withdrawn => write!(f, "withdrawn"),
        }
    }
}
impl std::str::FromStr for RepositoryAdvisoryPublishedRepositoryAdvisoryState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "closed" => Ok(Self::Closed),
            "draft" => Ok(Self::Draft),
            "published" => Ok(Self::Published),
            "triage" => Ok(Self::Triage),
            "withdrawn" => Ok(Self::Withdrawn),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RepositoryAdvisoryPublishedRepositoryAdvisoryState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryAdvisoryPublishedRepositoryAdvisoryState {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryAdvisoryPublishedRepositoryAdvisoryState {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisorySubmission {
    pub accepted: bool,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisorySubmission>
    for RepositoryAdvisoryPublishedRepositoryAdvisorySubmission
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisorySubmission) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilities {
    pub package: Option<RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackage>,
    pub patched_versions: Option<String>,
    pub vulnerable_functions: Option<Vec<String>>,
    pub vulnerable_version_range: Option<String>,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilities>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilities
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilities) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackage {
    pub ecosystem: RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackageEcosystem,
    pub name: Option<String>,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackage>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackage
{
    fn from(value: &RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackage) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackageEcosystem {
    #[serde(rename = "actions")]
    Actions,
    #[serde(rename = "composer")]
    Composer,
    #[serde(rename = "erlang")]
    Erlang,
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "maven")]
    Maven,
    #[serde(rename = "npm")]
    Npm,
    #[serde(rename = "nuget")]
    Nuget,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "pip")]
    Pip,
    #[serde(rename = "pub")]
    Pub,
    #[serde(rename = "rubygems")]
    Rubygems,
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "swift")]
    Swift,
}
impl From<&RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackageEcosystem>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackageEcosystem
{
    fn from(
        value: &RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackageEcosystem,
    ) -> Self {
        *value
    }
}
impl ::std::fmt::Display
    for RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackageEcosystem
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Actions => write!(f, "actions"),
            Self::Composer => write!(f, "composer"),
            Self::Erlang => write!(f, "erlang"),
            Self::Go => write!(f, "go"),
            Self::Maven => write!(f, "maven"),
            Self::Npm => write!(f, "npm"),
            Self::Nuget => write!(f, "nuget"),
            Self::Other => write!(f, "other"),
            Self::Pip => write!(f, "pip"),
            Self::Pub => write!(f, "pub"),
            Self::Rubygems => write!(f, "rubygems"),
            Self::Rust => write!(f, "rust"),
            Self::Swift => write!(f, "swift"),
        }
    }
}
impl std::str::FromStr
    for RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackageEcosystem
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "actions" => Ok(Self::Actions),
            "composer" => Ok(Self::Composer),
            "erlang" => Ok(Self::Erlang),
            "go" => Ok(Self::Go),
            "maven" => Ok(Self::Maven),
            "npm" => Ok(Self::Npm),
            "nuget" => Ok(Self::Nuget),
            "other" => Ok(Self::Other),
            "pip" => Ok(Self::Pip),
            "pub" => Ok(Self::Pub),
            "rubygems" => Ok(Self::Rubygems),
            "rust" => Ok(Self::Rust),
            "swift" => Ok(Self::Swift),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackageEcosystem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackageEcosystem
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for RepositoryAdvisoryPublishedRepositoryAdvisoryVulnerabilitiesPackageEcosystem
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryDispatch {
    pub action: String,
    pub branch: String,
    pub client_payload: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    pub installation: InstallationAttribute,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub repository: Untyped,
    pub sender: Untyped,
}
impl From<&RepositoryDispatch> for RepositoryDispatch {
    fn from(value: &RepositoryDispatch) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<RepositoryEditedChangesDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<RepositoryEditedChangesDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<RepositoryEditedChangesTopics>,
}
impl From<&RepositoryEditedChanges> for RepositoryEditedChanges {
    fn from(value: &RepositoryEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryEditedChangesDescription {
    pub from: Option<String>,
}
impl From<&RepositoryEditedChangesDescription> for RepositoryEditedChangesDescription {
    fn from(value: &RepositoryEditedChangesDescription) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryEditedChangesTopics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<String>>,
}
impl From<&RepositoryEditedChangesTopics> for RepositoryEditedChangesTopics {
    fn from(value: &RepositoryEditedChangesTopics) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRenamedChanges {
    pub repository: RepositoryRenamedChangesRepository,
}
impl From<&RepositoryRenamedChanges> for RepositoryRenamedChanges {
    fn from(value: &RepositoryRenamedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRenamedChangesRepository {
    pub name: DiscussionEditedChangesBody,
}
impl From<&RepositoryRenamedChangesRepository> for RepositoryRenamedChangesRepository {
    fn from(value: &RepositoryRenamedChangesRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum RepositoryRuleset {
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        repository_ruleset: RepositoryRulesetCreatedRepositoryRuleset,
        sender: Untyped,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        repository_ruleset: RepositoryRulesetCreatedRepositoryRuleset,
        sender: Untyped,
    },
    #[serde(rename = "edited")]
    Edited {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        changes: Option<RepositoryRulesetEditedChanges>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        repository_ruleset: RepositoryRulesetCreatedRepositoryRuleset,
        sender: Untyped,
    },
}
impl From<&RepositoryRuleset> for RepositoryRuleset {
    fn from(value: &RepositoryRuleset) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRuleset {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bypass_actors: Vec<RepositoryRulesetCreatedRepositoryRulesetBypassActors>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_user_can_bypass:
        Option<RepositoryRulesetCreatedRepositoryRulesetCurrentUserCanBypass>,
    pub enforcement: RepositoryRulesetCreatedRepositoryRulesetEnforcement,
    pub id: i64,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub links: Option<RepositoryRulesetCreatedRepositoryRulesetLinks>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<RepositoryRulesetCreatedRepositoryRulesetRules>,
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_type: Option<RepositoryRulesetCreatedRepositoryRulesetSourceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<RepositoryRulesetCreatedRepositoryRulesetTarget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
impl From<&RepositoryRulesetCreatedRepositoryRuleset>
    for RepositoryRulesetCreatedRepositoryRuleset
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRuleset) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetBypassActors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<i64>,
    pub actor_type: RepositoryRulesetCreatedRepositoryRulesetBypassActorsActorType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bypass_mode: Option<RepositoryRulesetCreatedRepositoryRulesetBypassActorsBypassMode>,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetBypassActors>
    for RepositoryRulesetCreatedRepositoryRulesetBypassActors
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetBypassActors) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryRulesetCreatedRepositoryRulesetBypassActorsActorType {
    DeployKey,
    Integration,
    OrganizationAdmin,
    RepositoryRole,
    Team,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetBypassActorsActorType>
    for RepositoryRulesetCreatedRepositoryRulesetBypassActorsActorType
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetBypassActorsActorType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for RepositoryRulesetCreatedRepositoryRulesetBypassActorsActorType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DeployKey => write!(f, "DeployKey"),
            Self::Integration => write!(f, "Integration"),
            Self::OrganizationAdmin => write!(f, "OrganizationAdmin"),
            Self::RepositoryRole => write!(f, "RepositoryRole"),
            Self::Team => write!(f, "Team"),
        }
    }
}
impl std::str::FromStr for RepositoryRulesetCreatedRepositoryRulesetBypassActorsActorType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "DeployKey" => Ok(Self::DeployKey),
            "Integration" => Ok(Self::Integration),
            "OrganizationAdmin" => Ok(Self::OrganizationAdmin),
            "RepositoryRole" => Ok(Self::RepositoryRole),
            "Team" => Ok(Self::Team),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for RepositoryRulesetCreatedRepositoryRulesetBypassActorsActorType
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for RepositoryRulesetCreatedRepositoryRulesetBypassActorsActorType
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for RepositoryRulesetCreatedRepositoryRulesetBypassActorsActorType
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(
    Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Default,
)]
pub enum RepositoryRulesetCreatedRepositoryRulesetBypassActorsBypassMode {
    #[serde(rename = "always")]
    #[default]
    Always,
    #[serde(rename = "pull_request")]
    PullRequest,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetBypassActorsBypassMode>
    for RepositoryRulesetCreatedRepositoryRulesetBypassActorsBypassMode
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetBypassActorsBypassMode) -> Self {
        *value
    }
}
impl ::std::fmt::Display for RepositoryRulesetCreatedRepositoryRulesetBypassActorsBypassMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Always => write!(f, "always"),
            Self::PullRequest => write!(f, "pull_request"),
        }
    }
}
impl std::str::FromStr for RepositoryRulesetCreatedRepositoryRulesetBypassActorsBypassMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "always" => Ok(Self::Always),
            "pull_request" => Ok(Self::PullRequest),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for RepositoryRulesetCreatedRepositoryRulesetBypassActorsBypassMode
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for RepositoryRulesetCreatedRepositoryRulesetBypassActorsBypassMode
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for RepositoryRulesetCreatedRepositoryRulesetBypassActorsBypassMode
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryRulesetCreatedRepositoryRulesetCurrentUserCanBypass {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "pull_requests_only")]
    PullRequestsOnly,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetCurrentUserCanBypass>
    for RepositoryRulesetCreatedRepositoryRulesetCurrentUserCanBypass
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetCurrentUserCanBypass) -> Self {
        *value
    }
}
impl ::std::fmt::Display for RepositoryRulesetCreatedRepositoryRulesetCurrentUserCanBypass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Always => write!(f, "always"),
            Self::Never => write!(f, "never"),
            Self::PullRequestsOnly => write!(f, "pull_requests_only"),
        }
    }
}
impl std::str::FromStr for RepositoryRulesetCreatedRepositoryRulesetCurrentUserCanBypass {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            "pull_requests_only" => Ok(Self::PullRequestsOnly),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RepositoryRulesetCreatedRepositoryRulesetCurrentUserCanBypass {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for RepositoryRulesetCreatedRepositoryRulesetCurrentUserCanBypass
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for RepositoryRulesetCreatedRepositoryRulesetCurrentUserCanBypass
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryRulesetCreatedRepositoryRulesetEnforcement {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "evaluate")]
    Evaluate,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetEnforcement>
    for RepositoryRulesetCreatedRepositoryRulesetEnforcement
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetEnforcement) -> Self {
        *value
    }
}
impl ::std::fmt::Display for RepositoryRulesetCreatedRepositoryRulesetEnforcement {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => write!(f, "active"),
            Self::Disabled => write!(f, "disabled"),
            Self::Evaluate => write!(f, "evaluate"),
        }
    }
}
impl std::str::FromStr for RepositoryRulesetCreatedRepositoryRulesetEnforcement {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "disabled" => Ok(Self::Disabled),
            "evaluate" => Ok(Self::Evaluate),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RepositoryRulesetCreatedRepositoryRulesetEnforcement {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryRulesetCreatedRepositoryRulesetEnforcement {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryRulesetCreatedRepositoryRulesetEnforcement {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html: Option<RepositoryRulesetCreatedRepositoryRulesetLinksSelf>,
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<RepositoryRulesetCreatedRepositoryRulesetLinksSelf>,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetLinks>
    for RepositoryRulesetCreatedRepositoryRulesetLinks
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetLinks) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetLinksSelf {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetLinksSelf>
    for RepositoryRulesetCreatedRepositoryRulesetLinksSelf
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetLinksSelf) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRules {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_name_pattern:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesBranchNamePattern>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_scanning: Option<RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanning>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_author_email_pattern:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesCommitAuthorEmailPattern>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_message_pattern:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePattern>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer_email_pattern:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesCommitterEmailPattern>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation: Option<RepositoryRulesetCreatedRepositoryRulesetRulesCreation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion: Option<RepositoryRulesetCreatedRepositoryRulesetRulesDeletion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_extension_restriction:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesFileExtensionRestriction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_path_restriction:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesFilePathRestriction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_file_path_length:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesMaxFilePathLength>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<RepositoryRulesetCreatedRepositoryRulesetRulesMaxFileSize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_queue: Option<RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_fast_forward: Option<RepositoryRulesetCreatedRepositoryRulesetRulesNonFastForward>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<RepositoryRulesetCreatedRepositoryRulesetRulesPullRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_deployments:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesRequiredDeployments>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_linear_history:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesRequiredLinearHistory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_signatures:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesRequiredSignatures>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_status_checks:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_name_pattern: Option<RepositoryRulesetCreatedRepositoryRulesetRulesTagNamePattern>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<RepositoryRulesetCreatedRepositoryRulesetRulesUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<RepositoryRulesetCreatedRepositoryRulesetRulesWorkflows>,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRules>
    for RepositoryRulesetCreatedRepositoryRulesetRules
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRules) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesBranchNamePattern {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesBranchNamePattern>
    for RepositoryRulesetCreatedRepositoryRulesetRulesBranchNamePattern
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesBranchNamePattern) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanning {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanning>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanning
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanning) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParameters {
    pub code_scanning_tools:
        Vec<RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningTools>,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParameters
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParameters) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningTools { pub alerts_threshold : RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsAlertsThreshold , pub security_alerts_threshold : RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsSecurityAlertsThreshold , # [doc = "<p>The name of a code scanning tool</p>"] pub tool : String , }
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningTools>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningTools
{
    fn from(
        value : & RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningTools,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsAlertsThreshold
{
    #[serde(rename = "all")]
    All,
    #[serde(rename = "errors")]
    Errors,
    #[serde(rename = "errors_and_warnings")]
    ErrorsAndWarnings,
    #[serde(rename = "none")]
    None,
}
impl From < & RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsAlertsThreshold > for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsAlertsThreshold { fn from (value : & RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsAlertsThreshold) -> Self { *value } }
impl :: std :: fmt :: Display for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsAlertsThreshold { fn fmt (& self , f : & mut :: std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result { match * self { Self :: All => write ! (f , "all") , Self :: Errors => write ! (f , "errors") , Self :: ErrorsAndWarnings => write ! (f , "errors_and_warnings") , Self :: None => write ! (f , "none") , } } }
impl std :: str :: FromStr for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsAlertsThreshold { type Err = self :: error :: ConversionError ; fn from_str (value : & str) -> Result < Self , self :: error :: ConversionError > { match value { "all" => Ok (Self :: All) , "errors" => Ok (Self :: Errors) , "errors_and_warnings" => Ok (Self :: ErrorsAndWarnings) , "none" => Ok (Self :: None) , _ => Err ("invalid value" . into ()) , } } }
impl std :: convert :: TryFrom < & str > for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsAlertsThreshold { type Error = self :: error :: ConversionError ; fn try_from (value : & str) -> Result < Self , self :: error :: ConversionError > { value . parse () } }
impl std :: convert :: TryFrom < & String > for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsAlertsThreshold { type Error = self :: error :: ConversionError ; fn try_from (value : & String) -> Result < Self , self :: error :: ConversionError > { value . parse () } }
impl std :: convert :: TryFrom < String > for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsAlertsThreshold { type Error = self :: error :: ConversionError ; fn try_from (value : String) -> Result < Self , self :: error :: ConversionError > { value . parse () } }
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsSecurityAlertsThreshold
{
    #[serde(rename = "all")]
    All,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "high_or_higher")]
    HighOrHigher,
    #[serde(rename = "medium_or_higher")]
    MediumOrHigher,
    #[serde(rename = "none")]
    None,
}
impl From < & RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsSecurityAlertsThreshold > for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsSecurityAlertsThreshold { fn from (value : & RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsSecurityAlertsThreshold) -> Self { *value } }
impl :: std :: fmt :: Display for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsSecurityAlertsThreshold { fn fmt (& self , f : & mut :: std :: fmt :: Formatter < '_ >) -> :: std :: fmt :: Result { match * self { Self :: All => write ! (f , "all") , Self :: Critical => write ! (f , "critical") , Self :: HighOrHigher => write ! (f , "high_or_higher") , Self :: MediumOrHigher => write ! (f , "medium_or_higher") , Self :: None => write ! (f , "none") , } } }
impl std :: str :: FromStr for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsSecurityAlertsThreshold { type Err = self :: error :: ConversionError ; fn from_str (value : & str) -> Result < Self , self :: error :: ConversionError > { match value { "all" => Ok (Self :: All) , "critical" => Ok (Self :: Critical) , "high_or_higher" => Ok (Self :: HighOrHigher) , "medium_or_higher" => Ok (Self :: MediumOrHigher) , "none" => Ok (Self :: None) , _ => Err ("invalid value" . into ()) , } } }
impl std :: convert :: TryFrom < & str > for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsSecurityAlertsThreshold { type Error = self :: error :: ConversionError ; fn try_from (value : & str) -> Result < Self , self :: error :: ConversionError > { value . parse () } }
impl std :: convert :: TryFrom < & String > for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsSecurityAlertsThreshold { type Error = self :: error :: ConversionError ; fn try_from (value : & String) -> Result < Self , self :: error :: ConversionError > { value . parse () } }
impl std :: convert :: TryFrom < String > for RepositoryRulesetCreatedRepositoryRulesetRulesCodeScanningParametersCodeScanningToolsSecurityAlertsThreshold { type Error = self :: error :: ConversionError ; fn try_from (value : String) -> Result < Self , self :: error :: ConversionError > { value . parse () } }
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesCommitAuthorEmailPattern {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesCommitAuthorEmailPattern>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCommitAuthorEmailPattern
{
    fn from(
        value: &RepositoryRulesetCreatedRepositoryRulesetRulesCommitAuthorEmailPattern,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePattern {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePattern>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePattern
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePattern) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    pub operator:
        RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParametersOperator,
    pub pattern: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParameters
{
    fn from(
        value: &RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParameters,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParametersOperator {
    #[serde(rename = "contains")]
    Contains,
    #[serde(rename = "ends_with")]
    EndsWith,
    #[serde(rename = "regex")]
    Regex,
    #[serde(rename = "starts_with")]
    StartsWith,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParametersOperator>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParametersOperator
{
    fn from(
        value : & RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParametersOperator,
    ) -> Self {
        *value
    }
}
impl ::std::fmt::Display
    for RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParametersOperator
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Contains => write!(f, "contains"),
            Self::EndsWith => write!(f, "ends_with"),
            Self::Regex => write!(f, "regex"),
            Self::StartsWith => write!(f, "starts_with"),
        }
    }
}
impl std::str::FromStr
    for RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParametersOperator
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "contains" => Ok(Self::Contains),
            "ends_with" => Ok(Self::EndsWith),
            "regex" => Ok(Self::Regex),
            "starts_with" => Ok(Self::StartsWith),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParametersOperator
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParametersOperator
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParametersOperator
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesCommitterEmailPattern {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesCommitterEmailPattern>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCommitterEmailPattern
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesCommitterEmailPattern) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesCreation {
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesCreation>
    for RepositoryRulesetCreatedRepositoryRulesetRulesCreation
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesCreation) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesDeletion {
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesDeletion>
    for RepositoryRulesetCreatedRepositoryRulesetRulesDeletion
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesDeletion) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesFileExtensionRestriction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesFileExtensionRestrictionParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesFileExtensionRestriction>
    for RepositoryRulesetCreatedRepositoryRulesetRulesFileExtensionRestriction
{
    fn from(
        value: &RepositoryRulesetCreatedRepositoryRulesetRulesFileExtensionRestriction,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesFileExtensionRestrictionParameters {
    pub restricted_file_extensions: Vec<String>,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesFileExtensionRestrictionParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesFileExtensionRestrictionParameters
{
    fn from(
        value: &RepositoryRulesetCreatedRepositoryRulesetRulesFileExtensionRestrictionParameters,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesFilePathRestriction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesFilePathRestrictionParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesFilePathRestriction>
    for RepositoryRulesetCreatedRepositoryRulesetRulesFilePathRestriction
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesFilePathRestriction) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesFilePathRestrictionParameters {
    pub restricted_file_paths: Vec<String>,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesFilePathRestrictionParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesFilePathRestrictionParameters
{
    fn from(
        value: &RepositoryRulesetCreatedRepositoryRulesetRulesFilePathRestrictionParameters,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesMaxFilePathLength {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesMaxFilePathLengthParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesMaxFilePathLength>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMaxFilePathLength
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesMaxFilePathLength) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesMaxFilePathLengthParameters {
    pub max_file_path_length: i64,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesMaxFilePathLengthParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMaxFilePathLengthParameters
{
    fn from(
        value: &RepositoryRulesetCreatedRepositoryRulesetRulesMaxFilePathLengthParameters,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesMaxFileSize {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<RepositoryRulesetCreatedRepositoryRulesetRulesMaxFileSizeParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesMaxFileSize>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMaxFileSize
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesMaxFileSize) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesMaxFileSizeParameters {
    pub max_file_size: i64,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesMaxFileSizeParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMaxFileSizeParameters
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesMaxFileSizeParameters) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueue>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueue
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParameters {
    pub check_response_timeout_minutes: i64,
    pub grouping_strategy:
        RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersGroupingStrategy,
    pub max_entries_to_build: i64,
    pub max_entries_to_merge: i64,
    pub merge_method: RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersMergeMethod,
    pub min_entries_to_merge: i64,
    pub min_entries_to_merge_wait_minutes: i64,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParameters
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParameters) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersGroupingStrategy {
    #[serde(rename = "ALLGREEN")]
    Allgreen,
    #[serde(rename = "HEADGREEN")]
    Headgreen,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersGroupingStrategy>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersGroupingStrategy
{
    fn from(
        value: &RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersGroupingStrategy,
    ) -> Self {
        *value
    }
}
impl ::std::fmt::Display
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersGroupingStrategy
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Allgreen => write!(f, "ALLGREEN"),
            Self::Headgreen => write!(f, "HEADGREEN"),
        }
    }
}
impl std::str::FromStr
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersGroupingStrategy
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "ALLGREEN" => Ok(Self::Allgreen),
            "HEADGREEN" => Ok(Self::Headgreen),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersGroupingStrategy
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersGroupingStrategy
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersGroupingStrategy
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersMergeMethod {
    #[serde(rename = "MERGE")]
    Merge,
    #[serde(rename = "REBASE")]
    Rebase,
    #[serde(rename = "SQUASH")]
    Squash,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersMergeMethod>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersMergeMethod
{
    fn from(
        value: &RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersMergeMethod,
    ) -> Self {
        *value
    }
}
impl ::std::fmt::Display
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersMergeMethod
{
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Merge => write!(f, "MERGE"),
            Self::Rebase => write!(f, "REBASE"),
            Self::Squash => write!(f, "SQUASH"),
        }
    }
}
impl std::str::FromStr
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersMergeMethod
{
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "MERGE" => Ok(Self::Merge),
            "REBASE" => Ok(Self::Rebase),
            "SQUASH" => Ok(Self::Squash),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersMergeMethod
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersMergeMethod
{
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
    for RepositoryRulesetCreatedRepositoryRulesetRulesMergeQueueParametersMergeMethod
{
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesNonFastForward {
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesNonFastForward>
    for RepositoryRulesetCreatedRepositoryRulesetRulesNonFastForward
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesNonFastForward) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesPullRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<RepositoryRulesetCreatedRepositoryRulesetRulesPullRequestParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesPullRequest>
    for RepositoryRulesetCreatedRepositoryRulesetRulesPullRequest
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesPullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesPullRequestParameters {
    pub dismiss_stale_reviews_on_push: bool,
    pub require_code_owner_review: bool,
    pub require_last_push_approval: bool,
    pub required_approving_review_count: i64,
    pub required_review_thread_resolution: bool,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesPullRequestParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesPullRequestParameters
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesPullRequestParameters) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesRequiredDeployments {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesRequiredDeploymentsParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesRequiredDeployments>
    for RepositoryRulesetCreatedRepositoryRulesetRulesRequiredDeployments
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesRequiredDeployments) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesRequiredDeploymentsParameters {
    pub required_deployment_environments: Vec<String>,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesRequiredDeploymentsParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesRequiredDeploymentsParameters
{
    fn from(
        value: &RepositoryRulesetCreatedRepositoryRulesetRulesRequiredDeploymentsParameters,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesRequiredLinearHistory {
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesRequiredLinearHistory>
    for RepositoryRulesetCreatedRepositoryRulesetRulesRequiredLinearHistory
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesRequiredLinearHistory) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesRequiredSignatures {
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesRequiredSignatures>
    for RepositoryRulesetCreatedRepositoryRulesetRulesRequiredSignatures
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesRequiredSignatures) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecksParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecks>
    for RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecks
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecks) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecksParameters { # [doc = "<p>Allow repositories and branches to be created if a check would otherwise prohibit it.</p>"] # [serde (default , skip_serializing_if = "Option::is_none")] pub do_not_enforce_on_create : Option < bool > , # [doc = "<p>Status checks that are required.</p>"] pub required_status_checks : Vec < RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecksParametersRequiredStatusChecks > , # [doc = "<p>Whether pull requests targeting a matching branch must be tested with the latest code. This setting will not take effect unless at least one status check is enabled.</p>"] pub strict_required_status_checks_policy : bool , }
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecksParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecksParameters
{
    fn from(
        value: &RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecksParameters,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecksParametersRequiredStatusChecks
{
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<i64>,
}
impl From < & RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecksParametersRequiredStatusChecks > for RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecksParametersRequiredStatusChecks { fn from (value : & RepositoryRulesetCreatedRepositoryRulesetRulesRequiredStatusChecksParametersRequiredStatusChecks) -> Self { value . clone () } }
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesTagNamePattern {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters:
        Option<RepositoryRulesetCreatedRepositoryRulesetRulesCommitMessagePatternParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesTagNamePattern>
    for RepositoryRulesetCreatedRepositoryRulesetRulesTagNamePattern
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesTagNamePattern) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<RepositoryRulesetCreatedRepositoryRulesetRulesUpdateParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesUpdate>
    for RepositoryRulesetCreatedRepositoryRulesetRulesUpdate
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesUpdate) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesUpdateParameters {
    pub update_allows_fetch_and_merge: bool,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesUpdateParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesUpdateParameters
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesUpdateParameters) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesWorkflows {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<RepositoryRulesetCreatedRepositoryRulesetRulesWorkflowsParameters>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesWorkflows>
    for RepositoryRulesetCreatedRepositoryRulesetRulesWorkflows
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesWorkflows) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesWorkflowsParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub do_not_enforce_on_create: Option<bool>,
    pub workflows: Vec<RepositoryRulesetCreatedRepositoryRulesetRulesWorkflowsParametersWorkflows>,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesWorkflowsParameters>
    for RepositoryRulesetCreatedRepositoryRulesetRulesWorkflowsParameters
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetRulesWorkflowsParameters) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetCreatedRepositoryRulesetRulesWorkflowsParametersWorkflows {
    pub path: String,
    #[serde(rename = "ref", default, skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    pub repository_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetRulesWorkflowsParametersWorkflows>
    for RepositoryRulesetCreatedRepositoryRulesetRulesWorkflowsParametersWorkflows
{
    fn from(
        value: &RepositoryRulesetCreatedRepositoryRulesetRulesWorkflowsParametersWorkflows,
    ) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryRulesetCreatedRepositoryRulesetSourceType {
    Organization,
    Repository,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetSourceType>
    for RepositoryRulesetCreatedRepositoryRulesetSourceType
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetSourceType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for RepositoryRulesetCreatedRepositoryRulesetSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Organization => write!(f, "Organization"),
            Self::Repository => write!(f, "Repository"),
        }
    }
}
impl std::str::FromStr for RepositoryRulesetCreatedRepositoryRulesetSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "Organization" => Ok(Self::Organization),
            "Repository" => Ok(Self::Repository),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RepositoryRulesetCreatedRepositoryRulesetSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryRulesetCreatedRepositoryRulesetSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryRulesetCreatedRepositoryRulesetSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RepositoryRulesetCreatedRepositoryRulesetTarget {
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "tag")]
    Tag,
}
impl From<&RepositoryRulesetCreatedRepositoryRulesetTarget>
    for RepositoryRulesetCreatedRepositoryRulesetTarget
{
    fn from(value: &RepositoryRulesetCreatedRepositoryRulesetTarget) -> Self {
        *value
    }
}
impl ::std::fmt::Display for RepositoryRulesetCreatedRepositoryRulesetTarget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Branch => write!(f, "branch"),
            Self::Push => write!(f, "push"),
            Self::Tag => write!(f, "tag"),
        }
    }
}
impl std::str::FromStr for RepositoryRulesetCreatedRepositoryRulesetTarget {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "branch" => Ok(Self::Branch),
            "push" => Ok(Self::Push),
            "tag" => Ok(Self::Tag),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for RepositoryRulesetCreatedRepositoryRulesetTarget {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryRulesetCreatedRepositoryRulesetTarget {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryRulesetCreatedRepositoryRulesetTarget {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<RepositoryRulesetEditedChangesConditions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforcement: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<RepositoryRulesetEditedChangesRules>,
}
impl From<&RepositoryRulesetEditedChanges> for RepositoryRulesetEditedChanges {
    fn from(value: &RepositoryRulesetEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetEditedChangesConditions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<RepositoryRulesetEditedChangesConditionsAdded>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deleted: Vec<RepositoryRulesetEditedChangesConditionsAdded>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub updated: Vec<RepositoryRulesetEditedChangesConditionsUpdated>,
}
impl From<&RepositoryRulesetEditedChangesConditions> for RepositoryRulesetEditedChangesConditions {
    fn from(value: &RepositoryRulesetEditedChangesConditions) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetEditedChangesConditionsAdded {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_name: Option<RepositoryRulesetEditedChangesConditionsAddedRefName>,
}
impl From<&RepositoryRulesetEditedChangesConditionsAdded>
    for RepositoryRulesetEditedChangesConditionsAdded
{
    fn from(value: &RepositoryRulesetEditedChangesConditionsAdded) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetEditedChangesConditionsAddedRefName {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
}
impl From<&RepositoryRulesetEditedChangesConditionsAddedRefName>
    for RepositoryRulesetEditedChangesConditionsAddedRefName
{
    fn from(value: &RepositoryRulesetEditedChangesConditionsAddedRefName) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetEditedChangesConditionsUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<RepositoryRulesetEditedChangesConditionsUpdatedChanges>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<RepositoryRulesetEditedChangesConditionsAdded>,
}
impl From<&RepositoryRulesetEditedChangesConditionsUpdated>
    for RepositoryRulesetEditedChangesConditionsUpdated
{
    fn from(value: &RepositoryRulesetEditedChangesConditionsUpdated) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetEditedChangesConditionsUpdatedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<RepositoryRulesetEditedChangesConditionsUpdatedChangesInclude>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<RepositoryRulesetEditedChangesConditionsUpdatedChangesInclude>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<DiscussionEditedChangesBody>,
}
impl From<&RepositoryRulesetEditedChangesConditionsUpdatedChanges>
    for RepositoryRulesetEditedChangesConditionsUpdatedChanges
{
    fn from(value: &RepositoryRulesetEditedChangesConditionsUpdatedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetEditedChangesConditionsUpdatedChangesInclude {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub from: Vec<String>,
}
impl From<&RepositoryRulesetEditedChangesConditionsUpdatedChangesInclude>
    for RepositoryRulesetEditedChangesConditionsUpdatedChangesInclude
{
    fn from(value: &RepositoryRulesetEditedChangesConditionsUpdatedChangesInclude) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetEditedChangesRules {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<RepositoryRulesetCreatedRepositoryRulesetRules>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deleted: Vec<RepositoryRulesetCreatedRepositoryRulesetRules>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub updated: Vec<RepositoryRulesetEditedChangesRulesUpdated>,
}
impl From<&RepositoryRulesetEditedChangesRules> for RepositoryRulesetEditedChangesRules {
    fn from(value: &RepositoryRulesetEditedChangesRules) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetEditedChangesRulesUpdated {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<RepositoryRulesetEditedChangesRulesUpdatedChanges>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<RepositoryRulesetCreatedRepositoryRulesetRules>,
}
impl From<&RepositoryRulesetEditedChangesRulesUpdated>
    for RepositoryRulesetEditedChangesRulesUpdated
{
    fn from(value: &RepositoryRulesetEditedChangesRulesUpdated) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryRulesetEditedChangesRulesUpdatedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<DiscussionEditedChangesBody>,
}
impl From<&RepositoryRulesetEditedChangesRulesUpdatedChanges>
    for RepositoryRulesetEditedChangesRulesUpdatedChanges
{
    fn from(value: &RepositoryRulesetEditedChangesRulesUpdatedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryTransferredChanges {
    pub owner: RepositoryTransferredChangesOwner,
}
impl From<&RepositoryTransferredChanges> for RepositoryTransferredChanges {
    fn from(value: &RepositoryTransferredChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryTransferredChangesOwner {
    pub from: RepositoryTransferredChangesOwnerFrom,
}
impl From<&RepositoryTransferredChangesOwner> for RepositoryTransferredChangesOwner {
    fn from(value: &RepositoryTransferredChangesOwner) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryTransferredChangesOwnerFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<RepositoryTransferredChangesOwnerFromOrganization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<DeploymentStatusCreatedDeploymentCreator>,
}
impl From<&RepositoryTransferredChangesOwnerFrom> for RepositoryTransferredChangesOwnerFrom {
    fn from(value: &RepositoryTransferredChangesOwnerFrom) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct RepositoryTransferredChangesOwnerFromOrganization {
    pub avatar_url: String,
    pub description: Option<String>,
    pub events_url: String,
    pub hooks_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub id: i64,
    pub issues_url: String,
    pub login: String,
    pub members_url: String,
    pub node_id: String,
    pub public_members_url: String,
    pub repos_url: String,
    pub url: String,
}
impl From<&RepositoryTransferredChangesOwnerFromOrganization>
    for RepositoryTransferredChangesOwnerFromOrganization
{
    fn from(value: &RepositoryTransferredChangesOwnerFromOrganization) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum SecretScanningAlert {
    #[serde(rename = "created")]
    Created {
        alert: SecretScanningAlertLocationCreatedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "publicly_leaked")]
    PubliclyLeaked {
        alert: SecretScanningAlertLocationCreatedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "reopened")]
    Reopened {
        alert: SecretScanningAlertLocationCreatedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "resolved")]
    Resolved {
        alert: SecretScanningAlertLocationCreatedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "validated")]
    Validated {
        alert: SecretScanningAlertLocationCreatedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
}
impl From<&SecretScanningAlert> for SecretScanningAlert {
    fn from(value: &SecretScanningAlert) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SecretScanningAlertLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    pub alert: SecretScanningAlertLocationCreatedAlert,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    pub location: SecretScanningAlertLocationCreatedLocation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub repository: Untyped,
    pub sender: Untyped,
}
impl From<&SecretScanningAlertLocation> for SecretScanningAlertLocation {
    fn from(value: &SecretScanningAlertLocation) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SecretScanningAlertLocationCreatedAlert {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_repo: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publicly_leaked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_protection_bypassed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_protection_bypassed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push_protection_bypassed_by: Option<DiscussionTransferredChangesNewRepositoryOrganization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<SecretScanningAlertLocationCreatedAlertResolution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution_comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<DiscussionTransferredChangesNewRepositoryOrganization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_type_display_name: Option<String>,
    #[serde(default)]
    pub updated_at: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validity: Option<SecretScanningAlertLocationCreatedAlertValidity>,
}
impl From<&SecretScanningAlertLocationCreatedAlert> for SecretScanningAlertLocationCreatedAlert {
    fn from(value: &SecretScanningAlertLocationCreatedAlert) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SecretScanningAlertLocationCreatedAlertResolution {
    #[serde(rename = "false_positive")]
    FalsePositive,
    #[serde(rename = "pattern_deleted")]
    PatternDeleted,
    #[serde(rename = "pattern_edited")]
    PatternEdited,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "used_in_tests")]
    UsedInTests,
    #[serde(rename = "wont_fix")]
    WontFix,
}
impl From<&SecretScanningAlertLocationCreatedAlertResolution>
    for SecretScanningAlertLocationCreatedAlertResolution
{
    fn from(value: &SecretScanningAlertLocationCreatedAlertResolution) -> Self {
        *value
    }
}
impl ::std::fmt::Display for SecretScanningAlertLocationCreatedAlertResolution {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FalsePositive => write!(f, "false_positive"),
            Self::PatternDeleted => write!(f, "pattern_deleted"),
            Self::PatternEdited => write!(f, "pattern_edited"),
            Self::Revoked => write!(f, "revoked"),
            Self::UsedInTests => write!(f, "used_in_tests"),
            Self::WontFix => write!(f, "wont_fix"),
        }
    }
}
impl std::str::FromStr for SecretScanningAlertLocationCreatedAlertResolution {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "false_positive" => Ok(Self::FalsePositive),
            "pattern_deleted" => Ok(Self::PatternDeleted),
            "pattern_edited" => Ok(Self::PatternEdited),
            "revoked" => Ok(Self::Revoked),
            "used_in_tests" => Ok(Self::UsedInTests),
            "wont_fix" => Ok(Self::WontFix),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for SecretScanningAlertLocationCreatedAlertResolution {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SecretScanningAlertLocationCreatedAlertResolution {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SecretScanningAlertLocationCreatedAlertResolution {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SecretScanningAlertLocationCreatedAlertValidity {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "unknown")]
    Unknown,
}
impl From<&SecretScanningAlertLocationCreatedAlertValidity>
    for SecretScanningAlertLocationCreatedAlertValidity
{
    fn from(value: &SecretScanningAlertLocationCreatedAlertValidity) -> Self {
        *value
    }
}
impl ::std::fmt::Display for SecretScanningAlertLocationCreatedAlertValidity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}
impl std::str::FromStr for SecretScanningAlertLocationCreatedAlertValidity {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for SecretScanningAlertLocationCreatedAlertValidity {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SecretScanningAlertLocationCreatedAlertValidity {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SecretScanningAlertLocationCreatedAlertValidity {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SecretScanningAlertLocationCreatedLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Untyped>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<SecretScanningAlertLocationCreatedLocationType>,
}
impl From<&SecretScanningAlertLocationCreatedLocation>
    for SecretScanningAlertLocationCreatedLocation
{
    fn from(value: &SecretScanningAlertLocationCreatedLocation) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SecretScanningAlertLocationCreatedLocationType {
    #[serde(rename = "commit")]
    Commit,
    #[serde(rename = "discussion_body")]
    DiscussionBody,
    #[serde(rename = "discussion_comment")]
    DiscussionComment,
    #[serde(rename = "discussion_title")]
    DiscussionTitle,
    #[serde(rename = "issue_body")]
    IssueBody,
    #[serde(rename = "issue_comment")]
    IssueComment,
    #[serde(rename = "issue_title")]
    IssueTitle,
    #[serde(rename = "pull_request_body")]
    PullRequestBody,
    #[serde(rename = "pull_request_comment")]
    PullRequestComment,
    #[serde(rename = "pull_request_review")]
    PullRequestReview,
    #[serde(rename = "pull_request_review_comment")]
    PullRequestReviewComment,
    #[serde(rename = "pull_request_title")]
    PullRequestTitle,
    #[serde(rename = "wiki_commit")]
    WikiCommit,
}
impl From<&SecretScanningAlertLocationCreatedLocationType>
    for SecretScanningAlertLocationCreatedLocationType
{
    fn from(value: &SecretScanningAlertLocationCreatedLocationType) -> Self {
        *value
    }
}
impl ::std::fmt::Display for SecretScanningAlertLocationCreatedLocationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Commit => write!(f, "commit"),
            Self::DiscussionBody => write!(f, "discussion_body"),
            Self::DiscussionComment => write!(f, "discussion_comment"),
            Self::DiscussionTitle => write!(f, "discussion_title"),
            Self::IssueBody => write!(f, "issue_body"),
            Self::IssueComment => write!(f, "issue_comment"),
            Self::IssueTitle => write!(f, "issue_title"),
            Self::PullRequestBody => write!(f, "pull_request_body"),
            Self::PullRequestComment => write!(f, "pull_request_comment"),
            Self::PullRequestReview => write!(f, "pull_request_review"),
            Self::PullRequestReviewComment => write!(f, "pull_request_review_comment"),
            Self::PullRequestTitle => write!(f, "pull_request_title"),
            Self::WikiCommit => write!(f, "wiki_commit"),
        }
    }
}
impl std::str::FromStr for SecretScanningAlertLocationCreatedLocationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "commit" => Ok(Self::Commit),
            "discussion_body" => Ok(Self::DiscussionBody),
            "discussion_comment" => Ok(Self::DiscussionComment),
            "discussion_title" => Ok(Self::DiscussionTitle),
            "issue_body" => Ok(Self::IssueBody),
            "issue_comment" => Ok(Self::IssueComment),
            "issue_title" => Ok(Self::IssueTitle),
            "pull_request_body" => Ok(Self::PullRequestBody),
            "pull_request_comment" => Ok(Self::PullRequestComment),
            "pull_request_review" => Ok(Self::PullRequestReview),
            "pull_request_review_comment" => Ok(Self::PullRequestReviewComment),
            "pull_request_title" => Ok(Self::PullRequestTitle),
            "wiki_commit" => Ok(Self::WikiCommit),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for SecretScanningAlertLocationCreatedLocationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SecretScanningAlertLocationCreatedLocationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SecretScanningAlertLocationCreatedLocationType {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum SecurityAdvisory {
    #[serde(rename = "published")]
    Published {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        security_advisory: SecurityAdvisoryPublishedSecurityAdvisory,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "updated")]
    Updated {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        security_advisory: SecurityAdvisoryPublishedSecurityAdvisory,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
    #[serde(rename = "withdrawn")]
    Withdrawn {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        security_advisory: SecurityAdvisoryWithdrawnSecurityAdvisory,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
    },
}
impl From<&SecurityAdvisory> for SecurityAdvisory {
    fn from(value: &SecurityAdvisory) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SecurityAdvisoryPublishedSecurityAdvisory {
    pub cvss: DependabotAlertAutoDismissedAlertSecurityAdvisoryCvss,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvss_severities: Option<RepositoryAdvisoryPublishedRepositoryAdvisoryCvssSeverities>,
    pub cwes: Vec<RepositoryAdvisoryPublishedRepositoryAdvisoryCwes>,
    pub description: String,
    pub ghsa_id: String,
    pub identifiers: Vec<SecurityAdvisoryPublishedSecurityAdvisoryIdentifiers>,
    pub published_at: String,
    pub references: Vec<DependabotAlertAutoDismissedAlertSecurityAdvisoryReferences>,
    pub severity: String,
    pub summary: String,
    pub updated_at: String,
    pub vulnerabilities: Vec<SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilities>,
    pub withdrawn_at: Option<String>,
}
impl From<&SecurityAdvisoryPublishedSecurityAdvisory>
    for SecurityAdvisoryPublishedSecurityAdvisory
{
    fn from(value: &SecurityAdvisoryPublishedSecurityAdvisory) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SecurityAdvisoryPublishedSecurityAdvisoryIdentifiers {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}
impl From<&SecurityAdvisoryPublishedSecurityAdvisoryIdentifiers>
    for SecurityAdvisoryPublishedSecurityAdvisoryIdentifiers
{
    fn from(value: &SecurityAdvisoryPublishedSecurityAdvisoryIdentifiers) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilities {
    pub first_patched_version: Option<CheckRunRequestedActionRequestedAction>,
    pub package: DependabotAlertAutoDismissedAlertDependencyPackage,
    pub severity: String,
    pub vulnerable_version_range: String,
}
impl From<&SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilities>
    for SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilities
{
    fn from(value: &SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilities) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SecurityAdvisoryWithdrawnSecurityAdvisory {
    pub cvss: DependabotAlertAutoDismissedAlertSecurityAdvisoryCvss,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvss_severities: Option<RepositoryAdvisoryPublishedRepositoryAdvisoryCvssSeverities>,
    pub cwes: Vec<RepositoryAdvisoryPublishedRepositoryAdvisoryCwes>,
    pub description: String,
    pub ghsa_id: String,
    pub identifiers: Vec<SecurityAdvisoryPublishedSecurityAdvisoryIdentifiers>,
    pub published_at: String,
    pub references: Vec<DependabotAlertAutoDismissedAlertSecurityAdvisoryReferences>,
    pub severity: String,
    pub summary: String,
    pub updated_at: String,
    pub vulnerabilities: Vec<SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilities>,
    pub withdrawn_at: String,
}
impl From<&SecurityAdvisoryWithdrawnSecurityAdvisory>
    for SecurityAdvisoryWithdrawnSecurityAdvisory
{
    fn from(value: &SecurityAdvisoryWithdrawnSecurityAdvisory) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SecurityAndAnalysis {
    pub changes: SecurityAndAnalysisDefaultChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub repository: Untyped,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<Untyped>,
}
impl From<&SecurityAndAnalysis> for SecurityAndAnalysis {
    fn from(value: &SecurityAndAnalysis) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SecurityAndAnalysisDefaultChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<SecurityAndAnalysisDefaultChangesFrom>,
}
impl From<&SecurityAndAnalysisDefaultChanges> for SecurityAndAnalysisDefaultChanges {
    fn from(value: &SecurityAndAnalysisDefaultChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SecurityAndAnalysisDefaultChangesFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_and_analysis:
        Option<CheckRunCompletedCheckRunCheckSuiteRepositorySecurityAndAnalysis>,
}
impl From<&SecurityAndAnalysisDefaultChangesFrom> for SecurityAndAnalysisDefaultChangesFrom {
    fn from(value: &SecurityAndAnalysisDefaultChangesFrom) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Star {
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
        starred_at: Option<String>,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
        starred_at: (),
    },
}
impl From<&Star> for Star {
    fn from(value: &Star) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Status {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub branches: Vec<StatusDefaultBranches>,
    pub commit: StatusDefaultCommit,
    pub context: String,
    pub created_at: String,
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub repository: Untyped,
    pub sender: Untyped,
    pub sha: String,
    pub state: StatusDefaultState,
    pub target_url: Option<String>,
    pub updated_at: String,
}
impl From<&Status> for Status {
    fn from(value: &Status) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct StatusDefaultBranches {
    pub commit: StatusDefaultBranchesCommit,
    pub name: String,
    pub protected: bool,
}
impl From<&StatusDefaultBranches> for StatusDefaultBranches {
    fn from(value: &StatusDefaultBranches) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct StatusDefaultBranchesCommit {
    pub sha: Option<String>,
    pub url: Option<String>,
}
impl From<&StatusDefaultBranchesCommit> for StatusDefaultBranchesCommit {
    fn from(value: &StatusDefaultBranchesCommit) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct StatusDefaultCommit {
    pub author: Option<DeploymentReviewApprovedReviewersReviewer>,
    pub comments_url: String,
    pub commit: StatusDefaultCommitCommit,
    pub committer: Option<DeploymentReviewApprovedReviewersReviewer>,
    pub html_url: String,
    pub node_id: String,
    pub parents: Vec<StatusDefaultCommitParents>,
    pub sha: String,
    pub url: String,
}
impl From<&StatusDefaultCommit> for StatusDefaultCommit {
    fn from(value: &StatusDefaultCommit) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct StatusDefaultCommitCommit {
    pub author: PushDefaultCommitsAuthor,
    pub comment_count: i64,
    pub committer: PushDefaultCommitsAuthor,
    pub message: String,
    pub tree: StatusDefaultCommitCommitTree,
    pub url: String,
    pub verification: StatusDefaultCommitCommitVerification,
}
impl From<&StatusDefaultCommitCommit> for StatusDefaultCommitCommit {
    fn from(value: &StatusDefaultCommitCommit) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct StatusDefaultCommitCommitTree {
    pub sha: String,
    pub url: String,
}
impl From<&StatusDefaultCommitCommitTree> for StatusDefaultCommitCommitTree {
    fn from(value: &StatusDefaultCommitCommitTree) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct StatusDefaultCommitCommitVerification {
    pub payload: Option<String>,
    pub reason: StatusDefaultCommitCommitVerificationReason,
    pub signature: Option<String>,
    pub verified: bool,
}
impl From<&StatusDefaultCommitCommitVerification> for StatusDefaultCommitCommitVerification {
    fn from(value: &StatusDefaultCommitCommitVerification) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StatusDefaultCommitCommitVerificationReason {
    #[serde(rename = "bad_cert")]
    BadCert,
    #[serde(rename = "bad_email")]
    BadEmail,
    #[serde(rename = "expired_key")]
    ExpiredKey,
    #[serde(rename = "gpgverify_error")]
    GpgverifyError,
    #[serde(rename = "gpgverify_unavailable")]
    GpgverifyUnavailable,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "malformed_signature")]
    MalformedSignature,
    #[serde(rename = "no_user")]
    NoUser,
    #[serde(rename = "not_signing_key")]
    NotSigningKey,
    #[serde(rename = "ocsp_pending")]
    OcspPending,
    #[serde(rename = "unknown_key")]
    UnknownKey,
    #[serde(rename = "unknown_signature_type")]
    UnknownSignatureType,
    #[serde(rename = "unsigned")]
    Unsigned,
    #[serde(rename = "unverified_email")]
    UnverifiedEmail,
    #[serde(rename = "valid")]
    Valid,
}
impl From<&StatusDefaultCommitCommitVerificationReason>
    for StatusDefaultCommitCommitVerificationReason
{
    fn from(value: &StatusDefaultCommitCommitVerificationReason) -> Self {
        *value
    }
}
impl ::std::fmt::Display for StatusDefaultCommitCommitVerificationReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::BadCert => write!(f, "bad_cert"),
            Self::BadEmail => write!(f, "bad_email"),
            Self::ExpiredKey => write!(f, "expired_key"),
            Self::GpgverifyError => write!(f, "gpgverify_error"),
            Self::GpgverifyUnavailable => write!(f, "gpgverify_unavailable"),
            Self::Invalid => write!(f, "invalid"),
            Self::MalformedSignature => write!(f, "malformed_signature"),
            Self::NoUser => write!(f, "no_user"),
            Self::NotSigningKey => write!(f, "not_signing_key"),
            Self::OcspPending => write!(f, "ocsp_pending"),
            Self::UnknownKey => write!(f, "unknown_key"),
            Self::UnknownSignatureType => write!(f, "unknown_signature_type"),
            Self::Unsigned => write!(f, "unsigned"),
            Self::UnverifiedEmail => write!(f, "unverified_email"),
            Self::Valid => write!(f, "valid"),
        }
    }
}
impl std::str::FromStr for StatusDefaultCommitCommitVerificationReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "bad_cert" => Ok(Self::BadCert),
            "bad_email" => Ok(Self::BadEmail),
            "expired_key" => Ok(Self::ExpiredKey),
            "gpgverify_error" => Ok(Self::GpgverifyError),
            "gpgverify_unavailable" => Ok(Self::GpgverifyUnavailable),
            "invalid" => Ok(Self::Invalid),
            "malformed_signature" => Ok(Self::MalformedSignature),
            "no_user" => Ok(Self::NoUser),
            "not_signing_key" => Ok(Self::NotSigningKey),
            "ocsp_pending" => Ok(Self::OcspPending),
            "unknown_key" => Ok(Self::UnknownKey),
            "unknown_signature_type" => Ok(Self::UnknownSignatureType),
            "unsigned" => Ok(Self::Unsigned),
            "unverified_email" => Ok(Self::UnverifiedEmail),
            "valid" => Ok(Self::Valid),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for StatusDefaultCommitCommitVerificationReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StatusDefaultCommitCommitVerificationReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StatusDefaultCommitCommitVerificationReason {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct StatusDefaultCommitParents {
    pub html_url: String,
    pub sha: String,
    pub url: String,
}
impl From<&StatusDefaultCommitParents> for StatusDefaultCommitParents {
    fn from(value: &StatusDefaultCommitParents) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StatusDefaultState {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
}
impl From<&StatusDefaultState> for StatusDefaultState {
    fn from(value: &StatusDefaultState) -> Self {
        *value
    }
}
impl ::std::fmt::Display for StatusDefaultState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Error => write!(f, "error"),
            Self::Failure => write!(f, "failure"),
            Self::Pending => write!(f, "pending"),
            Self::Success => write!(f, "success"),
        }
    }
}
impl std::str::FromStr for StatusDefaultState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "error" => Ok(Self::Error),
            "failure" => Ok(Self::Failure),
            "pending" => Ok(Self::Pending),
            "success" => Ok(Self::Success),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for StatusDefaultState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StatusDefaultState {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StatusDefaultState {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum SubIssues {
    #[serde(rename = "parent_issue_added")]
    ParentIssueAdded {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        parent_issue: SubIssuesParentIssueAddedParentIssue,
        parent_issue_id: f64,
        parent_issue_repo: DeploymentProtectionRuleRequestedPullRequestsHeadRepo,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
        sub_issue: SubIssuesParentIssueAddedParentIssue,
        sub_issue_id: f64,
    },
    #[serde(rename = "parent_issue_removed")]
    ParentIssueRemoved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        parent_issue: SubIssuesParentIssueAddedParentIssue,
        parent_issue_id: f64,
        parent_issue_repo: DeploymentProtectionRuleRequestedPullRequestsHeadRepo,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
        sub_issue: SubIssuesParentIssueAddedParentIssue,
        sub_issue_id: f64,
    },
    #[serde(rename = "sub_issue_added")]
    SubIssueAdded {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        parent_issue: SubIssuesParentIssueAddedParentIssue,
        parent_issue_id: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
        sub_issue: SubIssuesParentIssueAddedParentIssue,
        sub_issue_id: f64,
        sub_issue_repo: DeploymentProtectionRuleRequestedPullRequestsHeadRepo,
    },
    #[serde(rename = "sub_issue_removed")]
    SubIssueRemoved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        parent_issue: SubIssuesParentIssueAddedParentIssue,
        parent_issue_id: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
        sub_issue: SubIssuesParentIssueAddedParentIssue,
        sub_issue_id: f64,
        sub_issue_repo: DeploymentProtectionRuleRequestedPullRequestsHeadRepo,
    },
}
impl From<&SubIssues> for SubIssues {
    fn from(value: &SubIssues) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SubIssuesParentIssueAddedParentIssue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub assignee: DiscussionTransferredChangesNewRepositoryOrganization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<DiscussionTransferredChangesNewRepositoryOrganization>>,
    pub author_association: DiscussionAnsweredAnswerAuthorAssociation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    pub closed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<DiscussionTransferredChangesNewRepositoryOrganization>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    pub labels: Vec<::serde_json::Value>,
    pub labels_url: String,
    pub locked: bool,
    pub milestone: DeploymentProtectionRuleRequestedPullRequestsMilestone,
    pub node_id: String,
    pub number: i64,
    #[serde(default)]
    pub performed_via_github_app: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<SubIssuesParentIssueAddedParentIssuePullRequest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<SubIssuesParentIssueAddedParentIssueReactions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<DeploymentProtectionRuleRequestedPullRequestsHeadRepo>,
    pub repository_url: String,
    pub state: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<SubIssuesParentIssueAddedParentIssueStateReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<String>,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: DiscussionTransferredChangesNewRepositoryOrganization,
}
impl From<&SubIssuesParentIssueAddedParentIssue> for SubIssuesParentIssueAddedParentIssue {
    fn from(value: &SubIssuesParentIssueAddedParentIssue) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SubIssuesParentIssueAddedParentIssuePullRequest {
    pub diff_url: Option<String>,
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_at: Option<String>,
    pub patch_url: Option<String>,
    pub url: Option<String>,
}
impl From<&SubIssuesParentIssueAddedParentIssuePullRequest>
    for SubIssuesParentIssueAddedParentIssuePullRequest
{
    fn from(value: &SubIssuesParentIssueAddedParentIssuePullRequest) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SubIssuesParentIssueAddedParentIssueReactions {
    pub confused: i64,
    pub eyes: i64,
    pub heart: i64,
    pub hooray: i64,
    pub laugh: i64,
    #[serde(rename = "-1")]
    pub minus1: i64,
    #[serde(rename = "+1")]
    pub plus1: i64,
    pub rocket: i64,
    pub total_count: i64,
    pub url: String,
}
impl From<&SubIssuesParentIssueAddedParentIssueReactions>
    for SubIssuesParentIssueAddedParentIssueReactions
{
    fn from(value: &SubIssuesParentIssueAddedParentIssueReactions) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SubIssuesParentIssueAddedParentIssueStateReason {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "not_planned")]
    NotPlanned,
    #[serde(rename = "reopened")]
    Reopened,
}
impl From<&SubIssuesParentIssueAddedParentIssueStateReason>
    for SubIssuesParentIssueAddedParentIssueStateReason
{
    fn from(value: &SubIssuesParentIssueAddedParentIssueStateReason) -> Self {
        *value
    }
}
impl ::std::fmt::Display for SubIssuesParentIssueAddedParentIssueStateReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => write!(f, "completed"),
            Self::NotPlanned => write!(f, "not_planned"),
            Self::Reopened => write!(f, "reopened"),
        }
    }
}
impl std::str::FromStr for SubIssuesParentIssueAddedParentIssueStateReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "not_planned" => Ok(Self::NotPlanned),
            "reopened" => Ok(Self::Reopened),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for SubIssuesParentIssueAddedParentIssueStateReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SubIssuesParentIssueAddedParentIssueStateReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SubIssuesParentIssueAddedParentIssueStateReason {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Team {
    #[serde(rename = "added_to_repository")]
    AddedToRepository {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
        team: TeamAddDefaultTeam,
    },
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
        team: TeamAddDefaultTeam,
    },
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sender: Option<Untyped>,
        team: TeamAddDefaultTeam,
    },
    #[serde(rename = "edited")]
    Edited {
        changes: TeamEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
        team: TeamAddDefaultTeam,
    },
    #[serde(rename = "removed_from_repository")]
    RemovedFromRepository {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        organization: Untyped,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Untyped>,
        sender: Untyped,
        team: TeamAddDefaultTeam,
    },
}
impl From<&Team> for Team {
    fn from(value: &Team) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TeamAdd {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    pub repository: Untyped,
    pub sender: Untyped,
    pub team: TeamAddDefaultTeam,
}
impl From<&TeamAdd> for TeamAdd {
    fn from(value: &TeamAdd) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TeamAddDefaultTeam {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_url: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<TeamAddDefaultTeamParentNotificationSetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<TeamAddDefaultTeamParent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<TeamAddDefaultTeamParentPrivacy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl From<&TeamAddDefaultTeam> for TeamAddDefaultTeam {
    fn from(value: &TeamAddDefaultTeam) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TeamAddDefaultTeamParent {
    pub description: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub members_url: String,
    pub name: String,
    pub node_id: String,
    pub notification_setting: TeamAddDefaultTeamParentNotificationSetting,
    pub permission: String,
    pub privacy: TeamAddDefaultTeamParentPrivacy,
    pub repositories_url: String,
    pub slug: String,
    pub url: String,
}
impl From<&TeamAddDefaultTeamParent> for TeamAddDefaultTeamParent {
    fn from(value: &TeamAddDefaultTeamParent) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TeamAddDefaultTeamParentNotificationSetting {
    #[serde(rename = "notifications_disabled")]
    NotificationsDisabled,
    #[serde(rename = "notifications_enabled")]
    NotificationsEnabled,
}
impl From<&TeamAddDefaultTeamParentNotificationSetting>
    for TeamAddDefaultTeamParentNotificationSetting
{
    fn from(value: &TeamAddDefaultTeamParentNotificationSetting) -> Self {
        *value
    }
}
impl ::std::fmt::Display for TeamAddDefaultTeamParentNotificationSetting {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NotificationsDisabled => write!(f, "notifications_disabled"),
            Self::NotificationsEnabled => write!(f, "notifications_enabled"),
        }
    }
}
impl std::str::FromStr for TeamAddDefaultTeamParentNotificationSetting {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "notifications_disabled" => Ok(Self::NotificationsDisabled),
            "notifications_enabled" => Ok(Self::NotificationsEnabled),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TeamAddDefaultTeamParentNotificationSetting {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TeamAddDefaultTeamParentNotificationSetting {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TeamAddDefaultTeamParentNotificationSetting {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TeamAddDefaultTeamParentPrivacy {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "secret")]
    Secret,
}
impl From<&TeamAddDefaultTeamParentPrivacy> for TeamAddDefaultTeamParentPrivacy {
    fn from(value: &TeamAddDefaultTeamParentPrivacy) -> Self {
        *value
    }
}
impl ::std::fmt::Display for TeamAddDefaultTeamParentPrivacy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Closed => write!(f, "closed"),
            Self::Open => write!(f, "open"),
            Self::Secret => write!(f, "secret"),
        }
    }
}
impl std::str::FromStr for TeamAddDefaultTeamParentPrivacy {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "closed" => Ok(Self::Closed),
            "open" => Ok(Self::Open),
            "secret" => Ok(Self::Secret),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for TeamAddDefaultTeamParentPrivacy {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TeamAddDefaultTeamParentPrivacy {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TeamAddDefaultTeamParentPrivacy {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TeamEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_setting: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<TeamEditedChangesRepository>,
}
impl From<&TeamEditedChanges> for TeamEditedChanges {
    fn from(value: &TeamEditedChanges) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TeamEditedChangesRepository {
    pub permissions: TeamEditedChangesRepositoryPermissions,
}
impl From<&TeamEditedChangesRepository> for TeamEditedChangesRepository {
    fn from(value: &TeamEditedChangesRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TeamEditedChangesRepositoryPermissions {
    pub from: TeamEditedChangesRepositoryPermissionsFrom,
}
impl From<&TeamEditedChangesRepositoryPermissions> for TeamEditedChangesRepositoryPermissions {
    fn from(value: &TeamEditedChangesRepositoryPermissions) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct TeamEditedChangesRepositoryPermissionsFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<bool>,
}
impl From<&TeamEditedChangesRepositoryPermissionsFrom>
    for TeamEditedChangesRepositoryPermissionsFrom
{
    fn from(value: &TeamEditedChangesRepositoryPermissionsFrom) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Untyped(pub ::serde_json::Map<String, ::serde_json::Value>);
impl ::std::ops::Deref for Untyped {
    type Target = ::serde_json::Map<String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<String, ::serde_json::Value> {
        &self.0
    }
}
impl From<Untyped> for ::serde_json::Map<String, ::serde_json::Value> {
    fn from(value: Untyped) -> Self {
        value.0
    }
}
impl From<&Untyped> for Untyped {
    fn from(value: &Untyped) -> Self {
        value.clone()
    }
}
impl From<::serde_json::Map<String, ::serde_json::Value>> for Untyped {
    fn from(value: ::serde_json::Map<String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum Watch {
    #[serde(rename = "started")]
    Started {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
    },
}
impl From<&Watch> for Watch {
    fn from(value: &Watch) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug,PartialEq)]
pub enum WebhookBody {
    #[serde(rename = "team_add")]
    TeamAdd(TeamAdd),
    #[serde(rename = "deployment_status")]
    DeploymentStatus(DeploymentStatus),
    #[serde(rename = "discussion")]
    Discussion(Discussion),
    #[serde(rename = "secret_scanning_alert_location")]
    SecretScanningAlertLocation(SecretScanningAlertLocation),
    #[serde(rename = "delete")]
    Delete(Delete),
    #[serde(rename = "workflow_dispatch")]
    WorkflowDispatch(WorkflowDispatch),
    #[serde(rename = "repository_ruleset")]
    RepositoryRuleset(RepositoryRuleset),
    #[serde(rename = "installation")]
    Installation(Installation),
    #[serde(rename = "deployment_protection_rule")]
    DeploymentProtectionRule(DeploymentProtectionRule),
    #[serde(rename = "personal_access_token_request")]
    PersonalAccessTokenRequest(PersonalAccessTokenRequest),
    #[serde(rename = "membership")]
    Membership(Membership),
    #[serde(rename = "milestone")]
    Milestone(Milestone),
    #[serde(rename = "deployment")]
    Deployment(Deployment),
    #[serde(rename = "deployment_review")]
    DeploymentReview(DeploymentReview),
    #[serde(rename = "project")]
    Project(Project),
    #[serde(rename = "issue_comment")]
    IssueComment(IssueComment),
    #[serde(rename = "branch_protection_rule")]
    BranchProtectionRule(BranchProtectionRule),
    #[serde(rename = "pull_request_review_comment")]
    PullRequestReviewComment(PullRequestReviewComment),
    #[serde(rename = "member")]
    Member(Member),
    #[serde(rename = "repository")]
    Repository(Repository),
    #[serde(rename = "sub_issues")]
    SubIssues(SubIssues),
    #[serde(rename = "branch_protection_configuration")]
    BranchProtectionConfiguration(BranchProtectionConfiguration),
    #[serde(rename = "project_column")]
    ProjectColumn(ProjectColumn),
    #[serde(rename = "repository_dispatch")]
    RepositoryDispatch(RepositoryDispatch),
    #[serde(rename = "push")]
    Push(Push),
    #[serde(rename = "installation_repositories")]
    InstallationRepositories(InstallationRepositories),
    #[serde(rename = "github_app_authorization")]
    GithubAppAuthorization(GithubAppAuthorization),
    #[serde(rename = "discussion_comment")]
    DiscussionComment(DiscussionComment),
    #[serde(rename = "release")]
    Release(Release),
    #[serde(rename = "org_block")]
    OrgBlock(OrgBlock),
    #[serde(rename = "secret_scanning_alert")]
    SecretScanningAlert(SecretScanningAlert),
    #[serde(rename = "page_build")]
    PageBuild(PageBuild),
    #[serde(rename = "issues")]
    Issues(Issues),
    #[serde(rename = "create")]
    Create(Create),
    #[serde(rename = "repository_advisory")]
    RepositoryAdvisory(RepositoryAdvisory),
    #[serde(rename = "package")]
    Package(Package),
    #[serde(rename = "pull_request_review")]
    PullRequestReview(PullRequestReview),
    #[serde(rename = "public")]
    Public(Public),
    #[serde(rename = "code_scanning_alert")]
    CodeScanningAlert(CodeScanningAlert),
    #[serde(rename = "watch")]
    Watch(Watch),
    #[serde(rename = "fork")]
    Fork(Fork),
    #[serde(rename = "commit_comment")]
    CommitComment(CommitComment),
    #[serde(rename = "organization")]
    Organization(Organization),
    #[serde(rename = "workflow_run")]
    WorkflowRun(WorkflowRun),
    #[serde(rename = "custom_property_values")]
    CustomPropertyValues(CustomPropertyValues),
    #[serde(rename = "deploy_key")]
    DeployKey(DeployKey),
    #[serde(rename = "star")]
    Star(Star),
    #[serde(rename = "registry_package")]
    RegistryPackage(RegistryPackage),
    #[serde(rename = "merge_group")]
    MergeGroup(MergeGroup),
    #[serde(rename = "check_run")]
    CheckRun(CheckRun),
    #[serde(rename = "installation_target")]
    InstallationTarget(InstallationTarget),
    #[serde(rename = "security_and_analysis")]
    SecurityAndAnalysis(SecurityAndAnalysis),
    #[serde(rename = "ping")]
    Ping(Ping),
    #[serde(rename = "label")]
    Label(Label),
    #[serde(rename = "dependabot_alert")]
    DependabotAlert(DependabotAlert),
    #[serde(rename = "project_card")]
    ProjectCard(ProjectCard),
    #[serde(rename = "gollum")]
    Gollum(Gollum),
    #[serde(rename = "team")]
    Team(Team),
    #[serde(rename = "status")]
    Status(Status),
    #[serde(rename = "check_suite")]
    CheckSuite(CheckSuite),
    #[serde(rename = "workflow_job")]
    WorkflowJob(WorkflowJob),
    #[serde(rename = "pull_request_review_thread")]
    PullRequestReviewThread(PullRequestReviewThread),
    #[serde(rename = "pull_request")]
    PullRequest(PullRequest),
    #[serde(rename = "security_advisory")]
    SecurityAdvisory(SecurityAdvisory),
    #[serde(rename = "meta")]
    Meta(Meta),
    #[serde(rename = "custom_property")]
    CustomProperty(CustomProperty),
}
impl From<&WebhookBody> for WebhookBody {
    fn from(value: &WebhookBody) -> Self {
        value.clone()
    }
}
impl From<TeamAdd> for WebhookBody {
    fn from(value: TeamAdd) -> Self {
        Self::TeamAdd(value)
    }
}
impl From<DeploymentStatus> for WebhookBody {
    fn from(value: DeploymentStatus) -> Self {
        Self::DeploymentStatus(value)
    }
}
impl From<Discussion> for WebhookBody {
    fn from(value: Discussion) -> Self {
        Self::Discussion(value)
    }
}
impl From<SecretScanningAlertLocation> for WebhookBody {
    fn from(value: SecretScanningAlertLocation) -> Self {
        Self::SecretScanningAlertLocation(value)
    }
}
impl From<Delete> for WebhookBody {
    fn from(value: Delete) -> Self {
        Self::Delete(value)
    }
}
impl From<WorkflowDispatch> for WebhookBody {
    fn from(value: WorkflowDispatch) -> Self {
        Self::WorkflowDispatch(value)
    }
}
impl From<RepositoryRuleset> for WebhookBody {
    fn from(value: RepositoryRuleset) -> Self {
        Self::RepositoryRuleset(value)
    }
}
impl From<Installation> for WebhookBody {
    fn from(value: Installation) -> Self {
        Self::Installation(value)
    }
}
impl From<DeploymentProtectionRule> for WebhookBody {
    fn from(value: DeploymentProtectionRule) -> Self {
        Self::DeploymentProtectionRule(value)
    }
}
impl From<PersonalAccessTokenRequest> for WebhookBody {
    fn from(value: PersonalAccessTokenRequest) -> Self {
        Self::PersonalAccessTokenRequest(value)
    }
}
impl From<Membership> for WebhookBody {
    fn from(value: Membership) -> Self {
        Self::Membership(value)
    }
}
impl From<Milestone> for WebhookBody {
    fn from(value: Milestone) -> Self {
        Self::Milestone(value)
    }
}
impl From<Deployment> for WebhookBody {
    fn from(value: Deployment) -> Self {
        Self::Deployment(value)
    }
}
impl From<DeploymentReview> for WebhookBody {
    fn from(value: DeploymentReview) -> Self {
        Self::DeploymentReview(value)
    }
}
impl From<Project> for WebhookBody {
    fn from(value: Project) -> Self {
        Self::Project(value)
    }
}
impl From<IssueComment> for WebhookBody {
    fn from(value: IssueComment) -> Self {
        Self::IssueComment(value)
    }
}
impl From<BranchProtectionRule> for WebhookBody {
    fn from(value: BranchProtectionRule) -> Self {
        Self::BranchProtectionRule(value)
    }
}
impl From<PullRequestReviewComment> for WebhookBody {
    fn from(value: PullRequestReviewComment) -> Self {
        Self::PullRequestReviewComment(value)
    }
}
impl From<Member> for WebhookBody {
    fn from(value: Member) -> Self {
        Self::Member(value)
    }
}
impl From<Repository> for WebhookBody {
    fn from(value: Repository) -> Self {
        Self::Repository(value)
    }
}
impl From<SubIssues> for WebhookBody {
    fn from(value: SubIssues) -> Self {
        Self::SubIssues(value)
    }
}
impl From<BranchProtectionConfiguration> for WebhookBody {
    fn from(value: BranchProtectionConfiguration) -> Self {
        Self::BranchProtectionConfiguration(value)
    }
}
impl From<ProjectColumn> for WebhookBody {
    fn from(value: ProjectColumn) -> Self {
        Self::ProjectColumn(value)
    }
}
impl From<RepositoryDispatch> for WebhookBody {
    fn from(value: RepositoryDispatch) -> Self {
        Self::RepositoryDispatch(value)
    }
}
impl From<Push> for WebhookBody {
    fn from(value: Push) -> Self {
        Self::Push(value)
    }
}
impl From<InstallationRepositories> for WebhookBody {
    fn from(value: InstallationRepositories) -> Self {
        Self::InstallationRepositories(value)
    }
}
impl From<GithubAppAuthorization> for WebhookBody {
    fn from(value: GithubAppAuthorization) -> Self {
        Self::GithubAppAuthorization(value)
    }
}
impl From<DiscussionComment> for WebhookBody {
    fn from(value: DiscussionComment) -> Self {
        Self::DiscussionComment(value)
    }
}
impl From<Release> for WebhookBody {
    fn from(value: Release) -> Self {
        Self::Release(value)
    }
}
impl From<OrgBlock> for WebhookBody {
    fn from(value: OrgBlock) -> Self {
        Self::OrgBlock(value)
    }
}
impl From<SecretScanningAlert> for WebhookBody {
    fn from(value: SecretScanningAlert) -> Self {
        Self::SecretScanningAlert(value)
    }
}
impl From<PageBuild> for WebhookBody {
    fn from(value: PageBuild) -> Self {
        Self::PageBuild(value)
    }
}
impl From<Issues> for WebhookBody {
    fn from(value: Issues) -> Self {
        Self::Issues(value)
    }
}
impl From<Create> for WebhookBody {
    fn from(value: Create) -> Self {
        Self::Create(value)
    }
}
impl From<RepositoryAdvisory> for WebhookBody {
    fn from(value: RepositoryAdvisory) -> Self {
        Self::RepositoryAdvisory(value)
    }
}
impl From<Package> for WebhookBody {
    fn from(value: Package) -> Self {
        Self::Package(value)
    }
}
impl From<PullRequestReview> for WebhookBody {
    fn from(value: PullRequestReview) -> Self {
        Self::PullRequestReview(value)
    }
}
impl From<Public> for WebhookBody {
    fn from(value: Public) -> Self {
        Self::Public(value)
    }
}
impl From<CodeScanningAlert> for WebhookBody {
    fn from(value: CodeScanningAlert) -> Self {
        Self::CodeScanningAlert(value)
    }
}
impl From<Watch> for WebhookBody {
    fn from(value: Watch) -> Self {
        Self::Watch(value)
    }
}
impl From<Fork> for WebhookBody {
    fn from(value: Fork) -> Self {
        Self::Fork(value)
    }
}
impl From<CommitComment> for WebhookBody {
    fn from(value: CommitComment) -> Self {
        Self::CommitComment(value)
    }
}
impl From<Organization> for WebhookBody {
    fn from(value: Organization) -> Self {
        Self::Organization(value)
    }
}
impl From<WorkflowRun> for WebhookBody {
    fn from(value: WorkflowRun) -> Self {
        Self::WorkflowRun(value)
    }
}
impl From<CustomPropertyValues> for WebhookBody {
    fn from(value: CustomPropertyValues) -> Self {
        Self::CustomPropertyValues(value)
    }
}
impl From<DeployKey> for WebhookBody {
    fn from(value: DeployKey) -> Self {
        Self::DeployKey(value)
    }
}
impl From<Star> for WebhookBody {
    fn from(value: Star) -> Self {
        Self::Star(value)
    }
}
impl From<RegistryPackage> for WebhookBody {
    fn from(value: RegistryPackage) -> Self {
        Self::RegistryPackage(value)
    }
}
impl From<MergeGroup> for WebhookBody {
    fn from(value: MergeGroup) -> Self {
        Self::MergeGroup(value)
    }
}
impl From<CheckRun> for WebhookBody {
    fn from(value: CheckRun) -> Self {
        Self::CheckRun(value)
    }
}
impl From<InstallationTarget> for WebhookBody {
    fn from(value: InstallationTarget) -> Self {
        Self::InstallationTarget(value)
    }
}
impl From<SecurityAndAnalysis> for WebhookBody {
    fn from(value: SecurityAndAnalysis) -> Self {
        Self::SecurityAndAnalysis(value)
    }
}
impl From<Ping> for WebhookBody {
    fn from(value: Ping) -> Self {
        Self::Ping(value)
    }
}
impl From<Label> for WebhookBody {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}
impl From<DependabotAlert> for WebhookBody {
    fn from(value: DependabotAlert) -> Self {
        Self::DependabotAlert(value)
    }
}
impl From<ProjectCard> for WebhookBody {
    fn from(value: ProjectCard) -> Self {
        Self::ProjectCard(value)
    }
}
impl From<Gollum> for WebhookBody {
    fn from(value: Gollum) -> Self {
        Self::Gollum(value)
    }
}
impl From<Team> for WebhookBody {
    fn from(value: Team) -> Self {
        Self::Team(value)
    }
}
impl From<Status> for WebhookBody {
    fn from(value: Status) -> Self {
        Self::Status(value)
    }
}
impl From<CheckSuite> for WebhookBody {
    fn from(value: CheckSuite) -> Self {
        Self::CheckSuite(value)
    }
}
impl From<WorkflowJob> for WebhookBody {
    fn from(value: WorkflowJob) -> Self {
        Self::WorkflowJob(value)
    }
}
impl From<PullRequestReviewThread> for WebhookBody {
    fn from(value: PullRequestReviewThread) -> Self {
        Self::PullRequestReviewThread(value)
    }
}
impl From<PullRequest> for WebhookBody {
    fn from(value: PullRequest) -> Self {
        Self::PullRequest(value)
    }
}
impl From<SecurityAdvisory> for WebhookBody {
    fn from(value: SecurityAdvisory) -> Self {
        Self::SecurityAdvisory(value)
    }
}
impl From<Meta> for WebhookBody {
    fn from(value: Meta) -> Self {
        Self::Meta(value)
    }
}
impl From<CustomProperty> for WebhookBody {
    fn from(value: CustomProperty) -> Self {
        Self::CustomProperty(value)
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowDispatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Untyped>,
    pub inputs: Option<Untyped>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationAttribute>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Untyped>,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Untyped,
    pub sender: Untyped,
    pub workflow: String,
}
impl From<&WorkflowDispatch> for WorkflowDispatch {
    fn from(value: &WorkflowDispatch) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum WorkflowJob {
    #[serde(rename = "completed")]
    Completed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        deployment: Option<DeploymentProtectionRuleRequestedDeployment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
        workflow_job: WorkflowJobCompletedWorkflowJob,
    },
    #[serde(rename = "in_progress")]
    InProgress {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        deployment: Option<DeploymentProtectionRuleRequestedDeployment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
        workflow_job: WorkflowJobInProgressWorkflowJob,
    },
    #[serde(rename = "queued")]
    Queued {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        deployment: Option<DeploymentProtectionRuleRequestedDeployment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
        workflow_job: WorkflowJobQueuedWorkflowJob,
    },
    #[serde(rename = "waiting")]
    Waiting {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        deployment: Option<DeploymentProtectionRuleRequestedDeployment>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
        workflow_job: WorkflowJobWaitingWorkflowJob,
    },
}
impl From<&WorkflowJob> for WorkflowJob {
    fn from(value: &WorkflowJob) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowJobCompletedWorkflowJob {
    pub check_run_url: String,
    pub completed_at: Option<String>,
    pub conclusion: Option<WorkflowJobCompletedWorkflowJobConclusion>,
    pub created_at: String,
    pub head_branch: Option<String>,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    pub labels: Vec<String>,
    pub name: String,
    pub node_id: String,
    pub run_attempt: i64,
    pub run_id: f64,
    pub run_url: String,
    pub runner_group_id: Option<i64>,
    pub runner_group_name: Option<String>,
    pub runner_id: Option<i64>,
    pub runner_name: Option<String>,
    pub started_at: String,
    pub status: WorkflowJobCompletedWorkflowJobStatus,
    pub steps: Vec<WorkflowJobCompletedWorkflowJobSteps>,
    pub url: String,
    pub workflow_name: Option<String>,
}
impl From<&WorkflowJobCompletedWorkflowJob> for WorkflowJobCompletedWorkflowJob {
    fn from(value: &WorkflowJobCompletedWorkflowJob) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorkflowJobCompletedWorkflowJobConclusion {
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "timed_out")]
    TimedOut,
}
impl From<&WorkflowJobCompletedWorkflowJobConclusion>
    for WorkflowJobCompletedWorkflowJobConclusion
{
    fn from(value: &WorkflowJobCompletedWorkflowJobConclusion) -> Self {
        *value
    }
}
impl ::std::fmt::Display for WorkflowJobCompletedWorkflowJobConclusion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ActionRequired => write!(f, "action_required"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Failure => write!(f, "failure"),
            Self::Neutral => write!(f, "neutral"),
            Self::Skipped => write!(f, "skipped"),
            Self::Success => write!(f, "success"),
            Self::TimedOut => write!(f, "timed_out"),
        }
    }
}
impl std::str::FromStr for WorkflowJobCompletedWorkflowJobConclusion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "action_required" => Ok(Self::ActionRequired),
            "cancelled" => Ok(Self::Cancelled),
            "failure" => Ok(Self::Failure),
            "neutral" => Ok(Self::Neutral),
            "skipped" => Ok(Self::Skipped),
            "success" => Ok(Self::Success),
            "timed_out" => Ok(Self::TimedOut),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for WorkflowJobCompletedWorkflowJobConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for WorkflowJobCompletedWorkflowJobConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for WorkflowJobCompletedWorkflowJobConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorkflowJobCompletedWorkflowJobStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "waiting")]
    Waiting,
}
impl From<&WorkflowJobCompletedWorkflowJobStatus> for WorkflowJobCompletedWorkflowJobStatus {
    fn from(value: &WorkflowJobCompletedWorkflowJobStatus) -> Self {
        *value
    }
}
impl ::std::fmt::Display for WorkflowJobCompletedWorkflowJobStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => write!(f, "completed"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Queued => write!(f, "queued"),
            Self::Waiting => write!(f, "waiting"),
        }
    }
}
impl std::str::FromStr for WorkflowJobCompletedWorkflowJobStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "in_progress" => Ok(Self::InProgress),
            "queued" => Ok(Self::Queued),
            "waiting" => Ok(Self::Waiting),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for WorkflowJobCompletedWorkflowJobStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for WorkflowJobCompletedWorkflowJobStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for WorkflowJobCompletedWorkflowJobStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowJobCompletedWorkflowJobSteps {
    pub completed_at: Option<String>,
    pub conclusion: Option<WorkflowJobCompletedWorkflowJobStepsConclusion>,
    pub name: String,
    pub number: i64,
    pub started_at: Option<String>,
    pub status: WorkflowJobCompletedWorkflowJobStepsStatus,
}
impl From<&WorkflowJobCompletedWorkflowJobSteps> for WorkflowJobCompletedWorkflowJobSteps {
    fn from(value: &WorkflowJobCompletedWorkflowJobSteps) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorkflowJobCompletedWorkflowJobStepsConclusion {
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "success")]
    Success,
}
impl From<&WorkflowJobCompletedWorkflowJobStepsConclusion>
    for WorkflowJobCompletedWorkflowJobStepsConclusion
{
    fn from(value: &WorkflowJobCompletedWorkflowJobStepsConclusion) -> Self {
        *value
    }
}
impl ::std::fmt::Display for WorkflowJobCompletedWorkflowJobStepsConclusion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cancelled => write!(f, "cancelled"),
            Self::Failure => write!(f, "failure"),
            Self::Skipped => write!(f, "skipped"),
            Self::Success => write!(f, "success"),
        }
    }
}
impl std::str::FromStr for WorkflowJobCompletedWorkflowJobStepsConclusion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "cancelled" => Ok(Self::Cancelled),
            "failure" => Ok(Self::Failure),
            "skipped" => Ok(Self::Skipped),
            "success" => Ok(Self::Success),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for WorkflowJobCompletedWorkflowJobStepsConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for WorkflowJobCompletedWorkflowJobStepsConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for WorkflowJobCompletedWorkflowJobStepsConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorkflowJobCompletedWorkflowJobStepsStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "queued")]
    Queued,
}
impl From<&WorkflowJobCompletedWorkflowJobStepsStatus>
    for WorkflowJobCompletedWorkflowJobStepsStatus
{
    fn from(value: &WorkflowJobCompletedWorkflowJobStepsStatus) -> Self {
        *value
    }
}
impl ::std::fmt::Display for WorkflowJobCompletedWorkflowJobStepsStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => write!(f, "completed"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Queued => write!(f, "queued"),
        }
    }
}
impl std::str::FromStr for WorkflowJobCompletedWorkflowJobStepsStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "in_progress" => Ok(Self::InProgress),
            "queued" => Ok(Self::Queued),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for WorkflowJobCompletedWorkflowJobStepsStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for WorkflowJobCompletedWorkflowJobStepsStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for WorkflowJobCompletedWorkflowJobStepsStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowJobInProgressWorkflowJob {
    pub check_run_url: String,
    pub completed_at: Option<String>,
    pub conclusion: Option<WorkflowJobInProgressWorkflowJobConclusion>,
    pub created_at: String,
    pub head_branch: Option<String>,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    pub labels: Vec<String>,
    pub name: String,
    pub node_id: String,
    pub run_attempt: i64,
    pub run_id: f64,
    pub run_url: String,
    pub runner_group_id: Option<i64>,
    pub runner_group_name: Option<String>,
    pub runner_id: Option<i64>,
    pub runner_name: Option<String>,
    pub started_at: String,
    pub status: WorkflowJobCompletedWorkflowJobStepsStatus,
    pub steps: Vec<WorkflowJobInProgressWorkflowJobSteps>,
    pub url: String,
    pub workflow_name: Option<String>,
}
impl From<&WorkflowJobInProgressWorkflowJob> for WorkflowJobInProgressWorkflowJob {
    fn from(value: &WorkflowJobInProgressWorkflowJob) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorkflowJobInProgressWorkflowJobConclusion {
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "success")]
    Success,
}
impl From<&WorkflowJobInProgressWorkflowJobConclusion>
    for WorkflowJobInProgressWorkflowJobConclusion
{
    fn from(value: &WorkflowJobInProgressWorkflowJobConclusion) -> Self {
        *value
    }
}
impl ::std::fmt::Display for WorkflowJobInProgressWorkflowJobConclusion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cancelled => write!(f, "cancelled"),
            Self::Failure => write!(f, "failure"),
            Self::Neutral => write!(f, "neutral"),
            Self::Success => write!(f, "success"),
        }
    }
}
impl std::str::FromStr for WorkflowJobInProgressWorkflowJobConclusion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "cancelled" => Ok(Self::Cancelled),
            "failure" => Ok(Self::Failure),
            "neutral" => Ok(Self::Neutral),
            "success" => Ok(Self::Success),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for WorkflowJobInProgressWorkflowJobConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for WorkflowJobInProgressWorkflowJobConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for WorkflowJobInProgressWorkflowJobConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowJobInProgressWorkflowJobSteps {
    pub completed_at: Option<String>,
    pub conclusion: Option<WorkflowJobCompletedWorkflowJobStepsConclusion>,
    pub name: String,
    pub number: i64,
    pub started_at: Option<String>,
    pub status: CheckRunCompletedCheckRunStatus,
}
impl From<&WorkflowJobInProgressWorkflowJobSteps> for WorkflowJobInProgressWorkflowJobSteps {
    fn from(value: &WorkflowJobInProgressWorkflowJobSteps) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowJobQueuedWorkflowJob {
    pub check_run_url: String,
    pub completed_at: Option<String>,
    pub conclusion: Option<String>,
    pub created_at: String,
    pub head_branch: Option<String>,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    pub labels: Vec<String>,
    pub name: String,
    pub node_id: String,
    pub run_attempt: i64,
    pub run_id: f64,
    pub run_url: String,
    pub runner_group_id: Option<i64>,
    pub runner_group_name: Option<String>,
    pub runner_id: Option<i64>,
    pub runner_name: Option<String>,
    pub started_at: String,
    pub status: WorkflowJobCompletedWorkflowJobStatus,
    pub steps: Vec<WorkflowJobInProgressWorkflowJobSteps>,
    pub url: String,
    pub workflow_name: Option<String>,
}
impl From<&WorkflowJobQueuedWorkflowJob> for WorkflowJobQueuedWorkflowJob {
    fn from(value: &WorkflowJobQueuedWorkflowJob) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowJobWaitingWorkflowJob {
    pub check_run_url: String,
    pub completed_at: Option<String>,
    pub conclusion: Option<String>,
    pub created_at: String,
    pub head_branch: Option<String>,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    pub labels: Vec<String>,
    pub name: String,
    pub node_id: String,
    pub run_attempt: i64,
    pub run_id: f64,
    pub run_url: String,
    pub runner_group_id: Option<i64>,
    pub runner_group_name: Option<String>,
    pub runner_id: Option<i64>,
    pub runner_name: Option<String>,
    pub started_at: String,
    pub status: WorkflowJobCompletedWorkflowJobStatus,
    pub steps: Vec<WorkflowJobWaitingWorkflowJobSteps>,
    pub url: String,
    pub workflow_name: Option<String>,
}
impl From<&WorkflowJobWaitingWorkflowJob> for WorkflowJobWaitingWorkflowJob {
    fn from(value: &WorkflowJobWaitingWorkflowJob) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowJobWaitingWorkflowJobSteps {
    pub completed_at: Option<String>,
    pub conclusion: Option<WorkflowJobCompletedWorkflowJobStepsConclusion>,
    pub name: String,
    pub number: i64,
    pub started_at: Option<String>,
    pub status: DeploymentStatusCreatedCheckRunStatus,
}
impl From<&WorkflowJobWaitingWorkflowJobSteps> for WorkflowJobWaitingWorkflowJobSteps {
    fn from(value: &WorkflowJobWaitingWorkflowJobSteps) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "action")]
pub enum WorkflowRun {
    #[serde(rename = "completed")]
    Completed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
        workflow: Option<DeploymentStatusCreatedWorkflow>,
        workflow_run: WorkflowRunCompletedWorkflowRun,
    },
    #[serde(rename = "in_progress")]
    InProgress {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
        workflow: Option<DeploymentStatusCreatedWorkflow>,
        workflow_run: WorkflowRunInProgressWorkflowRun,
    },
    #[serde(rename = "requested")]
    Requested {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        enterprise: Option<Untyped>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationAttribute>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Untyped>,
        repository: Untyped,
        sender: Untyped,
        workflow: Option<DeploymentStatusCreatedWorkflow>,
        workflow_run: WorkflowRunRequestedWorkflowRun,
    },
}
impl From<&WorkflowRun> for WorkflowRun {
    fn from(value: &WorkflowRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowRunCompletedWorkflowRun {
    pub actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub artifacts_url: String,
    pub cancel_url: String,
    pub check_suite_id: i64,
    pub check_suite_node_id: String,
    pub check_suite_url: String,
    pub conclusion: Option<WorkflowRunCompletedWorkflowRunConclusion>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_title: Option<String>,
    pub event: String,
    pub head_branch: Option<String>,
    pub head_commit: WorkflowRunCompletedWorkflowRunHeadCommit,
    pub head_repository: WorkflowRunCompletedWorkflowRunHeadRepository,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    pub jobs_url: String,
    pub logs_url: String,
    pub name: Option<String>,
    pub node_id: String,
    pub path: String,
    pub previous_attempt_url: Option<String>,
    pub pull_requests: Vec<Option<Untyped>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_workflows: Option<Vec<DeploymentStatusCreatedWorkflowRunReferencedWorkflows>>,
    pub repository: WorkflowRunCompletedWorkflowRunHeadRepository,
    pub rerun_url: String,
    pub run_attempt: i64,
    pub run_number: i64,
    pub run_started_at: String,
    pub status: DeploymentStatusCreatedWorkflowRunStatus,
    pub triggering_actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub updated_at: String,
    pub url: String,
    pub workflow_id: i64,
    pub workflow_url: String,
}
impl From<&WorkflowRunCompletedWorkflowRun> for WorkflowRunCompletedWorkflowRun {
    fn from(value: &WorkflowRunCompletedWorkflowRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorkflowRunCompletedWorkflowRunConclusion {
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "startup_failure")]
    StartupFailure,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "timed_out")]
    TimedOut,
}
impl From<&WorkflowRunCompletedWorkflowRunConclusion>
    for WorkflowRunCompletedWorkflowRunConclusion
{
    fn from(value: &WorkflowRunCompletedWorkflowRunConclusion) -> Self {
        *value
    }
}
impl ::std::fmt::Display for WorkflowRunCompletedWorkflowRunConclusion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ActionRequired => write!(f, "action_required"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Failure => write!(f, "failure"),
            Self::Neutral => write!(f, "neutral"),
            Self::Skipped => write!(f, "skipped"),
            Self::Stale => write!(f, "stale"),
            Self::StartupFailure => write!(f, "startup_failure"),
            Self::Success => write!(f, "success"),
            Self::TimedOut => write!(f, "timed_out"),
        }
    }
}
impl std::str::FromStr for WorkflowRunCompletedWorkflowRunConclusion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "action_required" => Ok(Self::ActionRequired),
            "cancelled" => Ok(Self::Cancelled),
            "failure" => Ok(Self::Failure),
            "neutral" => Ok(Self::Neutral),
            "skipped" => Ok(Self::Skipped),
            "stale" => Ok(Self::Stale),
            "startup_failure" => Ok(Self::StartupFailure),
            "success" => Ok(Self::Success),
            "timed_out" => Ok(Self::TimedOut),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for WorkflowRunCompletedWorkflowRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for WorkflowRunCompletedWorkflowRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for WorkflowRunCompletedWorkflowRunConclusion {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowRunCompletedWorkflowRunHeadCommit {
    pub author: PushDefaultCommitsAuthor,
    pub committer: PushDefaultCommitsAuthor,
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub tree_id: String,
}
impl From<&WorkflowRunCompletedWorkflowRunHeadCommit>
    for WorkflowRunCompletedWorkflowRunHeadCommit
{
    fn from(value: &WorkflowRunCompletedWorkflowRunHeadCommit) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowRunCompletedWorkflowRunHeadRepository {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub description: Option<String>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub owner: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub private: bool,
    pub pulls_url: String,
    pub releases_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub url: String,
}
impl From<&WorkflowRunCompletedWorkflowRunHeadRepository>
    for WorkflowRunCompletedWorkflowRunHeadRepository
{
    fn from(value: &WorkflowRunCompletedWorkflowRunHeadRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowRunInProgressWorkflowRun {
    pub actor: Option<DeploymentReviewApprovedReviewersReviewer>,
    pub artifacts_url: String,
    pub cancel_url: String,
    pub check_suite_id: i64,
    pub check_suite_node_id: String,
    pub check_suite_url: String,
    pub conclusion: Option<DeploymentStatusCreatedCheckRunConclusion>,
    pub created_at: String,
    pub event: String,
    pub head_branch: Option<String>,
    pub head_commit: WorkflowRunCompletedWorkflowRunHeadCommit,
    pub head_repository: WorkflowRunInProgressWorkflowRunHeadRepository,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    pub jobs_url: String,
    pub logs_url: String,
    pub name: Option<String>,
    pub node_id: String,
    pub path: String,
    pub previous_attempt_url: Option<String>,
    pub pull_requests: Vec<Option<Untyped>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_workflows: Option<Vec<DeploymentStatusCreatedWorkflowRunReferencedWorkflows>>,
    pub repository: WorkflowRunInProgressWorkflowRunRepository,
    pub rerun_url: String,
    pub run_attempt: i64,
    pub run_number: i64,
    pub run_started_at: String,
    pub status: WorkflowRunInProgressWorkflowRunStatus,
    pub triggering_actor: Option<DeploymentReviewApprovedReviewersReviewer>,
    pub updated_at: String,
    pub url: String,
    pub workflow_id: i64,
    pub workflow_url: String,
}
impl From<&WorkflowRunInProgressWorkflowRun> for WorkflowRunInProgressWorkflowRun {
    fn from(value: &WorkflowRunInProgressWorkflowRun) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowRunInProgressWorkflowRunHeadRepository {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub description: Option<String>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub name: Option<String>,
    pub node_id: String,
    pub notifications_url: String,
    pub owner: Option<DeploymentReviewApprovedReviewersReviewer>,
    pub private: bool,
    pub pulls_url: String,
    pub releases_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub url: String,
}
impl From<&WorkflowRunInProgressWorkflowRunHeadRepository>
    for WorkflowRunInProgressWorkflowRunHeadRepository
{
    fn from(value: &WorkflowRunInProgressWorkflowRunHeadRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowRunInProgressWorkflowRunRepository {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub description: Option<String>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub hooks_url: String,
    pub html_url: String,
    pub id: i64,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub owner: Option<DeploymentReviewApprovedReviewersReviewer>,
    pub private: bool,
    pub pulls_url: String,
    pub releases_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub url: String,
}
impl From<&WorkflowRunInProgressWorkflowRunRepository>
    for WorkflowRunInProgressWorkflowRunRepository
{
    fn from(value: &WorkflowRunInProgressWorkflowRunRepository) -> Self {
        value.clone()
    }
}
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorkflowRunInProgressWorkflowRunStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "requested")]
    Requested,
}
impl From<&WorkflowRunInProgressWorkflowRunStatus> for WorkflowRunInProgressWorkflowRunStatus {
    fn from(value: &WorkflowRunInProgressWorkflowRunStatus) -> Self {
        *value
    }
}
impl ::std::fmt::Display for WorkflowRunInProgressWorkflowRunStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Completed => write!(f, "completed"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Pending => write!(f, "pending"),
            Self::Queued => write!(f, "queued"),
            Self::Requested => write!(f, "requested"),
        }
    }
}
impl std::str::FromStr for WorkflowRunInProgressWorkflowRunStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
        match value {
            "completed" => Ok(Self::Completed),
            "in_progress" => Ok(Self::InProgress),
            "pending" => Ok(Self::Pending),
            "queued" => Ok(Self::Queued),
            "requested" => Ok(Self::Requested),
            _ => Err("invalid value".into()),
        }
    }
}
impl std::convert::TryFrom<&str> for WorkflowRunInProgressWorkflowRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for WorkflowRunInProgressWorkflowRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for WorkflowRunInProgressWorkflowRunStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowRunRequestedWorkflowRun {
    pub actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub artifacts_url: String,
    pub cancel_url: String,
    pub check_suite_id: i64,
    pub check_suite_node_id: String,
    pub check_suite_url: String,
    pub conclusion: Option<WorkflowRunCompletedWorkflowRunConclusion>,
    pub created_at: String,
    pub display_title: String,
    pub event: String,
    pub head_branch: Option<String>,
    pub head_commit: WorkflowRunCompletedWorkflowRunHeadCommit,
    pub head_repository: WorkflowRunCompletedWorkflowRunHeadRepository,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    pub jobs_url: String,
    pub logs_url: String,
    pub name: Option<String>,
    pub node_id: String,
    pub path: String,
    pub previous_attempt_url: Option<String>,
    pub pull_requests: Vec<WorkflowRunRequestedWorkflowRunPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_workflows: Option<Vec<DeploymentStatusCreatedWorkflowRunReferencedWorkflows>>,
    pub repository: WorkflowRunCompletedWorkflowRunHeadRepository,
    pub rerun_url: String,
    pub run_attempt: i64,
    pub run_number: i64,
    pub run_started_at: String,
    pub status: DeploymentStatusCreatedWorkflowRunStatus,
    pub triggering_actor: Option<DeploymentStatusCreatedDeploymentCreator>,
    pub updated_at: String,
    pub url: String,
    pub workflow_id: i64,
    pub workflow_url: String,
}

impl From<&WorkflowRunRequestedWorkflowRun> for WorkflowRunRequestedWorkflowRun {
    fn from(value: &WorkflowRunRequestedWorkflowRun) -> Self {
        value.clone()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkflowRunRequestedWorkflowRunPullRequests {
    pub base: DeploymentStatusCreatedWorkflowRunPullRequestsBase,
    pub head: DeploymentStatusCreatedWorkflowRunPullRequestsBase,
    pub id: f64,
    pub number: f64,
    pub url: String,
}
impl From<&WorkflowRunRequestedWorkflowRunPullRequests>
    for WorkflowRunRequestedWorkflowRunPullRequests
{
    fn from(value: &WorkflowRunRequestedWorkflowRunPullRequests) -> Self {
        value.clone()
    }
}
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn deployment_protection_rule_requested_pull_requests_head_repo_visibility() -> String
    {
        "public".to_string()
    }
    pub(super) fn discussion_transferred_changes_new_repository_visibility() -> String {
        "public".to_string()
    }
}
