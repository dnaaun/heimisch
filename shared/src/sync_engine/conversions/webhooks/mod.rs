use std::convert::Infallible;

use github_webhook_body::*;
use ignore_untyped::ignore_untyped;

mod ignore_untyped {
    use github_webhook_body::Untyped;

    /// Used to statically guarnatee that we're ignoring only when something is `Untyped` in the
    /// generated types for the webhook.
    pub fn ignore_untyped<T: UntypedTrait>(_: T) {}

    pub trait UntypedTrait {}

    impl UntypedTrait for Untyped {}
    impl<T: UntypedTrait> UntypedTrait for Option<T> {}
    impl<T: UntypedTrait> UntypedTrait for Vec<T> {}
    impl<T: UntypedTrait> UntypedTrait for &T {}
    impl<T: UntypedTrait> UntypedTrait for (T,) {}
    impl<T1: UntypedTrait, T2: UntypedTrait> UntypedTrait for (T1, T2) {}
}

use crate::{
    avail::Avail,
    sync_engine::{
        changes::{AddChanges, Changes},
        conversions::{InfallibleToDbNoOtherChanges, ToDb, ToDbNoOtherChanges},
    },
    types,
};

impl ToDb for IssueCommentCreatedIssueActiveLockReason {
    type OtherChanges = ();
    type DbType = github_api::models::issue::ActiveLockReason;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, ()), Self::Error> {
        use github_api::models::issue::ActiveLockReason::*;
        Ok((
            match self {
                IssueCommentCreatedIssueActiveLockReason::OffTopic => OffTopic,
                IssueCommentCreatedIssueActiveLockReason::Resolved => Resolved,
                IssueCommentCreatedIssueActiveLockReason::Spam => Spam,
                IssueCommentCreatedIssueActiveLockReason::TooHeated => TooHeated,
            },
            Default::default(),
        ))
    }
}

impl ToDb for DeploymentStatusCreatedDeploymentCreatorType {
    type OtherChanges = ();
    type DbType = github_api::models::user::Type;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        use github_api::models::user::Type::*;
        Ok((
            match self {
                DeploymentStatusCreatedDeploymentCreatorType::Bot => Bot,
                DeploymentStatusCreatedDeploymentCreatorType::Organization => Organization,
                DeploymentStatusCreatedDeploymentCreatorType::User => User,
            },
            Default::default(),
        ))
    }
}

impl ToDb for MilestoneClosedMilestoneCreatorType {
    type OtherChanges = ();
    type DbType = github_api::models::user::Type;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        use github_api::models::user::Type::*;
        Ok((
            match self {
                MilestoneClosedMilestoneCreatorType::Bot => Bot,
                MilestoneClosedMilestoneCreatorType::Organization => Organization,
                MilestoneClosedMilestoneCreatorType::User => User,
                MilestoneClosedMilestoneCreatorType::Mannequin => Mannequin,
            },
            Default::default(),
        ))
    }
}

impl ToDb for DeploymentReviewApprovedReviewersReviewer {
    type OtherChanges = ();
    type DbType = types::user::User;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let DeploymentReviewApprovedReviewersReviewer {
            avatar_url,
            deleted,
            email,
            events_url,
            followers_url,
            following_url,
            gists_url,
            gravatar_id,
            html_url,
            id,
            login,
            name,
            node_id,
            organizations_url,
            received_events_url,
            repos_url,
            site_admin,
            starred_url,
            subscriptions_url,
            type_,
            url,
        } = self;
        let id = types::user::UserId::from(id);
        Ok((
            types::user::User {
                avatar_url: Avail::from_option(avatar_url),
                bio: Avail::No,
                blog: Avail::No,
                business_plus: Avail::No,
                collaborators: Avail::No,
                company: Avail::No,
                created_at: Avail::No,
                disk_usage: Avail::No,
                deleted: Avail::from_option(deleted),
                email: email.into(),
                events_url: Avail::from_option(events_url),
                followers: Avail::No,
                followers_url: Avail::from_option(followers_url),
                following: Avail::No,
                following_url: Avail::from_option(following_url),
                gists_url: Avail::from_option(gists_url),
                gravatar_id: gravatar_id.into(),
                hireable: Avail::No,
                html_url: Avail::from_option(html_url),
                id,
                ldap_dn: Avail::No,
                location: Avail::No,
                login,
                name: name.into(),
                node_id: Avail::from_option(node_id),
                notification_email: Avail::No,
                organizations_url: Avail::from_option(organizations_url),
                owned_private_repos: Avail::No,
                private_gists: Avail::No,
                public_gists: Avail::No,
                public_repos: Avail::No,
                received_events_url: Avail::from_option(received_events_url),
                repos_url: Avail::from_option(repos_url),
                site_admin: Avail::from_option(site_admin),
                starred_url: Avail::from_option(starred_url),
                subscriptions_url: Avail::from_option(subscriptions_url),
                total_private_repos: Avail::No,
                twitter_username: Avail::No,
                two_factor_authentication: Avail::No,
                r#type: Avail::from_option(type_.map(|t| t.to_db_type())),
                updated_at: Avail::No,
                url: Avail::from_option(url),
                user_view_type: Avail::No,
                starred_at: Avail::No,
            },
            Default::default(),
        ))
    }
}

impl ToDb for IssuesDemilestonedIssueAssignee {
    type OtherChanges = ();
    type DbType = types::user::User;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let IssuesDemilestonedIssueAssignee {
            avatar_url,
            deleted,
            email,
            events_url,
            followers_url,
            following_url,
            gists_url,
            gravatar_id,
            html_url,
            id,
            login,
            name,
            node_id,
            organizations_url,
            received_events_url,
            repos_url,
            site_admin,
            starred_url,
            subscriptions_url,
            type_,
            url,
        } = self;
        let id = types::user::UserId::from(id);
        Ok((
            types::user::User {
                avatar_url: Avail::from_option(avatar_url),
                bio: Avail::No,
                blog: Avail::No,
                business_plus: Avail::No,
                collaborators: Avail::No,
                company: Avail::No,
                created_at: Avail::No,
                disk_usage: Avail::No,
                deleted: Avail::from_option(deleted),
                email: email.into(),
                events_url: Avail::from_option(events_url),
                followers: Avail::No,
                followers_url: Avail::from_option(followers_url),
                following: Avail::No,
                following_url: Avail::from_option(following_url),
                gists_url: Avail::from_option(gists_url),
                gravatar_id: gravatar_id.into(),
                hireable: Avail::No,
                html_url: Avail::from_option(html_url),
                id,
                ldap_dn: Avail::No,
                location: Avail::No,
                login,
                name: name.into(),
                node_id: Avail::from_option(node_id),
                notification_email: Avail::No,
                organizations_url: Avail::from_option(organizations_url),
                owned_private_repos: Avail::No,
                private_gists: Avail::No,
                public_gists: Avail::No,
                public_repos: Avail::No,
                received_events_url: Avail::from_option(received_events_url),
                repos_url: Avail::from_option(repos_url),
                site_admin: Avail::from_option(site_admin),
                starred_url: Avail::from_option(starred_url),
                subscriptions_url: Avail::from_option(subscriptions_url),
                total_private_repos: Avail::No,
                twitter_username: Avail::No,
                two_factor_authentication: Avail::No,
                r#type: Avail::from_option(type_.map(|t| t.to_db_type())),
                updated_at: Avail::No,
                url: Avail::from_option(url),
                starred_at: Avail::No,
                user_view_type: Avail::No,
            },
            Default::default(),
        ))
    }
}
impl ToDb for DeploymentStatusCreatedDeploymentCreator {
    type OtherChanges = ();
    type DbType = types::user::User;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let DeploymentStatusCreatedDeploymentCreator {
            avatar_url,
            deleted,
            email,
            events_url,
            followers_url,
            following_url,
            gists_url,
            gravatar_id,
            html_url,
            id,
            login,
            name,
            node_id,
            organizations_url,
            received_events_url,
            repos_url,
            site_admin,
            starred_url,
            subscriptions_url,
            type_,
            url,
            user_view_type,
        } = self;
        let id = types::user::UserId::from(id);
        Ok((
            types::user::User {
                avatar_url: Avail::from_option(avatar_url),
                bio: Avail::No,
                blog: Avail::No,
                business_plus: Avail::No,
                collaborators: Avail::No,
                company: Avail::No,
                created_at: Avail::No,
                disk_usage: Avail::No,
                deleted: Avail::from_option(deleted),
                email: email.into(),
                events_url: Avail::from_option(events_url),
                followers: Avail::No,
                followers_url: Avail::from_option(followers_url),
                following: Avail::No,
                following_url: Avail::from_option(following_url),
                gists_url: Avail::from_option(gists_url),
                gravatar_id: gravatar_id.into(),
                hireable: Avail::No,
                html_url: Avail::from_option(html_url),
                id,
                ldap_dn: Avail::No,
                location: Avail::No,
                login,
                name: name.into(),
                node_id: Avail::from_option(node_id),
                notification_email: Avail::No,
                organizations_url: Avail::from_option(organizations_url),
                owned_private_repos: Avail::No,
                private_gists: Avail::No,
                public_gists: Avail::No,
                public_repos: Avail::No,
                received_events_url: Avail::from_option(received_events_url),
                repos_url: Avail::from_option(repos_url),
                site_admin: Avail::from_option(site_admin),
                starred_url: Avail::from_option(starred_url),
                subscriptions_url: Avail::from_option(subscriptions_url),
                total_private_repos: Avail::No,
                twitter_username: Avail::No,
                two_factor_authentication: Avail::No,
                r#type: Avail::from_option(type_.map(|t| t.to_db_type())),
                updated_at: Avail::No,
                url: Avail::from_option(url),
                user_view_type: user_view_type.into(),
                starred_at: Avail::No,
            },
            Default::default(),
        ))
    }
}

