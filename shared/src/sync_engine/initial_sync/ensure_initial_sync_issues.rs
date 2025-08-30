use futures::future::try_join_all;
use typed_db::RawDbTrait;

use super::super::{
    changes::{AddChanges, Changes},
    conversions::ToDb,
    error::{SyncErrorSrc, SyncResult},
    SyncEngine, MAX_PER_PAGE,
};
use crate::sync_engine::websocket_updates::transport::TransportTrait;
use crate::{
    avail::MergeError,
    github_api_trait::GithubApiTrait,
    types::{
        installation::InstallationId,
        issue::{Issue, RepositoryIdIndex},
        issues_initial_sync_status::{InitialSyncStatusEnum, IssuesInitialSyncStatus},
        repository::{Repository, RepositoryId},
        user::User,
    },
};
use crate::{backend_api_trait::BackendApiTrait, sync_engine::error::RawDbErrorToSyncError};

impl<
        RawDb: RawDbTrait,
        BackendApi: BackendApiTrait,
        Transport: TransportTrait,
        GithubApi: GithubApiTrait,
    > SyncEngine<RawDb, BackendApi, Transport, GithubApi>
{
    pub async fn ensure_initial_sync_issues(
        &self,
        id: &RepositoryId,
        installation_id: &InstallationId,
    ) -> SyncResult<(), Transport, RawDb> {
        let mut page = 1;
        let txn = self
            .db
            .txn()
            .with_table::<IssuesInitialSyncStatus>()
            .with_table::<Issue>()
            .build()
            .tse()?;
        let initial_sync_status = txn
            .table::<IssuesInitialSyncStatus>()
            .tse()?
            .get(id)
            .await
            .tse()?;
        if let Some(initial_sync_status) = initial_sync_status {
            match initial_sync_status.status {
                InitialSyncStatusEnum::Full => return Ok(()),
                InitialSyncStatusEnum::Partial => {
                    page = (txn
                        .table::<Issue>()
                        .tse()?
                        .index::<RepositoryIdIndex>()
                        .tse()?
                        .get_all_optimistically(Some(id))
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
            .tse()?;
        let repo = txn
            .table::<Repository>()
            .tse()?
            .get(id)
            .await?
            .ok_or_else(|| {
                SyncErrorSrc::DataModel(format!("repository with id {id:?}: doesn't exist"))
            })?;
        let repo_owner_id = repo.owner_id.ok_or_else(|| {
            SyncErrorSrc::DataModel(format!(
                "repository with id {id:?} has no owner id available"
            ))
        })?;
        let repo_owner = txn
            .object_store::<User>()?
            .get(&repo_owner_id)
            .await?
            .ok_or_else(|| {
                SyncErrorSrc::DataModel(format!("user with id {repo_owner_id:?}: doesn't exist"))
            })?;
        let owner_name = repo_owner.login;
        drop(txn);

        let repo_name = repo.name;

        // NOTE: Maybe abstract away dealing with pagination.
        loop {
            let issues = self
                .github_api
                .issues_slash_list_for_repo(
                    &conf,
                    &owner_name,
                    &repo_name,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    "created".into(),
                    "asc".into(),
                    None,
                    MAX_PER_PAGE.into(),
                    page.into(),
                )
                .await?;
            let last_fetched_num = issues.len();
            let changes = Changes::try_try_from_iter(
                try_join_all(
                    issues
                        .into_iter()
                        .map(|r| r.try_to_db_type_and_other_changes(*id)),
                )
                .await?
                .into_iter()
                .map(|(issue, mut other_changes)| {
                    other_changes.add(issue)?;
                    Ok::<_, MergeError>(other_changes)
                }),
            )??;

            let txn = Changes::txn(&self.db)
                .with_table::<IssuesInitialSyncStatus>()
                .read_write()
                .build()
                .tse()?;
            self.persist_changes(&txn, changes).await?;
            txn.table::<IssuesInitialSyncStatus>()
                .tse()?
                .put(&IssuesInitialSyncStatus {
                    status: InitialSyncStatusEnum::Partial,
                    id: *id,
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
            .with_table::<IssuesInitialSyncStatus>()
            .read_write()
            .build()
            .tse()?;
        txn.table::<IssuesInitialSyncStatus>()
            .tse()?
            .put(&IssuesInitialSyncStatus {
                status: InitialSyncStatusEnum::Full,
                id: *id,
            })
            .await
            .tse()?;
        Ok(())
    }
}
