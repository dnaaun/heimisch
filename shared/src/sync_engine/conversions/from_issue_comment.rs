use std::future::Future;

use crate::{
    avail::MergeError,
    sync_engine::changes::{AddChanges, Changes},
    types::{issue::IssueId, issue_comment::IssueCommentId, repository::RepositoryId},
};

use super::{from_integration::from_nullable_integration, from_user1::from_user1};

pub async fn from_issue_comment<Fut: Future<Output = Option<IssueId>>>(
    issue_id_from_number: impl FnOnce(i64) -> Fut,
    api_issue_comment: github_api::models::IssueComment,
    repository_id: &RepositoryId,
) -> Result<(IssueCommentId, Changes), MergeError> {
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

    let db_user = user.map(|u| from_user1(*u));
    let db_github_app_info = performed_via_github_app
        .map(|p| from_nullable_integration(*p))
        .transpose()?;
    let (db_github_app_id, changes_from_github_app) = match db_github_app_info {
        Some((a, b)) => (Some(a), Some(b)),
        None => (None, None),
    };

    let issue_number: Option<i64> = issue_url
        .split('/')
        .last()
        .map(|i| i.parse().ok())
        .flatten();

    let issue_id = match issue_number {
        Some(issue_number) => issue_id_from_number(issue_number).await,
        None => None,
    };

    let db_issue_comment = crate::types::issue_comment::IssueComment {
        author_association: author_association.into(),
        body: body.into(),
        created_at: created_at.into(),
        html_url: html_url.into(),
        id: id.into(),
        issue_url: issue_url.into(),
        node_id: node_id.into(),
        performed_via_github_app_id: db_github_app_id.into(),
        reactions: (*reactions).into(),
        updated_at: updated_at.into(),
        url: url.into(),
        user_id: db_user.as_ref().map(|u| u.id.clone()).into(),
        issue_id,
        repository_id: repository_id.clone().into(),
    };

    let issue_comment_id = db_issue_comment.id;

    let mut changes = Changes::default();
    changes.add(db_issue_comment)?;
    changes.add(db_user)?;
    changes.add(changes_from_github_app)?;

    Ok((issue_comment_id, changes))
}