impl ToDb for DiscussionAnsweredAnswerAuthorAssociation {
    type OtherChanges = ();
    type DbType = github_api::models::AuthorAssociation;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        use github_api::models::AuthorAssociation::*;
        Ok((
            match self {
                DiscussionAnsweredAnswerAuthorAssociation::Collaborator => Collaborator,
                DiscussionAnsweredAnswerAuthorAssociation::Contributor => Contributor,
                DiscussionAnsweredAnswerAuthorAssociation::FirstTimer => FirstTimer,
                DiscussionAnsweredAnswerAuthorAssociation::FirstTimeContributor => {
                    FirstTimeContributor
                }
                DiscussionAnsweredAnswerAuthorAssociation::Mannequin => Mannequin,
                DiscussionAnsweredAnswerAuthorAssociation::Member => Member,
                DiscussionAnsweredAnswerAuthorAssociation::None => None,
                DiscussionAnsweredAnswerAuthorAssociation::Owner => Owner,
            },
            Default::default(),
        ))
    }
}

impl ToDb for MilestoneClosedMilestoneCreator {
    type OtherChanges = ();
    type DbType = types::user::User;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let MilestoneClosedMilestoneCreator {
            avatar_url,
            deleted,
            email,
            events_url,
            followers_url,
            following_url,
            gists_url,
            gravatar_id,
            html_url,
            id,
            login,
            name,
            node_id,
            organizations_url,
            received_events_url,
            repos_url,
            site_admin,
            starred_url,
            subscriptions_url,
            type_,
            url,
            user_view_type,
        } = self;
        let id = types::user::UserId::from(id);
        Ok((
            types::user::User {
                avatar_url: Avail::from_option(avatar_url),
                bio: Avail::No,
                blog: Avail::No,
                business_plus: Avail::No,
                collaborators: Avail::No,
                company: Avail::No,
                created_at: Avail::No,
                disk_usage: Avail::No,
                deleted: Avail::from_option(deleted),
                email: email.into(),
                events_url: Avail::from_option(events_url),
                followers: Avail::No,
                followers_url: Avail::from_option(followers_url),
                following: Avail::No,
                following_url: Avail::from_option(following_url),
                gists_url: Avail::from_option(gists_url),
                gravatar_id: gravatar_id.into(),
                hireable: Avail::No,
                html_url: Avail::from_option(html_url),
                id,
                ldap_dn: Avail::No,
                location: Avail::No,
                login,
                name: name.into(),
                node_id: Avail::from_option(node_id),
                notification_email: Avail::No,
                organizations_url: Avail::from_option(organizations_url),
                owned_private_repos: Avail::No,
                private_gists: Avail::No,
                public_gists: Avail::No,
                public_repos: Avail::No,
                received_events_url: Avail::from_option(received_events_url),
                repos_url: Avail::from_option(repos_url),
                site_admin: Avail::from_option(site_admin),
                starred_url: Avail::from_option(starred_url),
                subscriptions_url: Avail::from_option(subscriptions_url),
                total_private_repos: Avail::No,
                twitter_username: Avail::No,
                two_factor_authentication: Avail::No,
                r#type: Avail::from_option(type_.map(|t| t.to_db_type())),
                updated_at: Avail::No,
                url: Avail::from_option(url),
                user_view_type: user_view_type.into(),
                starred_at: Avail::No,
            },
            Default::default(),
        ))
    }
}

impl ToDb for DiscussionLabeledLabel {
    type OtherChanges = ();
    type DbType = types::label::Label;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub enum ConversionError {
    Merge(crate::avail::MergeError),
    Json(serde_json::Error),
    Jiff(jiff::Error),
}

impl From<crate::avail::MergeError> for ConversionError {
    fn from(value: crate::avail::MergeError) -> Self {
        Self::Merge(value)
    }
}

impl From<serde_json::Error> for ConversionError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<jiff::Error> for ConversionError {
    fn from(value: jiff::Error) -> Self {
        Self::Jiff(value)
    }
}

impl ToDb for IssueCommentCreatedIssuePullRequest {
    type OtherChanges = ();
    type DbType = github_api::models::WebhooksIssuePullRequest;

    type Error = jiff::Error;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let IssueCommentCreatedIssuePullRequest {
            diff_url,
            html_url,
            merged_at,
            patch_url,
            url,
        } = self;
        let pr = github_api::models::WebhooksIssuePullRequest {
            diff_url,
            html_url,
            merged_at: Some(merged_at.map(|m| (m).parse()).transpose()?),
            patch_url,
            url,
        };
        Ok((pr, Default::default()))
    }
}

impl ToDb for DiscussionAnsweredAnswerReactions {
    type OtherChanges = ();
    type DbType = github_api::models::Reactions;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let DiscussionAnsweredAnswerReactions {
            confused,
            eyes,
            heart,
            hooray,
            laugh,
            minus1,
            plus1,
            rocket,
            total_count,
            url,
        } = self;
        Ok((
            github_api::models::Reactions {
                plus_1: plus1,
                _1: minus1,
                confused,
                eyes,
                heart,
                hooray,
                laugh,
                rocket,
                total_count,
                url,
            },
            Default::default(),
        ))
    }
}

