use crate::{
    avail::{Avail, MergeError},
    sync_engine::changes::{AddChanges, Changes},
    types::{issue::IssueId, repository::RepositoryId},
};
use typesafe_idb::Store;

use super::{from_milestone1::from_milestone1, InfallibleToDbNoOtherChanges, ToDb};

impl ToDb for github_api::models::Label {
    type Args = ();
    type OtherChanges = ();
    type DbType = crate::types::label::Label;

    type Error = std::convert::Infallible;

    fn try_to_db_type_and_other_changes(
        self,
        _: Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
        let github_api::models::Label {
            color,
            default,
            description,
            id,
            name,
            node_id,
            url,
        } = self;
        let id = crate::types::label::LabelId::from(id);
        let label = crate::types::label::Label {
            color,
            default,
            description,
            id,
            name,
            node_id,
            url,
        };

        Ok((label, Default::default()))
    }
}

pub fn from_issue(
    api_issue: github_api::models::Issue,
    repository_id: &RepositoryId,
) -> Result<(IssueId, Changes), MergeError> {
    let github_api::models::Issue {
        active_lock_reason,
        assignee,
        assignees,
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
        performed_via_github_app,
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
    } = api_issue;

    let db_assignee = assignee.map(|a| a.map(|a| a.to_db_type(())));
    let db_assignees = assignees
        .into_iter()
        .map(|x| x.to_db_type(()))
        .collect::<Vec<_>>();
    let db_user = user.map(|u| u.to_db_type(()));
    let db_milestone_info = milestone.map(|m| from_milestone1(*m)).transpose()?;
    let db_github_app_info =
        performed_via_github_app.map(|p| p.map(|p| p.try_to_db_type_and_other_changes(())));
    let (db_github_app, changes_from_github_app) = match db_github_app_info {
        Some(db_github_app_info) => {
            if let Some(db_github_app_info) = db_github_app_info {
                let db_github_app_info = db_github_app_info?;
                (Some(db_github_app_info.0), Some(db_github_app_info.1))
            } else {
                (None, None)
            }
        }
        None => (None, None),
    };
    let db_github_app_id = db_github_app.as_ref().map(|d| d.as_ref().map(|d| *d.id()));

    let db_issue = crate::types::issue::Issue {
        active_lock_reason: active_lock_reason.into(),
        assignee_id: Avail::from_option(db_assignee.as_ref().map(|i| i.as_ref().map(|i| i.id))),
        assignee_ids: db_assignees.iter().map(|a| a.id).collect::<Vec<_>>().into(),
        author_association: author_association.into(),
        body: body.into(),
        body_html: Avail::No,
        body_text: Avail::No,
        closed_at: closed_at.into(),
        closed_by_id: Avail::No,
        comments: i64::from(comments).into(),
        comments_url: comments_url.into(),
        created_at: created_at.into(),
        draft: draft.into(),
        events_url: events_url.into(),
        html_url: html_url.into(),
        id: id.into(),
        labels: Avail::Yes(
            labels
                .into_iter()
                .flatten()
                .map(|l| l.to_db_type(()))
                .collect(),
        ),
        labels_url: labels_url.into(),
        locked: Avail::from_option(locked),
        milestone_id: db_milestone_info.as_ref().map(|(m, _)| *m).into(),
        node_id: node_id.into(),
        number: i64::from(number),
        performed_via_github_app_id: Avail::from_option(db_github_app_id),
        pull_request: pull_request.map(|i| *i).into(),
        reactions: (*reactions).into(),
        repository_id: *repository_id,
        repository_url: repository_url.into(),
        state: Avail::from_option(state),
        state_reason: Avail::from_option(state_reason),
        timeline_url: timeline_url.into(),
        title: title.into(),
        updated_at: updated_at.into(),
        url: url.into(),
        user_id: db_user.as_ref().map(|u| u.id).into(),
    };

    let issue_id = db_issue.id;

    let mut changes = Changes::default();
    changes.add(db_issue)?;
    changes.add(db_assignee)?;
    changes.add(db_user)?;
    changes.add(db_assignees)?;
    if let Some((_, inner_changes)) = db_milestone_info {
        changes.add(inner_changes)?;
    }
    changes.add(db_github_app)?;
    changes.add(changes_from_github_app)?;

    Ok((issue_id, changes))
}
