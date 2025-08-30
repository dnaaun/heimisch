use std::sync::Arc;

use crate::avail::Avail;
use crate::sync_engine::conversions::conversion_error::ConversionError;
use crate::sync_engine::{
    changes::{AddChanges, Changes},
    conversions::{InfallibleToDbNoOtherChanges, ToDb},
};
use crate::types::issue::IssueId;
use futures::future::{LocalBoxFuture, OptionFuture};
use typed_db::Table;

impl ToDb for github_api::models::IssueComment {
    type DbType = crate::types::issue_comment::IssueComment;

    type Error = ConversionError;

    type OtherChanges = Changes;

    type Args = (
        Arc<dyn Fn(i64) -> LocalBoxFuture<'static, Option<IssueId>>>,
        crate::types::repository::RepositoryId,
    );

    async fn try_to_db_type_and_other_changes(
        self,
        (issue_id_from_number, repository_id): Self::Args,
    ) -> Result<(Self::DbType, Self::OtherChanges), Self::Error> {
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
        } = self;

        let db_user = OptionFuture::from(user.map(|u| u.to_db_type(()))).await;
        let db_github_app_info = OptionFuture::from(
            performed_via_github_app.map(|p| p.try_to_db_type_and_other_changes(())),
        )
        .await
        .transpose()?;
        let (db_github_app, changes_from_github_app) = match db_github_app_info {
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
            performed_via_github_app_id: db_github_app.as_ref().map(|d| *d.id()).into(),
            reactions: (*reactions).into(),
            updated_at: Avail::Yes(updated_at.parse()?),
            url: url.into(),
            user_id: db_user.as_ref().map(|u| u.id).into(),
            issue_id,
            repository_id,
        };

        let mut changes = Changes::default();
        changes.add(db_user)?;
        changes.add(db_github_app)?;
        changes.add(changes_from_github_app)?;

        Ok((db_issue_comment, changes))
    }
}