impl ToDb for (types::repository::RepositoryId, IssuesAssignedIssue) {
    type DbType = types::issue::Issue;
    type OtherChanges = Changes;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, Changes), Self::Error> {
        let (repository_id, issues_assigned_issue) = self;
        let IssuesAssignedIssue {
            active_lock_reason,
            assignee,

            // TODO: THIS IS A MASSIVE TODO!! The updating of `assignees` is important for actual
            // usefulness of the app. But currently, this is untyped (most likely) because the
            // messed up JSON that we reverse engineer the types from (instead of using
            // `x-webhooks` in the OpenAPI Spec (Hey! I didn't know that existed at first! Don't
            // judge me!)) doesn't contain the types.
            assignees: _,
            author_association,
            body,
            closed_at,
            comments,
            comments_url,
            created_at,
            draft,
            events_url,
            html_url,
            id,
            labels,
            labels_url,
            locked,
            milestone,
            node_id,
            number,

            // This might not contain the ID (according to the types), in which case we can't
            // currently construct a GithubApp for it because we require ids on all the primary db
            // thingys. Plus, we don't care about `performed_via_github_app` too much for now (I
            // don't even know where it would even surface in the app yet).
            performed_via_github_app: _,

            pull_request,
            reactions,
            repository_url,
            state,
            state_reason,
            timeline_url,
            title,
            updated_at,
            url,
            user,
        } = issues_assigned_issue;
        let user = user.map(|u| u.to_db_type());
        let assignee = assignee.map(|a| a.to_db_type());
        let milestone_and_changes = milestone
            .map(|m| m.try_to_db_type_and_other_changes())
            .transpose()?;
        let issue = types::issue::Issue {
            active_lock_reason: active_lock_reason.map(|a| a.to_db_type()).into(),
            assignee_id: assignee.as_ref().map(|a| a.id).into(),
            assignee_ids: Avail::No,
            author_association: author_association.to_db_type().into(),
            body: body.into(),
            body_html: Avail::No,
            body_text: Avail::No,
            closed_at: Avail::Yes(closed_at.map(|s| s.parse()).transpose()?),
            closed_by_id: Avail::No,
            comments: comments.into(),
            comments_url: comments_url.into(),
            created_at: Avail::Yes(created_at.parse()?),
            draft: draft.into(),
            events_url: events_url.into(),
            html_url: html_url.into(),
            id: types::issue::IssueId::from(id),
            labels: Avail::Yes(labels.into_iter().map(|l| l.to_db_type()).collect()),
            labels_url: labels_url.into(),
            locked: Avail::from_option(locked),
            milestone_id: Avail::Yes(milestone_and_changes.as_ref().map(|(m, _)| m.id)),
            node_id: node_id.into(),
            number,
            performed_via_github_app_id: Avail::No,
            pull_request: Avail::Yes(pull_request.map(|p| p.try_to_db_type()).transpose()?),
            reactions: reactions.to_db_type().into(),
            repository_id,
            repository_url: repository_url.into(),
            state: Avail::from_option(state.map(|s| s.to_db_type())),
            state_reason: Avail::Yes(
                // TODO: This bridging between `String` and `StateReason` should be tested.
                state_reason
                    .map(|s| serde_json::from_str(&format!("\"{s}\"")))
                    .transpose()?,
            ),
            timeline_url: timeline_url.into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
            user_id: user.as_ref().map(|u| u.id).into(),
        };

        let mut changes = Changes::default();
        changes.add(assignee)?;
        changes.add(user)?;
        if let Some((milestone, changes_from_milestone)) = milestone_and_changes {
            changes.add(milestone)?;
            changes.add(changes_from_milestone)?;
        }

        Ok((issue, changes))
    }
}

impl ToDb for (types::repository::RepositoryId, IssuesClosedIssue) {
    type DbType = types::issue::Issue;
    type OtherChanges = Changes;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, Changes), Self::Error> {
        let (repository_id, issues_assigned_issue) = self;
        let IssuesClosedIssue {
            active_lock_reason,
            assignee,

            // TODO: THIS IS A MASSIVE TODO!! The updating of `assignees` is important for actual
            // usefulness of the app. But currently, this is untyped (most likely) because the
            // messed up JSON that we reverse engineer the types from (instead of using
            // `x-webhooks` in the OpenAPI Spec (Hey! I didn't know that existed at first! Don't
            // judge me!)) doesn't contain the types.
            assignees: _,
            author_association,
            body,
            closed_at,
            comments,
            comments_url,
            created_at,
            draft,
            events_url,
            html_url,
            id,
            labels,
            labels_url,
            locked,
            milestone,
            node_id,
            number,

            // This might not contain the ID (according to the types), in which case we can't
            // currently construct a GithubApp for it because we require ids on all the primary db
            // thingys. Plus, we don't care about `performed_via_github_app` too much for now (I
            // don't even know where it would even surface in the app yet).
            performed_via_github_app: _,

            pull_request,
            reactions,
            repository_url,
            state,
            state_reason,
            timeline_url,
            title,
            updated_at,
            url,
            user,
        } = issues_assigned_issue;
        let user = user.map(|u| u.to_db_type());
        ignore_untyped((assignee, labels));
        let milestone_and_changes = milestone
            .map(|m| m.try_to_db_type_and_other_changes())
            .transpose()?;
        let issue = types::issue::Issue {
            active_lock_reason: active_lock_reason.map(|a| a.to_db_type()).into(),
            assignee_id: Avail::No,
            assignee_ids: Avail::No,
            author_association: author_association.to_db_type().into(),
            body: body.into(),
            body_html: Avail::No,
            body_text: Avail::No,
            closed_at: Avail::Yes(closed_at.map(|s| (s).parse()).transpose()?),
            closed_by_id: Avail::No,
            comments: comments.into(),
            comments_url: comments_url.into(),
            created_at: Avail::Yes((created_at).parse()?),
            draft: draft.into(),
            events_url: events_url.into(),
            html_url: html_url.into(),
            id: types::issue::IssueId::from(id),
            labels: Avail::No,
            labels_url: labels_url.into(),
            locked: Avail::from_option(locked),
            milestone_id: Avail::Yes(milestone_and_changes.as_ref().map(|(m, _)| m.id)),
            node_id: node_id.into(),
            number,
            performed_via_github_app_id: Avail::No,
            pull_request: Avail::Yes(pull_request.map(|p| p.try_to_db_type()).transpose()?),
            reactions: reactions.to_db_type().into(),
            repository_id,
            repository_url: repository_url.into(),
            state: state.to_db_type().into(),
            state_reason: Avail::Yes(
                // TODO: This bridging between `String` and `StateReason` should be tested.
                state_reason
                    .map(|s| serde_json::from_str(&format!("\"{s}\"")))
                    .transpose()?,
            ),
            timeline_url: timeline_url.into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
            user_id: user.as_ref().map(|u| u.id).into(),
        };

        let mut changes = Changes::default();
        changes.add(user)?;
        if let Some((milestone, changes_from_milestone)) = milestone_and_changes {
            changes.add(milestone)?;
            changes.add(changes_from_milestone)?;
        }

        Ok((issue, changes))
    }
}
impl ToDb for (types::repository::RepositoryId, IssuesOpenedIssue) {
    type DbType = types::issue::Issue;
    type OtherChanges = Changes;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, Changes), Self::Error> {
        let (repository_id, issues_assigned_issue) = self;
        let IssuesOpenedIssue {
            active_lock_reason,
            assignee,

            // TODO: THIS IS A MASSIVE TODO!! The updating of `assignees` is important for actual
            // usefulness of the app. But currently, this is untyped (most likely) because the
            // messed up JSON that we reverse engineer the types from (instead of using
            // `x-webhooks` in the OpenAPI Spec (Hey! I didn't know that existed at first! Don't
            // judge me!)) doesn't contain the types.
            assignees: _,
            author_association,
            body,
            closed_at,
            comments,
            comments_url,
            created_at,
            draft,
            events_url,
            html_url,
            id,
            labels,
            labels_url,
            locked,
            milestone,
            node_id,
            number,

            // This might not contain the ID (according to the types), in which case we can't
            // currently construct a GithubApp for it because we require ids on all the primary db
            // thingys. Plus, we don't care about `performed_via_github_app` too much for now (I
            // don't even know where it would even surface in the app yet).
            performed_via_github_app: _,

            pull_request,
            reactions,
            repository_url,
            state,
            state_reason,
            timeline_url,
            title,
            updated_at,
            url,
            user,
        } = issues_assigned_issue;
        let user = user.map(|u| u.to_db_type());
        let assignee = assignee.map(|a| a.to_db_type());
        let milestone_and_changes = milestone
            .map(|m| m.try_to_db_type_and_other_changes())
            .transpose()?;
        let issue = types::issue::Issue {
            active_lock_reason: active_lock_reason.map(|a| a.to_db_type()).into(),
            assignee_id: assignee.as_ref().map(|a| a.id).into(),
            assignee_ids: Avail::No,
            author_association: author_association.to_db_type().into(),
            body: body.into(),
            body_html: Avail::No,
            body_text: Avail::No,
            closed_at: Avail::Yes(closed_at.map(|s| (s).parse()).transpose()?),
            closed_by_id: Avail::No,
            comments: comments.into(),
            comments_url: comments_url.into(),
            created_at: Avail::Yes(created_at.parse()?),
            draft: draft.into(),
            events_url: events_url.into(),
            html_url: html_url.into(),
            id: types::issue::IssueId::from(id),
            labels: Avail::Yes(labels.into_iter().map(|l| l.to_db_type()).collect()),
            labels_url: labels_url.into(),
            locked: Avail::from_option(locked),
            milestone_id: Avail::Yes(milestone_and_changes.as_ref().map(|(m, _)| m.id)),
            node_id: node_id.into(),
            number,
            performed_via_github_app_id: Avail::No,
            pull_request: Avail::Yes(pull_request.map(|p| p.try_to_db_type()).transpose()?),
            reactions: reactions.to_db_type().into(),
            repository_id,
            repository_url: repository_url.into(),
            state: Avail::from_option(state.map(|s| s.to_db_type())),
            state_reason: Avail::Yes(
                // TODO: This bridging between `String` and `StateReason` should be tested.
                state_reason
                    .map(|s| serde_json::from_str(&format!("\"{s}\"")))
                    .transpose()?,
            ),
            timeline_url: timeline_url.into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
            user_id: user.as_ref().map(|u| u.id).into(),
        };

        let mut changes = Changes::default();
        changes.add(assignee)?;
        changes.add(user)?;
        if let Some((milestone, changes_from_milestone)) = milestone_and_changes {
            changes.add(milestone)?;
            changes.add(changes_from_milestone)?;
        }

        Ok((issue, changes))
    }
}

