use std::convert::Infallible;

use github_webhook_body::*;
use jiff::Timestamp;

use crate::{
    avail::Avail,
    sync_engine::{
        changes::{AddChanges, Changes},
        conversions::{
            conversion_error::ConversionError, InfallibleToDbNoOtherChanges, ToDb,
            ToDbNoOtherChanges,
        },
    },
    types,
};

use super::ignore_untyped::ignore_untyped;

impl ToDb for IssueCommentCreatedIssueActiveLockReason {
    type Args = ();
    type OtherChanges = ();
    type DbType = github_api::models::issue::ActiveLockReason;

    type Error = Infallible;

    fn try_to_db_type_and_other_changes(
        self,
        _: Self::Args,
    ) -> Result<(Self::DbType, ()), Self::Error> {
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

impl ToDb for IssueCommentCreatedIssuePullRequest {
    type Args = ();
    type OtherChanges = ();
    type DbType = github_api::models::WebhooksIssuePullRequest;

    type Error = jiff::Error;

    fn try_to_db_type_and_other_changes(
        self,
        _: Self::Args,
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

impl ToDb for IssueCommentCreatedComment {
    type Args = (types::repository::RepositoryId, types::issue::IssueId);

    type DbType = types::issue_comment::IssueComment;

    type Error = ConversionError;

    type OtherChanges = Changes;

    fn try_to_db_type_and_other_changes(
        self,
        (repository_id, issue_id): Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let IssueCommentCreatedComment {
            author_association,
            body,
            created_at,
            html_url,
            id,
            issue_url,
            node_id,
            performed_via_github_app,
            reactions,
            updated_at,
            url,
            user,
        } = self;

        ignore_untyped(performed_via_github_app);

        let user = user.map(|u| u.to_db_type(()));

        let issue_comment = types::issue_comment::IssueComment {
            author_association: author_association.to_db_type(()).into(),
            body: body.into(),
            created_at: created_at.parse::<Timestamp>()?.into(),
            html_url: html_url.into(),
            id: id.into(),
            issue_url: issue_url.into(),
            node_id: node_id.into(),
            performed_via_github_app_id: Avail::No,
            reactions: reactions.to_db_type(()).into(),
            updated_at: updated_at.parse::<Timestamp>()?.into(),
            url: url.into(),
            user_id: user.as_ref().map(|u| u.id).into(),
            issue_id: Some(issue_id),
            repository_id,
        };

        let mut changes = Changes::default();
        changes.add(user)?;

        Ok((issue_comment, changes))
    }
}

impl ToDb for IssueCommentDeletedComment {
    type Args = (types::repository::RepositoryId, types::issue::IssueId);
    type DbType = types::issue_comment::IssueComment;

    type Error = ConversionError;

    type OtherChanges = Changes;

    fn try_to_db_type_and_other_changes(
        self,
        (repository_id, issue_id): Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let IssueCommentDeletedComment {
            author_association,
            body,
            created_at,
            html_url,
            id,
            issue_url,
            node_id,
            performed_via_github_app: _, // useless
            reactions,
            updated_at,
            url,
            user,
        } = self;

        let user = user.map(|u| u.to_db_type(()));

        let issue_comment = types::issue_comment::IssueComment {
            author_association: author_association.to_db_type(()).into(),
            body: body.into(),
            created_at: created_at.parse::<Timestamp>()?.into(),
            html_url: html_url.into(),
            id: id.into(),
            issue_url: issue_url.into(),
            node_id: node_id.into(),
            performed_via_github_app_id: Avail::No,
            reactions: reactions.to_db_type(()).into(),
            updated_at: updated_at.parse::<Timestamp>()?.into(),
            url: url.into(),
            user_id: user.as_ref().map(|u| u.id).into(),
            issue_id: Some(issue_id),
            repository_id,
        };

        let mut changes = Changes::default();
        changes.add(user)?;

        Ok((issue_comment, changes))
    }
}

impl ToDb for IssueCommentCreatedIssue {
    type Args = types::repository::RepositoryId;
    type DbType = types::issue::Issue;
    type OtherChanges = Changes;

    type Error = ConversionError;

    fn try_to_db_type_and_other_changes(
        self,
        repository_id: Self::Args,
    ) -> Result<(Self::DbType, Changes), Self::Error> {
        let IssueCommentCreatedIssue {
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
        } = self;
        let user = user.map(|u| u.to_db_type(()));
        let assignee = assignee.map(|a| a.to_db_type(()));
        let milestone_and_changes = milestone
            .map(|m| m.try_to_db_type_and_other_changes(()))
            .transpose()?;
        let issue = types::issue::Issue {
            active_lock_reason: active_lock_reason.map(|a| a.to_db_type(())).into(),
            assignee_id: assignee.as_ref().map(|a| a.id).into(),
            assignee_ids: Avail::No,
            author_association: author_association.to_db_type(()).into(),
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
            labels: Avail::Yes(labels.into_iter().map(|l| l.to_db_type(())).collect()),
            labels_url: labels_url.into(),
            locked: locked.into(),
            milestone_id: Avail::Yes(milestone_and_changes.as_ref().map(|(m, _)| m.id)),
            node_id: node_id.into(),
            number,
            performed_via_github_app_id: Avail::No,
            pull_request: Avail::Yes(pull_request.map(|p| p.try_to_db_type(())).transpose()?),
            reactions: reactions.to_db_type(()).into(),
            repository_id,
            repository_url: repository_url.into(),
            state: state.to_db_type(()).into(),
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

impl ToDb for IssueComment {
    type Args = ();
    type DbType = types::issue_comment::IssueComment;

    type Error = ConversionError;

    type OtherChanges = Changes;

    fn try_to_db_type_and_other_changes(
        self,

        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        Ok(match self {
            IssueComment::Created {
                comment,
                enterprise,
                installation: _, // Not needed.
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped((enterprise, organization, sender));
                let repository_id = repository.id.into();
                let (issue, other_changes_from_issue) =
                    issue.try_to_db_type_and_other_changes(repository_id)?;
                let (issue_comment, other_changes_from_issue_comment) =
                    comment.try_to_db_type_and_other_changes((repository_id, issue.id))?;

                let mut changes = Changes::default();
                changes.add(issue)?;
                changes.add(other_changes_from_issue)?;
                changes.add(other_changes_from_issue_comment)?;

                (issue_comment, changes)
            }
            IssueComment::Deleted {
                comment,
                enterprise,
                installation: _, // Not needed.
                issue,
                organization,
                repository,
                sender,
            } => {
                ignore_untyped((enterprise, organization, sender));
                let repository_id = repository.id.into();
                let (issue, other_changes_from_issue) =
                    issue.try_to_db_type_and_other_changes(repository_id)?;
                let (issue_comment, other_changes_from_issue_comment) =
                    comment.try_to_db_type_and_other_changes((repository_id, issue.id))?;

                let mut changes = Changes::default();
                changes.add(issue)?;
                changes.add(other_changes_from_issue)?;
                changes.add(other_changes_from_issue_comment)?;

                (issue_comment, changes)
            }
            IssueComment::Edited {
                changes: _, // Not needd / not really useful.
                comment,
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
                let repository_id = repository.id.into();
                let (issue, other_changes_from_issue) =
                    issue.try_to_db_type_and_other_changes(repository_id)?;
                let (issue_comment, other_changes_from_issue_comment) =
                    comment.try_to_db_type_and_other_changes((repository_id, issue.id))?;

                let mut changes = Changes::default();
                changes.add(issue)?;
                changes.add(other_changes_from_issue)?;
                changes.add(other_changes_from_issue_comment)?;

                (issue_comment, changes)
            }
        })
    }
}
