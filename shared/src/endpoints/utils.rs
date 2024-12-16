use github_webhook_body::*;

use crate::types::installation::InstallationId;

pub trait GetInstallationIdFromWebhookBody {
    fn get_installation_id(&self) -> Option<InstallationId>;
}

impl GetInstallationIdFromWebhookBody for WebhookBody {
    fn get_installation_id(&self) -> Option<InstallationId> {
        match self {
            // Variants that are structs and have `installation` as an `Option<>`:
            WebhookBody::SecurityAndAnalysis(SecurityAndAnalysis { installation, .. })
            | WebhookBody::Status(Status { installation, .. })
            | WebhookBody::Fork(Fork { installation, .. })
            | WebhookBody::Gollum(Gollum { installation, .. })
            | WebhookBody::Public(Public { installation, .. })
            | WebhookBody::PageBuild(PageBuild { installation, .. })
            | WebhookBody::Create(Create { installation, .. })
            | WebhookBody::WorkflowDispatch(WorkflowDispatch { installation, .. })
            | WebhookBody::Delete(Delete { installation, .. })
            | WebhookBody::TeamAdd(TeamAdd { installation, .. })
            | WebhookBody::Push(Push { installation, .. })
            | WebhookBody::DeploymentProtectionRule(DeploymentProtectionRule {
                installation,
                ..
            })
            | WebhookBody::SecretScanningAlertLocation(SecretScanningAlertLocation {
                installation,
                ..
            })
            | WebhookBody::DeploymentStatus(DeploymentStatus::Created { installation, .. }) => {
                installation.as_ref().map(|i| i.id)
            }

            // Variants that are structs and have `installation` as not an `Option<>`:
            WebhookBody::RepositoryDispatch(RepositoryDispatch { installation, .. }) => {
                Some(installation.id)
            }
            WebhookBody::PersonalAccessTokenRequest(personal_access_token_request) => {
                match personal_access_token_request {
                    PersonalAccessTokenRequest::Approved { installation, .. }
                    | PersonalAccessTokenRequest::Cancelled { installation, .. }
                    | PersonalAccessTokenRequest::Denied { installation, .. } => {
                        Some(installation.id)
                    }
                    PersonalAccessTokenRequest::Created { installation, .. } => {
                        installation.as_ref().map(|i| i.id)
                    }
                }
            }
            WebhookBody::Membership(membership) => match membership {
                Membership::Added { installation, .. }
                | Membership::Removed { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::Milestone(milestone) => match milestone {
                Milestone::Closed { installation, .. }
                | Milestone::Created { installation, .. }
                | Milestone::Deleted { installation, .. }
                | Milestone::Edited { installation, .. }
                | Milestone::Opened { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::Deployment(deployment) => match deployment {
                Deployment::Created { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::DeploymentReview(deployment_review) => match deployment_review {
                DeploymentReview::Approved { installation, .. }
                | DeploymentReview::Rejected { installation, .. }
                | DeploymentReview::Requested { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::Project(project) => match project {
                Project::Closed { installation, .. }
                | Project::Created { installation, .. }
                | Project::Deleted { installation, .. }
                | Project::Edited { installation, .. }
                | Project::Reopened { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::IssueComment(issue_comment) => match issue_comment {
                IssueComment::Created { installation, .. }
                | IssueComment::Deleted { installation, .. }
                | IssueComment::Edited { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::BranchProtectionRule(branch_protection_rule) => {
                match branch_protection_rule {
                    BranchProtectionRule::Created { installation, .. }
                    | BranchProtectionRule::Deleted { installation, .. }
                    | BranchProtectionRule::Edited { installation, .. } => {
                        installation.as_ref().map(|i| i.id)
                    }
                }
            }
            WebhookBody::PullRequestReviewComment(pull_request_review_comment) => {
                match pull_request_review_comment {
                    PullRequestReviewComment::Created { installation, .. }
                    | PullRequestReviewComment::Deleted { installation, .. }
                    | PullRequestReviewComment::Edited { installation, .. } => {
                        installation.as_ref().map(|i| i.id)
                    }
                }
            }
            WebhookBody::Member(member) => match member {
                Member::Added { installation, .. }
                | Member::Edited { installation, .. }
                | Member::Removed { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::Repository(repository) => match repository {
                Repository::Archived { installation, .. }
                | Repository::Created { installation, .. }
                | Repository::Deleted { installation, .. }
                | Repository::Edited { installation, .. }
                | Repository::Privatized { installation, .. }
                | Repository::Publicized { installation, .. }
                | Repository::Renamed { installation, .. }
                | Repository::Transferred { installation, .. }
                | Repository::Unarchived { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::SubIssues(sub_issues) => match sub_issues {
                SubIssues::ParentIssueAdded { installation, .. }
                | SubIssues::ParentIssueRemoved { installation, .. }
                | SubIssues::SubIssueAdded { installation, .. }
                | SubIssues::SubIssueRemoved { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::BranchProtectionConfiguration(branch_protection_configuration) => {
                match branch_protection_configuration {
                    BranchProtectionConfiguration::Disabled { installation, .. }
                    | BranchProtectionConfiguration::Enabled { installation, .. } => {
                        installation.as_ref().map(|i| i.id)
                    }
                }
            }
            WebhookBody::ProjectColumn(project_column) => match project_column {
                ProjectColumn::Created { installation, .. }
                | ProjectColumn::Deleted { installation, .. }
                | ProjectColumn::Edited { installation, .. }
                | ProjectColumn::Moved { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::DiscussionComment(discussion_comment) => match discussion_comment {
                DiscussionComment::Created { installation, .. }
                | DiscussionComment::Deleted { installation, .. }
                | DiscussionComment::Edited { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::Release(release) => match release {
                Release::Created { installation, .. }
                | Release::Deleted { installation, .. }
                | Release::Edited { installation, .. }
                | Release::Prereleased { installation, .. }
                | Release::Published { installation, .. }
                | Release::Released { installation, .. }
                | Release::Unpublished { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::OrgBlock(org_block) => match org_block {
                OrgBlock::Blocked { installation, .. }
                | OrgBlock::Unblocked { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::SecretScanningAlert(secret_scanning_alert) => {
                match secret_scanning_alert {
                    SecretScanningAlert::Created { installation, .. }
                    | SecretScanningAlert::PubliclyLeaked { installation, .. }
                    | SecretScanningAlert::Reopened { installation, .. }
                    | SecretScanningAlert::Resolved { installation, .. }
                    | SecretScanningAlert::Validated { installation, .. } => {
                        installation.as_ref().map(|i| i.id)
                    }
                }
            }
            WebhookBody::Issues(issues) => match issues {
                Issues::Assigned { installation, .. }
                | Issues::Closed { installation, .. }
                | Issues::Deleted { installation, .. }
                | Issues::Demilestoned { installation, .. }
                | Issues::Edited { installation, .. }
                | Issues::Labeled { installation, .. }
                | Issues::Locked { installation, .. }
                | Issues::Milestoned { installation, .. }
                | Issues::Opened { installation, .. }
                | Issues::Pinned { installation, .. }
                | Issues::Reopened { installation, .. }
                | Issues::Transferred { installation, .. }
                | Issues::Unassigned { installation, .. }
                | Issues::Unlabeled { installation, .. }
                | Issues::Unlocked { installation, .. }
                | Issues::Unpinned { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::RepositoryAdvisory(repository_advisory) => match repository_advisory {
                RepositoryAdvisory::Published { installation, .. }
                | RepositoryAdvisory::Reported { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::Package(package) => match package {
                Package::Published { installation, .. } | Package::Updated { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::PullRequestReview(pull_request_review) => match pull_request_review {
                PullRequestReview::Dismissed { installation, .. }
                | PullRequestReview::Edited { installation, .. }
                | PullRequestReview::Submitted { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::CodeScanningAlert(code_scanning_alert) => match code_scanning_alert {
                CodeScanningAlert::AppearedInBranch { installation, .. }
                | CodeScanningAlert::ClosedByUser { installation, .. }
                | CodeScanningAlert::Created { installation, .. }
                | CodeScanningAlert::Fixed { installation, .. }
                | CodeScanningAlert::Reopened { installation, .. }
                | CodeScanningAlert::ReopenedByUser { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::Watch(Watch::Started { installation, .. }) => {
                installation.as_ref().map(|i| i.id)
            }
            WebhookBody::CommitComment(commit_comment) => match commit_comment {
                CommitComment::Created { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::Organization(organization) => match organization {
                Organization::Deleted { installation, .. }
                | Organization::MemberAdded { installation, .. }
                | Organization::MemberInvited { installation, .. }
                | Organization::MemberRemoved { installation, .. }
                | Organization::Renamed { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::WorkflowRun(workflow_run) => match workflow_run {
                WorkflowRun::Completed { installation, .. }
                | WorkflowRun::InProgress { installation, .. }
                | WorkflowRun::Requested { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::CustomPropertyValues(custom_property_values) => {
                match custom_property_values {
                    CustomPropertyValues::Updated { installation, .. } => {
                        installation.as_ref().map(|i| i.id)
                    }
                }
            }
            WebhookBody::DeployKey(deploy_key) => match deploy_key {
                DeployKey::Created { installation, .. }
                | DeployKey::Deleted { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::Star(star) => match star {
                Star::Created { installation, .. } | Star::Deleted { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::RegistryPackage(registry_package) => match registry_package {
                RegistryPackage::Published { installation, .. }
                | RegistryPackage::Updated { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::MergeGroup(merge_group) => match merge_group {
                MergeGroup::ChecksRequested { installation, .. }
                | MergeGroup::Destroyed { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::CheckRun(check_run) => match check_run {
                CheckRun::Completed { installation, .. }
                | CheckRun::Created { installation, .. }
                | CheckRun::RequestedAction { installation, .. }
                | CheckRun::Rerequested { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::InstallationTarget(installation_target) => match installation_target {
                InstallationTarget::Renamed { installation, .. } => Some(installation.id),
            },
            WebhookBody::Label(label) => match label {
                Label::Created { installation, .. }
                | Label::Deleted { installation, .. }
                | Label::Edited { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::DependabotAlert(dependabot_alert) => match dependabot_alert {
                DependabotAlert::AutoDismissed { installation, .. }
                | DependabotAlert::AutoReopened { installation, .. }
                | DependabotAlert::Created { installation, .. }
                | DependabotAlert::Dismissed { installation, .. }
                | DependabotAlert::Fixed { installation, .. }
                | DependabotAlert::Reintroduced { installation, .. }
                | DependabotAlert::Reopened { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::ProjectCard(project_card) => match project_card {
                ProjectCard::Converted { installation, .. }
                | ProjectCard::Created { installation, .. }
                | ProjectCard::Deleted { installation, .. }
                | ProjectCard::Edited { installation, .. }
                | ProjectCard::Moved { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::Team(team) => match team {
                Team::AddedToRepository { installation, .. }
                | Team::Created { installation, .. }
                | Team::Deleted { installation, .. }
                | Team::Edited { installation, .. }
                | Team::RemovedFromRepository { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::CheckSuite(check_suite) => match check_suite {
                CheckSuite::Completed { installation, .. }
                | CheckSuite::Requested { installation, .. }
                | CheckSuite::Rerequested { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::WorkflowJob(workflow_job) => match workflow_job {
                WorkflowJob::Completed { installation, .. }
                | WorkflowJob::InProgress { installation, .. }
                | WorkflowJob::Queued { installation, .. }
                | WorkflowJob::Waiting { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::InstallationRepositories(installation_repositories) => {
                match installation_repositories {
                    InstallationRepositories::Added { installation, .. }
                    | InstallationRepositories::Removed { installation, .. } => {
                        Some(installation.id)
                    }
                }
            }
            WebhookBody::PullRequestReviewThread(pull_request_review_thread) => {
                match pull_request_review_thread {
                    PullRequestReviewThread::Resolved { installation, .. }
                    | PullRequestReviewThread::Unresolved { installation, .. } => {
                        installation.as_ref().map(|i| i.id)
                    }
                }
            }
            WebhookBody::PullRequest(pull_request) => match pull_request {
                PullRequest::Assigned { installation, .. }
                | PullRequest::AutoMergeDisabled { installation, .. }
                | PullRequest::AutoMergeEnabled { installation, .. }
                | PullRequest::Closed { installation, .. }
                | PullRequest::ConvertedToDraft { installation, .. }
                | PullRequest::Dequeued { installation, .. }
                | PullRequest::Edited { installation, .. }
                | PullRequest::Enqueued { installation, .. }
                | PullRequest::Labeled { installation, .. }
                | PullRequest::Locked { installation, .. }
                | PullRequest::Opened { installation, .. }
                | PullRequest::ReadyForReview { installation, .. }
                | PullRequest::Reopened { installation, .. }
                | PullRequest::ReviewRequestRemoved { installation, .. }
                | PullRequest::ReviewRequested { installation, .. }
                | PullRequest::Synchronize { installation, .. }
                | PullRequest::Unassigned { installation, .. }
                | PullRequest::Unlabeled { installation, .. }
                | PullRequest::Unlocked { installation, .. } => installation.as_ref().map(|i| i.id),
                PullRequest::Demilestoned { .. } | PullRequest::Milestoned { .. } => None,
            },
            WebhookBody::SecurityAdvisory(security_advisory) => match security_advisory {
                SecurityAdvisory::Published { installation, .. }
                | SecurityAdvisory::Updated { installation, .. }
                | SecurityAdvisory::Withdrawn { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::Meta(meta) => match meta {
                Meta::Deleted { installation, .. } => installation.as_ref().map(|i| i.id),
            },
            WebhookBody::CustomProperty(custom_property) => match custom_property {
                CustomProperty::Created { installation, .. }
                | CustomProperty::Deleted { installation, .. }
                | CustomProperty::Updated { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::RepositoryRuleset(repository_ruleset) => match repository_ruleset {
                RepositoryRuleset::Created { installation, .. }
                | RepositoryRuleset::Deleted { installation, .. }
                | RepositoryRuleset::Edited { installation, .. } => {
                    installation.as_ref().map(|i| i.id)
                }
            },
            WebhookBody::Installation(installation) => match installation {
                Installation::Created { installation, .. }
                | Installation::Deleted { installation, .. }
                | Installation::NewPermissionsAccepted { installation, .. }
                | Installation::Suspend { installation, .. }
                | Installation::Unsuspend { installation, .. } => Some(installation.id),
            },
            WebhookBody::Discussion(discussion) => match discussion {
                Discussion::Answered { installation, .. }
                | Discussion::CategoryChanged { installation, .. }
                | Discussion::Closed { installation, .. }
                | Discussion::Created { installation, .. }
                | Discussion::Deleted { installation, .. }
                | Discussion::Edited { installation, .. }
                | Discussion::Labeled { installation, .. }
                | Discussion::Locked { installation, .. }
                | Discussion::Pinned { installation, .. }
                | Discussion::Reopened { installation, .. }
                | Discussion::Transferred { installation, .. }
                | Discussion::Unlabeled { installation, .. }
                | Discussion::Unlocked { installation, .. }
                | Discussion::Unpinned { installation, .. } => installation.as_ref().map(|i| i.id),
                Discussion::Unanswered { .. } => None,
            },

            WebhookBody::GithubAppAuthorization(_) => None,
            WebhookBody::Ping(_) => None,
        }
        .map(|i| i.into())
    }
}