impl ToDb for (types::repository::RepositoryId, IssuesReopenedIssue) {
    type DbType = types::issue::Issue;
    type OtherChanges = Changes;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, Changes), Self::Error> {
        let (repository_id, issues_assigned_issue) = self;
        let IssuesReopenedIssue {
            active_lock_reason,
            assignee,

            // TODO: THIS IS A MASSIVE TODO!! The updating of `assignees` is important for actual
            // usefulness of the app. But currently, this is untyped (most likely) because the
            // messed up JSON that we reverse engineer the types from (instead of using
            // `x-webhooks` in the OpenAPI Spec (Hey! I didn't know that existed at first! Don't
            // judge me!)) doesn't contain the types.
            assignees: _,
            author_association,
            body,
            closed_at,
            comments,
            comments_url,
            created_at,
            draft,
            events_url,
            html_url,
            id,
            labels,
            labels_url,
            locked,
            milestone,
            node_id,
            number,

            // This might not contain the ID (according to the types), in which case we can't
            // currently construct a GithubApp for it because we require ids on all the primary db
            // thingys. Plus, we don't care about `performed_via_github_app` too much for now (I
            // don't even know where it would even surface in the app yet).
            performed_via_github_app: _,

            pull_request,
            reactions,
            repository_url,
            state,
            state_reason,
            timeline_url,
            title,
            updated_at,
            url,
            user,
        } = issues_assigned_issue;

        let user = user.map(|u| u.to_db_type());
        let assignee = assignee.map(|a| a.to_db_type());
        let milestone_and_changes = milestone
            .map(|m| m.try_to_db_type_and_other_changes())
            .transpose()?;
        ignore_untyped(labels);

        let issue = types::issue::Issue {
            active_lock_reason: active_lock_reason.map(|a| a.to_db_type()).into(),
            assignee_id: assignee.as_ref().map(|a| a.id).into(),
            assignee_ids: Avail::No,
            author_association: author_association.to_db_type().into(),
            body: body.into(),
            body_html: Avail::No,
            body_text: Avail::No,
            closed_at: Avail::Yes(closed_at.map(|s| (s).parse()).transpose()?),
            closed_by_id: Avail::No,
            comments: comments.into(),
            comments_url: comments_url.into(),
            created_at: Avail::Yes((created_at).parse()?),
            draft: draft.into(),
            events_url: events_url.into(),
            html_url: html_url.into(),
            id: types::issue::IssueId::from(id),
            labels: Avail::No,
            labels_url: labels_url.into(),
            locked: Avail::from_option(locked),
            milestone_id: Avail::Yes(milestone_and_changes.as_ref().map(|(m, _)| m.id)),
            node_id: node_id.into(),
            number,
            performed_via_github_app_id: Avail::No,
            pull_request: Avail::Yes(pull_request.map(|p| p.try_to_db_type()).transpose()?),
            reactions: reactions.to_db_type().into(),
            repository_id,
            repository_url: repository_url.into(),
            state: state.to_db_type().into(),
            state_reason: Avail::Yes(
                // TODO: This bridging between `String` and `StateReason` should be tested.
                state_reason
                    .map(|s| serde_json::from_str(&format!("\"{s}\"")))
                    .transpose()?,
            ),
            timeline_url: timeline_url.into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
            user_id: user.as_ref().map(|u| u.id).into(),
        };

        let mut changes = Changes::default();
        changes.add(assignee)?;
        changes.add(user)?;
        if let Some((milestone, changes_from_milestone)) = milestone_and_changes {
            changes.add(milestone)?;
            changes.add(changes_from_milestone)?;
        }

        Ok((issue, changes))
    }
}
impl ToDb for (types::repository::RepositoryId, IssuesDeletedIssue) {
    type DbType = types::issue::Issue;
    type OtherChanges = Changes;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, Changes), Self::Error> {
        let (repository_id, issues_assigned_issue) = self;
        let IssuesDeletedIssue {
            active_lock_reason,
            assignee,

            // TODO: THIS IS A MASSIVE TODO!! The updating of `assignees` is important for actual
            // usefulness of the app. But currently, this is untyped (most likely) because the
            // messed up JSON that we reverse engineer the types from (instead of using
            // `x-webhooks` in the OpenAPI Spec (Hey! I didn't know that existed at first! Don't
            // judge me!)) doesn't contain the types.
            assignees: _,
            author_association,
            body,
            closed_at,
            comments,
            comments_url,
            created_at,
            draft,
            events_url,
            html_url,
            id,
            labels,
            labels_url,
            locked,
            milestone,
            node_id,
            number,

            // This might not contain the ID (according to the types), in which case we can't
            // currently construct a GithubApp for it because we require ids on all the primary db
            // thingys. Plus, we don't care about `performed_via_github_app` too much for now (I
            // don't even know where it would even surface in the app yet).
            performed_via_github_app: _,

            pull_request,
            reactions,
            repository_url,
            state,
            state_reason,
            timeline_url,
            title,
            updated_at,
            url,
            user,
        } = issues_assigned_issue;
        let user = user.map(|u| u.to_db_type());
        let assignee = assignee.map(|a| a.to_db_type());
        let milestone_and_changes = milestone
            .map(|m| m.try_to_db_type_and_other_changes())
            .transpose()?;
        let issue = types::issue::Issue {
            active_lock_reason: active_lock_reason.map(|a| a.to_db_type()).into(),
            assignee_id: assignee.as_ref().map(|a| a.id).into(),
            assignee_ids: Avail::No,
            author_association: author_association.to_db_type().into(),
            body: body.into(),
            body_html: Avail::No,
            body_text: Avail::No,
            closed_at: Avail::Yes(closed_at.map(|s| (s).parse()).transpose()?),
            closed_by_id: Avail::No,
            comments: comments.into(),
            comments_url: comments_url.into(),
            created_at: Avail::Yes((created_at).parse()?),
            draft: draft.into(),
            events_url: events_url.into(),
            html_url: html_url.into(),
            id: types::issue::IssueId::from(id),
            labels: Avail::Yes(labels.into_iter().map(|l| l.to_db_type()).collect()),
            labels_url: labels_url.into(),
            locked: Avail::from_option(locked),
            milestone_id: Avail::Yes(milestone_and_changes.as_ref().map(|(m, _)| m.id)),
            node_id: node_id.into(),
            number,
            performed_via_github_app_id: Avail::No,
            pull_request: Avail::Yes(pull_request.map(|p| p.try_to_db_type()).transpose()?),
            reactions: reactions.to_db_type().into(),
            repository_id,
            repository_url: repository_url.into(),
            state: Avail::from_option(state.map(|s| s.to_db_type())),
            state_reason: Avail::Yes(
                // TODO: This bridging between `String` and `StateReason` should be tested.
                state_reason
                    .map(|s| serde_json::from_str(&format!("\"{s}\"")))
                    .transpose()?,
            ),
            timeline_url: timeline_url.into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
            user_id: user.as_ref().map(|u| u.id).into(),
        };

        let mut changes = Changes::default();
        changes.add(assignee)?;
        changes.add(user)?;
        if let Some((milestone, changes_from_milestone)) = milestone_and_changes {
            changes.add(milestone)?;
            changes.add(changes_from_milestone)?;
        }

        Ok((issue, changes))
    }
}

