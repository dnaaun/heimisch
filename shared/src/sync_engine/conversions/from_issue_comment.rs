use std::future::Future;

use crate::{
    avail::Avail,
    sync_engine::changes::{AddChanges, Changes},
    types::{issue::IssueId, issue_comment::IssueCommentId, repository::RepositoryId},
};

use super::{
    conversion_error::ConversionError, from_integration::from_nullable_integration,
    InfallibleToDbNoOtherChanges,
};

pub async fn from_issue_comment<Fut: Future<Output = Option<IssueId>>>(
    issue_id_from_number: impl FnOnce(i64) -> Fut,
    api_issue_comment: github_api::models::IssueComment,
    repository_id: &RepositoryId,
) -> Result<(IssueCommentId, Changes), ConversionError> {
    let github_api::models::IssueComment {
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
    } = api_issue_comment;

    let db_user = user.map(|u| u.to_db_type(()));
    let db_github_app_info = performed_via_github_app
        .map(|p| from_nullable_integration(*p))
        .transpose()?;
    let (db_github_app_id, changes_from_github_app) = match db_github_app_info {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    };

    let issue_number: Option<i64> = issue_url
        .split('/')
        .next_back()
        .and_then(|i| i.parse().ok());

    let issue_id = match issue_number {
        Some(issue_number) => issue_id_from_number(issue_number).await,
        None => None,
    };

    let db_issue_comment = crate::types::issue_comment::IssueComment {
        author_association: author_association.into(),
        body: body.into(),
        created_at: Avail::Yes(created_at.parse()?),
        html_url: html_url.into(),
        id: id.into(),
        issue_url: issue_url.into(),
        node_id: node_id.into(),
        performed_via_github_app_id: db_github_app_id.into(),
        reactions: (*reactions).into(),
        updated_at: Avail::Yes(updated_at.parse()?),
        url: url.into(),
        user_id: db_user.as_ref().map(|u| u.id).into(),
        issue_id,
        repository_id: *repository_id,
    };

    let issue_comment_id = db_issue_comment.id;

    let mut changes = Changes::default();
    changes.add(db_issue_comment)?;
    changes.add(db_user)?;
    changes.add(changes_from_github_app)?;

    Ok((issue_comment_id, changes))
}
