use std::sync::Arc;

use futures::future::{join_all, LocalBoxFuture};
use typed_db::RawDbTrait;

use super::super::{
    changes::Changes,
    conversions::ToDb,
    error::{SyncErrorSrc, SyncResult},
    SyncEngine, MAX_PER_PAGE,
};
use crate::{
    backend_api_trait::BackendApiTrait,
    github_api_trait::GithubApiTrait,
    sync_engine::{error::RawDbErrorToSyncError, websocket_updates::transport::TransportTrait},
    types::{
        installation::InstallationId,
        issue::{Issue, IssueId, NumberIndex},
        issue_comment::{IssueComment, RepositoryIdIndex},
        issue_comment_initial_sync_status::IssueCommentsInitialSyncStatus,
        issues_initial_sync_status::InitialSyncStatusEnum,
        repository::{Repository, RepositoryId},
        user::User,
    },
};

impl<
        RawDb: RawDbTrait,
        BackendApi: BackendApiTrait,
        Transport: TransportTrait,
        GithubApi: GithubApiTrait,
    > SyncEngine<RawDb, BackendApi, Transport, GithubApi>
where
    RawDb: 'static,
{
    /// This function will try to find issue ids in the db by using the issue number in `issue_url`
    /// of issue_comment`.
    pub async fn ensure_initial_sync_issue_comments(
        &self,
        id: RepositoryId,
        installation_id: &InstallationId,
    ) -> SyncResult<(), Transport, RawDb> {
        let mut page = 1;
        let txn = self
            .db
            .txn()
            .with_table::<IssueCommentsInitialSyncStatus>()
            .with_table::<IssueComment>()
            .build()
            .await;
        let initial_sync_status = txn
            .table::<IssueCommentsInitialSyncStatus>()
            .get(&id)
            .await
            .tse()?;
        if let Some(initial_sync_status) = initial_sync_status {
            match initial_sync_status.status {
                InitialSyncStatusEnum::Full => return Ok(()),
                InitialSyncStatusEnum::Partial => {
                    page = (txn
                        .table::<IssueComment>()
                        .index::<RepositoryIdIndex>()
                        .get_all_optimistically(Some(&id))
                        .await
                        .tse()?
                        .len() as f64
                        / f64::from(MAX_PER_PAGE))
                    .ceil() as i32;
                }
                InitialSyncStatusEnum::NoSync => (),
            }
        }
        drop(txn);
        let conf = self.get_api_conf(installation_id).await?;

        let txn = self
            .db
            .txn()
            .with_table::<Repository>()
            .with_table::<User>()
            .build()
            .await;
        let repo = txn
            .table::<Repository>()
            .get(&id)
            .await
            .tse()?
            .ok_or_else(|| {
                SyncErrorSrc::DataModel(format!("repository with id {id:?}: doesn't exist"))
            })?;
        let repo_owner_id = repo.owner_id.ok_or_else(|| {
            SyncErrorSrc::DataModel(format!(
                "repository with id {id:?} has no owner id available"
            ))
        })?;
        let repo_owner = txn
            .table::<User>()
            .get(&repo_owner_id)
            .await
            .tse()?
            .ok_or_else(|| {
                SyncErrorSrc::DataModel(format!("user with id {repo_owner_id:?}: doesn't exist"))
            })?;
        let owner_name = repo_owner.login;
        drop(txn);
        let repo_name = repo.name;

        // NOTE: Maybe abstract away dealing with pagination.
        loop {
            let issue_comments = self
                .github_api
                .issues_slash_list_comments_for_repo(
                    &conf,
                    &owner_name,
                    &repo_name,
                    "created".into(),
                    "asc".into(),
                    None,
                    MAX_PER_PAGE.into(),
                    page.into(),
                )
                .await?;
            let last_fetched_num = issue_comments.len();

            let db = self.db.clone();
            let issue_id_from_number = Arc::new(move |number| {
                let db = db.clone();
                Box::pin(async move {
                    let txn = db.clone().txn().with_table::<Issue>().build().await;
                    txn.table::<Issue>()
                        .index::<NumberIndex>()
                        .get_all_optimistically(Some(&number))
                        .await
                        .unwrap()
                        .into_iter()
                        .find(|issue| issue.repository_id == id)
                        .map(|i| i.id)
                }) as LocalBoxFuture<'static, Option<IssueId>>
            })
                as Arc<dyn Fn(i64) -> LocalBoxFuture<'static, Option<IssueId>> + 'static>;
            let issue_comment_ids_and_changes =
                join_all(issue_comments.into_iter().map(|r| {
                    r.try_to_db_type_and_other_changes((issue_id_from_number.clone(), id))
                }))
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
            let changes =
                Changes::try_from_iter(issue_comment_ids_and_changes.into_iter().map(|r| r.1))?;

            let txn = Changes::txn(&self.db)
                .with_table::<IssueCommentsInitialSyncStatus>()
                .read_write()
                .build()
                .await;
            self.persist_changes(&txn, changes).await?;
            txn.table::<IssueCommentsInitialSyncStatus>()
                .put(&IssueCommentsInitialSyncStatus {
                    status: InitialSyncStatusEnum::Partial,
                    id,
                })
                .await
                .tse()?;

            page += 1;
            if last_fetched_num < MAX_PER_PAGE as usize {
                break;
            }
        }

        let txn = self
            .db
            .txn()
            .with_table::<IssueCommentsInitialSyncStatus>()
            .read_write()
            .build()
            .await;
        txn.table::<IssueCommentsInitialSyncStatus>()
            .put(&IssueCommentsInitialSyncStatus {
                status: InitialSyncStatusEnum::Full,
                id,
            })
            .await
            .tse()?;
        Ok(())
    }
}