impl ToDb for (types::repository::RepositoryId, IssuesDemilestonedIssue) {
    type DbType = types::issue::Issue;
    type OtherChanges = Changes;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, Changes), Self::Error> {
        let (repository_id, issues_assigned_issue) = self;
        let IssuesDemilestonedIssue {
            active_lock_reason,
            assignee,

            // TODO: THIS IS A MASSIVE TODO!! The updating of `assignees` is important for actual
            // usefulness of the app. But currently, this is untyped (most likely) because the
            // messed up JSON that we reverse engineer the types from (instead of using
            // `x-webhooks` in the OpenAPI Spec (Hey! I didn't know that existed at first! Don't
            // judge me!)) doesn't contain the types.
            assignees: _,
            author_association,
            body,
            closed_at,
            comments,
            comments_url,
            created_at,
            draft,
            events_url,
            html_url,
            id,
            labels,
            labels_url,
            locked,
            milestone,
            node_id,
            number,

            // This might not contain the ID (according to the types), in which case we can't
            // currently construct a GithubApp for it because we require ids on all the primary db
            // thingys. Plus, we don't care about `performed_via_github_app` too much for now (I
            // don't even know where it would even surface in the app yet).
            performed_via_github_app: _,

            pull_request,
            reactions,
            repository_url,
            state,
            state_reason,
            timeline_url,
            title,
            updated_at,
            url,
            user,
        } = issues_assigned_issue;
        let user = user.map(|u| u.to_db_type());
        let assignee = assignee.map(|a| a.to_db_type());
        let milestone_and_changes = milestone
            .map(|m| m.try_to_db_type_and_other_changes())
            .transpose()?;

        ignore_untyped::ignore_untyped(labels);

        let issue = types::issue::Issue {
            active_lock_reason: active_lock_reason.map(|a| a.to_db_type()).into(),
            assignee_id: assignee.as_ref().map(|a| a.id).into(),
            assignee_ids: Avail::No,
            author_association: author_association.to_db_type().into(),
            body: body.into(),
            body_html: Avail::No,
            body_text: Avail::No,
            closed_at: Avail::Yes(closed_at.map(|s| (s).parse()).transpose()?),
            closed_by_id: Avail::No,
            comments: comments.into(),
            comments_url: comments_url.into(),
            created_at: Avail::Yes((created_at).parse()?),
            draft: draft.into(),
            events_url: events_url.into(),
            html_url: html_url.into(),
            id: types::issue::IssueId::from(id),
            labels: Avail::No,
            labels_url: labels_url.into(),
            locked: Avail::from_option(locked),
            milestone_id: Avail::Yes(milestone_and_changes.as_ref().map(|(m, _)| m.id)),
            node_id: node_id.into(),
            number,
            performed_via_github_app_id: Avail::No,
            pull_request: Avail::Yes(pull_request.map(|p| p.try_to_db_type()).transpose()?),
            reactions: reactions.to_db_type().into(),
            repository_id,
            repository_url: repository_url.into(),
            state: Avail::from_option(state.map(|s| s.to_db_type())),
            state_reason: Avail::Yes(
                // TODO: This bridging between `String` and `StateReason` should be tested.
                state_reason
                    .map(|s| serde_json::from_str(&format!("\"{s}\"")))
                    .transpose()?,
            ),
            timeline_url: timeline_url.into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
            user_id: user.as_ref().map(|u| u.id).into(),
        };

        let mut changes = Changes::default();
        changes.add(assignee)?;
        changes.add(user)?;
        if let Some((milestone, changes_from_milestone)) = milestone_and_changes {
            changes.add(milestone)?;
            changes.add(changes_from_milestone)?;
        }

        Ok((issue, changes))
    }
}

impl ToDb for (types::repository::RepositoryId, IssuesMilestonedIssue) {
    type DbType = types::issue::Issue;
    type OtherChanges = Changes;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, Changes), Self::Error> {
        let (repository_id, issues_assigned_issue) = self;
        let IssuesMilestonedIssue {
            active_lock_reason,
            assignee,

            // TODO: THIS IS A MASSIVE TODO!! The updating of `assignees` is important for actual
            // usefulness of the app. But currently, this is untyped (most likely) because the
            // messed up JSON that we reverse engineer the types from (instead of using
            // `x-webhooks` in the OpenAPI Spec (Hey! I didn't know that existed at first! Don't
            // judge me!)) doesn't contain the types.
            assignees: _,
            author_association,
            body,
            closed_at,
            comments,
            comments_url,
            created_at,
            draft,
            events_url,
            html_url,
            id,
            labels,
            labels_url,
            locked,
            milestone,
            node_id,
            number,

            // This might not contain the ID (according to the types), in which case we can't
            // currently construct a GithubApp for it because we require ids on all the primary db
            // thingys. Plus, we don't care about `performed_via_github_app` too much for now (I
            // don't even know where it would even surface in the app yet).
            performed_via_github_app: _,

            pull_request,
            reactions,
            repository_url,
            state,
            state_reason,
            timeline_url,
            title,
            updated_at,
            url,
            user,
        } = issues_assigned_issue;
        let user = user.map(|u| u.to_db_type());
        let assignee = assignee.map(|a| a.to_db_type());
        let milestone_and_changes = milestone
            .map(|m| m.try_to_db_type_and_other_changes())
            .transpose()?;

        ignore_untyped::ignore_untyped(labels);

        let issue = types::issue::Issue {
            active_lock_reason: active_lock_reason.map(|a| a.to_db_type()).into(),
            assignee_id: assignee.as_ref().map(|a| a.id).into(),
            assignee_ids: Avail::No,
            author_association: author_association.to_db_type().into(),
            body: body.into(),
            body_html: Avail::No,
            body_text: Avail::No,
            closed_at: Avail::Yes(closed_at.map(|s| (s).parse()).transpose()?),
            closed_by_id: Avail::No,
            comments: comments.into(),
            comments_url: comments_url.into(),
            created_at: Avail::Yes((created_at).parse()?),
            draft: draft.into(),
            events_url: events_url.into(),
            html_url: html_url.into(),
            id: types::issue::IssueId::from(id),
            labels: Avail::No,
            labels_url: labels_url.into(),
            locked: Avail::from_option(locked),
            milestone_id: Avail::Yes(milestone_and_changes.as_ref().map(|(m, _)| m.id)),
            node_id: node_id.into(),
            number,
            performed_via_github_app_id: Avail::No,
            pull_request: Avail::Yes(pull_request.map(|p| p.try_to_db_type()).transpose()?),
            reactions: reactions.to_db_type().into(),
            repository_id,
            repository_url: repository_url.into(),
            state: Avail::from_option(state.map(|s| s.to_db_type())),
            state_reason: Avail::Yes(
                // TODO: This bridging between `String` and `StateReason` should be tested.
                state_reason
                    .map(|s| serde_json::from_str(&format!("\"{s}\"")))
                    .transpose()?,
            ),
            timeline_url: timeline_url.into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
            user_id: user.as_ref().map(|u| u.id).into(),
        };

        let mut changes = Changes::default();
        changes.add(assignee)?;
        changes.add(user)?;
        if let Some((milestone, changes_from_milestone)) = milestone_and_changes {
            changes.add(milestone)?;
            changes.add(changes_from_milestone)?;
        }

        Ok((issue, changes))
    }
}

impl ToDb for (types::repository::RepositoryId, IssuesLockedIssue) {
    type DbType = types::issue::Issue;
    type OtherChanges = Changes;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, Changes), Self::Error> {
        let (repository_id, issues_assigned_issue) = self;
        let IssuesLockedIssue {
            active_lock_reason,
            assignee,

            // TODO: THIS IS A MASSIVE TODO!! The updating of `assignees` is important for actual
            // usefulness of the app. But currently, this is untyped (most likely) because the
            // messed up JSON that we reverse engineer the types from (instead of using
            // `x-webhooks` in the OpenAPI Spec (Hey! I didn't know that existed at first! Don't
            // judge me!)) doesn't contain the types.
            assignees: _,
            author_association,
            body,
            closed_at,
            comments,
            comments_url,
            created_at,
            draft,
            events_url,
            html_url,
            id,
            labels,
            labels_url,
            locked,
            milestone,
            node_id,
            number,

            // This might not contain the ID (according to the types), in which case we can't
            // currently construct a GithubApp for it because we require ids on all the primary db
            // thingys. Plus, we don't care about `performed_via_github_app` too much for now (I
            // don't even know where it would even surface in the app yet).
            performed_via_github_app: _,

            pull_request,
            reactions,
            repository_url,
            state,
            state_reason,
            timeline_url,
            title,
            updated_at,
            url,
            user,
        } = issues_assigned_issue;
        let user = user.map(|u| u.to_db_type());
        let assignee = assignee.map(|a| a.to_db_type());
        let milestone_and_changes = milestone
            .map(|m| m.try_to_db_type_and_other_changes())
            .transpose()?;

        ignore_untyped::ignore_untyped(labels);

        let issue = types::issue::Issue {
            active_lock_reason: active_lock_reason.map(|a| a.to_db_type()).into(),
            assignee_id: assignee.as_ref().map(|a| a.id).into(),
            assignee_ids: Avail::No,
            author_association: author_association.to_db_type().into(),
            body: body.into(),
            body_html: Avail::No,
            body_text: Avail::No,
            closed_at: Avail::Yes(closed_at.map(|s| (s).parse()).transpose()?),
            closed_by_id: Avail::No,
            comments: comments.into(),
            comments_url: comments_url.into(),
            created_at: Avail::Yes((created_at).parse()?),
            draft: draft.into(),
            events_url: events_url.into(),
            html_url: html_url.into(),
            id: types::issue::IssueId::from(id),
            labels: Avail::No,
            labels_url: labels_url.into(),
            locked: locked.into(),
            milestone_id: Avail::Yes(milestone_and_changes.as_ref().map(|(m, _)| m.id)),
            node_id: node_id.into(),
            number,
            performed_via_github_app_id: Avail::No,
            pull_request: Avail::Yes(pull_request.map(|p| p.try_to_db_type()).transpose()?),
            reactions: reactions.to_db_type().into(),
            repository_id,
            repository_url: repository_url.into(),
            state: Avail::from_option(state.map(|s| s.to_db_type())),
            state_reason: Avail::Yes(
                // TODO: This bridging between `String` and `StateReason` should be tested.
                state_reason
                    .map(|s| serde_json::from_str(&format!("\"{s}\"")))
                    .transpose()?,
            ),
            timeline_url: timeline_url.into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
            user_id: user.as_ref().map(|u| u.id).into(),
        };

        let mut changes = Changes::default();
        changes.add(assignee)?;
        changes.add(user)?;
        if let Some((milestone, changes_from_milestone)) = milestone_and_changes {
            changes.add(milestone)?;
            changes.add(changes_from_milestone)?;
        }

        Ok((issue, changes))
    }
}

impl ToDb for (types::repository::RepositoryId, IssuesUnlockedIssue) {
    type DbType = types::issue::Issue;
    type OtherChanges = Changes;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, Changes), Self::Error> {
        let (repository_id, issues_assigned_issue) = self;
        let IssuesUnlockedIssue {
            active_lock_reason,
            assignee,

            // TODO: THIS IS A MASSIVE TODO!! The updating of `assignees` is important for actual
            // usefulness of the app. But currently, this is untyped (most likely) because the
            // messed up JSON that we reverse engineer the types from (instead of using
            // `x-webhooks` in the OpenAPI Spec (Hey! I didn't know that existed at first! Don't
            // judge me!)) doesn't contain the types.
            assignees: _,
            author_association,
            body,
            closed_at,
            comments,
            comments_url,
            created_at,
            draft,
            events_url,
            html_url,
            id,
            labels,
            labels_url,
            locked,
            milestone,
            node_id,
            number,

            // This might not contain the ID (according to the types), in which case we can't
            // currently construct a GithubApp for it because we require ids on all the primary db
            // thingys. Plus, we don't care about `performed_via_github_app` too much for now (I
            // don't even know where it would even surface in the app yet).
            performed_via_github_app: _,

            pull_request,
            reactions,
            repository_url,
            state,
            state_reason,
            timeline_url,
            title,
            updated_at,
            url,
            user,
        } = issues_assigned_issue;
        let user = user.map(|u| u.to_db_type());
        let assignee = assignee.map(|a| a.to_db_type());
        let milestone_and_changes = milestone
            .map(|m| m.try_to_db_type_and_other_changes())
            .transpose()?;

        ignore_untyped::ignore_untyped(labels);

        let issue = types::issue::Issue {
            active_lock_reason: active_lock_reason.map(|a| a.to_db_type()).into(),
            assignee_id: assignee.as_ref().map(|a| a.id).into(),
            assignee_ids: Avail::No,
            author_association: author_association.to_db_type().into(),
            body: body.into(),
            body_html: Avail::No,
            body_text: Avail::No,
            closed_at: Avail::Yes(closed_at.map(|s| (s).parse()).transpose()?),
            closed_by_id: Avail::No,
            comments: comments.into(),
            comments_url: comments_url.into(),
            created_at: Avail::Yes((created_at).parse()?),
            draft: draft.into(),
            events_url: events_url.into(),
            html_url: html_url.into(),
            id: types::issue::IssueId::from(id),
            labels: Avail::No,
            labels_url: labels_url.into(),
            locked: locked.into(),
            milestone_id: Avail::Yes(milestone_and_changes.as_ref().map(|(m, _)| m.id)),
            node_id: node_id.into(),
            number,
            performed_via_github_app_id: Avail::No,
            pull_request: Avail::Yes(pull_request.map(|p| p.try_to_db_type()).transpose()?),
            reactions: reactions.to_db_type().into(),
            repository_id,
            repository_url: repository_url.into(),
            state: Avail::from_option(state.map(|s| s.to_db_type())),
            state_reason: Avail::Yes(
                // TODO: This bridging between `String` and `StateReason` should be tested.
                state_reason
                    .map(|s| serde_json::from_str(&format!("\"{s}\"")))
                    .transpose()?,
            ),
            timeline_url: timeline_url.into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
            user_id: user.as_ref().map(|u| u.id).into(),
        };

        let mut changes = Changes::default();
        changes.add(assignee)?;
        changes.add(user)?;
        if let Some((milestone, changes_from_milestone)) = milestone_and_changes {
            changes.add(milestone)?;
            changes.add(changes_from_milestone)?;
        }

        Ok((issue, changes))
    }
}

impl ToDb for DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions {
    type OtherChanges = ();
    type DbType = github_api::models::app_1_permissions::ReadOrWrite;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        use github_api::models::app_1_permissions::ReadOrWrite::*;
        Ok((
            match self {
                DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions::Read => {
                    Read
                }
                DeploymentStatusCreatedDeploymentPerformedViaGithubAppPermissionsActions::Write => {
                    Write
                }
            },
            Default::default(),
        ))
    }
}

impl ToDb for MemberAddedChangesPermissionTo {
    type OtherChanges = ();
    type DbType = types::github_app::ReadOrWriteOrAdmin;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        use types::github_app::ReadOrWriteOrAdmin::*;
        Ok((
            match self {
                MemberAddedChangesPermissionTo::Read => Read,
                MemberAddedChangesPermissionTo::Write => Write,
                MemberAddedChangesPermissionTo::Admin => Admin,
            },
            Default::default(),
        ))
    }
}

impl ToDb for IssuesAssignedIssuePerformedViaGithubAppPermissions {
    type OtherChanges = ();
    type DbType = types::github_app::GitHubAppPermissions;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let IssuesAssignedIssuePerformedViaGithubAppPermissions {
            actions,
            administration,
            checks,
            content_references,
            contents,
            deployments,
            discussions,
            emails,
            environments,
            issues,
            keys,
            members,
            metadata,
            organization_administration,
            organization_hooks,
            organization_packages,
            organization_plan,
            organization_projects,
            organization_secrets,
            organization_self_hosted_runners,
            organization_user_blocking,
            packages,
            pages,
            pull_requests,
            repository_hooks,
            repository_projects,
            secret_scanning_alerts,
            secrets,
            security_events,
            security_scanning_alert,
            single_file,
            statuses,
            team_discussions,
            vulnerability_alerts,
            workflows,
        } = self;
        Ok((
            types::github_app::GitHubAppPermissions {
                actions: actions.map(|a| a.to_db_type()),
                administration: administration.map(|a| a.to_db_type()),
                checks: checks.map(|a| a.to_db_type()),
                content_references: content_references.map(|a| a.to_db_type()),
                contents: contents.map(|a| a.to_db_type()),
                deployments: deployments.map(|a| a.to_db_type()),
                discussions: discussions.map(|a| a.to_db_type()),
                emails: emails.map(|a| a.to_db_type()),
                environments: environments.map(|a| a.to_db_type()),
                issues: issues.map(|a| a.to_db_type()),
                keys: keys.map(|a| a.to_db_type()),
                members: members.map(|a| a.to_db_type()),
                metadata: metadata.map(|a| a.to_db_type()),
                organization_administration: organization_administration.map(|a| a.to_db_type()),
                organization_hooks: organization_hooks.map(|a| a.to_db_type()),
                organization_packages: organization_packages.map(|a| a.to_db_type()),
                organization_plan: organization_plan.map(|a| a.to_db_type()),
                organization_projects: organization_projects.map(|a| a.to_db_type()),
                organization_secrets: organization_secrets.map(|a| a.to_db_type()),
                organization_self_hosted_runners: organization_self_hosted_runners
                    .map(|a| a.to_db_type()),
                organization_user_blocking: organization_user_blocking.map(|a| a.to_db_type()),
                packages: packages.map(|a| a.to_db_type()),
                pages: pages.map(|a| a.to_db_type()),
                pull_requests: pull_requests.map(|a| a.to_db_type()),
                repository_hooks: repository_hooks.map(|a| a.to_db_type()),
                repository_projects: repository_projects.map(|a| a.to_db_type()),
                secret_scanning_alerts: secret_scanning_alerts.map(|a| a.to_db_type()),
                secrets: secrets.map(|a| a.to_db_type()),
                security_events: security_events.map(|a| a.to_db_type()),
                security_scanning_alert: security_scanning_alert.map(|a| a.to_db_type()),
                single_file: single_file.map(|a| a.to_db_type()),
                statuses: statuses.map(|a| a.to_db_type()),
                team_discussions: team_discussions.map(|a| a.to_db_type()),
                vulnerability_alerts: vulnerability_alerts.map(|a| a.to_db_type()),
                workflows: workflows.map(|a| a.to_db_type()),
            },
            Default::default(),
        ))
    }
}

impl ToDb for DeploymentProtectionRuleRequestedPullRequestsState {
    type OtherChanges = ();
    type DbType = github_api::models::milestone::OpenOrClosed;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        use github_api::models::milestone::OpenOrClosed::*;
        let state = match self {
            DeploymentProtectionRuleRequestedPullRequestsState::Closed => Closed,
            DeploymentProtectionRuleRequestedPullRequestsState::Open => Open,
        };
        Ok((state, Default::default()))
    }
}

impl ToDb for MilestoneClosedMilestone {
    type OtherChanges = Changes;
    type DbType = types::milestone::Milestone;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, Changes), Self::Error> {
        let MilestoneClosedMilestone {
            closed_at,
            closed_issues,
            created_at,
            creator,
            description,
            due_on,
            html_url,
            id,
            labels_url,
            node_id,
            number,
            open_issues,
            state,
            title,
            updated_at,
            url,
        } = self;
        let creator = creator.map(|c| c.to_db_type());

        let milestone = types::milestone::Milestone {
            closed_at: Avail::Yes(closed_at.map(|d| (d).parse()).transpose()?),
            closed_issues: closed_issues.into(),
            created_at: Avail::Yes((created_at).parse()?),
            creator_id: creator.as_ref().map(|c| c.id).into(),
            description: description.into(),
            due_on: Avail::Yes(due_on.map(|d| serde_json::from_str(&d)).transpose()?),
            html_url: html_url.into(),
            id: types::milestone::MilestoneId::from(id),
            labels_url: labels_url.into(),
            node_id: node_id.into(),
            number: number.into(),
            open_issues: open_issues.into(),
            state: state.to_db_type().into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
        };

        let mut changes = Changes::default();
        changes.add(creator)?;

        Ok((milestone, changes))
    }
}

impl ToDb for MilestoneCreatedMilestone {
    type OtherChanges = Changes;
    type DbType = types::milestone::Milestone;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(self) -> Result<(Self::DbType, Changes), Self::Error> {
        let MilestoneCreatedMilestone {
            closed_at,
            closed_issues,
            created_at,
            creator,
            description,
            due_on,
            html_url,
            id,
            labels_url,
            node_id,
            number,
            open_issues,
            state,
            title,
            updated_at,
            url,
        } = self;
        let creator = creator.map(|c| c.to_db_type());

        let milestone = types::milestone::Milestone {
            closed_at: Avail::Yes(closed_at.map(|d| (d).parse()).transpose()?),
            closed_issues: closed_issues.into(),
            created_at: Avail::Yes((created_at).parse()?),
            creator_id: creator.as_ref().map(|c| c.id).into(),
            description: description.into(),
            due_on: Avail::Yes(due_on.map(|d| serde_json::from_str(&d)).transpose()?),
            html_url: html_url.into(),
            id: types::milestone::MilestoneId::from(id),
            labels_url: labels_url.into(),
            node_id: node_id.into(),
            number: number.into(),
            open_issues: open_issues.into(),
            state: state.to_db_type().into(),
            title: title.into(),
            updated_at: Avail::Yes((updated_at).parse()?),
            url: url.into(),
        };

        let mut changes = Changes::default();
        changes.add(creator)?;

        Ok((milestone, changes))
    }
}

impl ToDb for Issues {
    type DbType = types::issue::Issue;

    type Error = ConversionError;

    type OtherChanges = Changes;

    fn try_to_db_type_and_other_changes(
        self,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        // Below, I don't modify the returned issue with `assignee` in `Issues::Assigned`, because
        // I assume that `issue.assignee` and `issue.assignees` will correctly reflect the most
        // recent state. But I really should verify this property holds. (TODO, VERY BIG TODO INDEED).
        // I make this assumption for at least one other top-level attribute (`label`) in one of
        // the casess below.
        Ok(match self {
            Issues::Assigned {
                assignee,
                enterprise,
                installation: _, // Not needed.
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);

                let (issue, mut other_changes) =
                    (types::repository::RepositoryId::from(repository.id), issue)
                        .try_to_db_type_and_other_changes()?;

                if let Some(assignee) = assignee {
                    let user = assignee.to_db_type();
                    other_changes.add(user)?;
                }

                (issue, other_changes)
            }
            Issues::Closed {
                enterprise,
                installation: _, // Not needed.
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);

                (types::repository::RepositoryId::from(repository.id), issue)
                    .try_to_db_type_and_other_changes()?
            }
            Issues::Deleted {
                enterprise,
                installation: _, // Not needed
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                (types::repository::RepositoryId::from(repository.id), issue)
                    .try_to_db_type_and_other_changes()?
            }
            Issues::Demilestoned {
                enterprise,
                installation: _, // Not needed
                issue,
                milestone,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                let (issue, mut other_changes) =
                    (types::repository::RepositoryId::from(repository.id), issue)
                        .try_to_db_type_and_other_changes()?;

                if let Some(milestone) = milestone {
                    let (milestone, changes_from_milestone) =
                        milestone.try_to_db_type_and_other_changes()?;

                    other_changes.add(milestone)?;
                    other_changes.add(changes_from_milestone)?;
                }

                (issue, other_changes)
            }
            Issues::Edited {
                changes: _, // Ignored cuz I hope `issue` will reflect changes.
                enterprise,
                installation: _, // Not needed
                issue,
                label,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                let (issue, mut other_changes) =
                    (types::repository::RepositoryId::from(repository.id), issue)
                        .try_to_db_type_and_other_changes()?;

                if let Some(label) = label {
                    other_changes.add(label.to_db_type())?;
                };

                (issue, other_changes)
            }
            Issues::Labeled {
                enterprise,
                installation: _, // Not needed
                issue,
                label,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                let (issue, mut other_changes) =
                    (types::repository::RepositoryId::from(repository.id), issue)
                        .try_to_db_type_and_other_changes()?;

                if let Some(label) = label {
                    other_changes.add(label.to_db_type())?;
                };

                (issue, other_changes)
            }
            Issues::Locked {
                enterprise,
                installation: _, // Not needed
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                (types::repository::RepositoryId::from(repository.id), issue)
                    .try_to_db_type_and_other_changes()?
            }
            Issues::Milestoned {
                enterprise,
                installation: _, // Not needed
                issue,
                milestone,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);

                let (issue, mut other_changes) =
                    (types::repository::RepositoryId::from(repository.id), issue)
                        .try_to_db_type_and_other_changes()?;

                let (milestone, changes_from_milestone) =
                    milestone.try_to_db_type_and_other_changes()?;

                other_changes.add(milestone)?;
                other_changes.add(changes_from_milestone)?;

                (issue, other_changes)
            }
            Issues::Opened {
                changes: _, // useless?
                enterprise,
                installation: _, // Not needed
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                (types::repository::RepositoryId::from(repository.id), issue)
                    .try_to_db_type_and_other_changes()?
            }
            Issues::Pinned {
                enterprise,
                installation: _, // Not needed
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                (types::repository::RepositoryId::from(repository.id), issue)
                    .try_to_db_type_and_other_changes()?
            }
            Issues::Reopened {
                enterprise,
                installation: _, // Not needed
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                (types::repository::RepositoryId::from(repository.id), issue)
                    .try_to_db_type_and_other_changes()?
            }
            Issues::Transferred {
                changes: _, // useless?
                enterprise,
                installation: _, // Not needed
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                (types::repository::RepositoryId::from(repository.id), issue)
                    .try_to_db_type_and_other_changes()?
            }
            Issues::Unassigned {
                assignee,
                enterprise,
                installation: _, // Not needed
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                let (issue, mut other_changes) =
                    (types::repository::RepositoryId::from(repository.id), issue)
                        .try_to_db_type_and_other_changes()?;

                if let Some(assignee) = assignee {
                    let user = assignee.to_db_type();
                    other_changes.add(user)?;
                }

                (issue, other_changes)
            }
            Issues::Unlabeled {
                enterprise,
                installation: _, // Not needed
                issue,
                label,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                let (issue, mut other_changes) =
                    (types::repository::RepositoryId::from(repository.id), issue)
                        .try_to_db_type_and_other_changes()?;

                if let Some(label) = label {
                    other_changes.add(label.to_db_type())?;
                };

                (issue, other_changes)
            }
            Issues::Unlocked {
                enterprise,
                installation: _, // Not needed
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                (types::repository::RepositoryId::from(repository.id), issue)
                    .try_to_db_type_and_other_changes()?
            }
            Issues::Unpinned {
                enterprise,
                installation: _, // Not needed
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped(enterprise);
                ignore_untyped(organization);
                ignore_untyped(sender);
                (types::repository::RepositoryId::from(repository.id), issue)
                    .try_to_db_type_and_other_changes()?
            }
        })
    }
}
