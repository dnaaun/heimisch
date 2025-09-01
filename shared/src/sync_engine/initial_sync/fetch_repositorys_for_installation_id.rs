use futures::future::try_join_all;
use typed_db::RawDbTrait;

use crate::{
    avail::MergeError,
    backend_api_trait::BackendApiTrait,
    github_api_trait::GithubApiTrait,
    sync_engine::{error::RawDbErrorToSyncError, websocket_updates::transport::TransportTrait},
    types::{
        installation::InstallationId,
        installation_initial_sync_status::InstallationInitialSyncStatus,
        issues_initial_sync_status::InitialSyncStatusEnum,
    },
};

use super::super::{
    changes::{AddChanges, Changes},
    conversions::ToDb,
    SyncEngine, SyncResult, MAX_PER_PAGE,
};

impl<
        RawDb: RawDbTrait,
        BackendApi: BackendApiTrait,
        Transport: TransportTrait,
        GithubApi: GithubApiTrait,
    > SyncEngine<RawDb, BackendApi, Transport, GithubApi>
{
    pub async fn fetch_repositorys_for_installation_id(
        &self,
        id: &InstallationId,
    ) -> SyncResult<(), Transport, RawDb> {
        let txn = self
            .db
            .txn()
            .with_table::<InstallationInitialSyncStatus>()
            .build();
        let initial_sync_status = txn.get::<InstallationInitialSyncStatus>(id).await.tse()?;
        if let Some(initial_sync_status) = initial_sync_status {
            if let InitialSyncStatusEnum::Full = initial_sync_status.status {
                return Ok(());
            }
        }
        drop(txn);

        let conf = self.get_api_conf(id).await?;

        // No need to mark as Partial since we only have Full and NoSync states now
        let mut repos = vec![];
        let mut page = 1;
        loop {
            let repos_in_page = self
                .github_api
                .apps_slash_list_repos_accessible_to_installation(
                    &conf,
                    Some(MAX_PER_PAGE),
                    Some(page),
                )
                .await?
                .repositories;
            let last_fetched_num = repos_in_page.len();
            repos.extend(repos_in_page);
            page += 1;
            if last_fetched_num < MAX_PER_PAGE as usize {
                break;
            }
        }

        let changes = try_join_all(
            repos
                .into_iter()
                .map(|r| r.try_to_db_type_and_other_changes(*id)),
        )
        .await?
        .into_iter()
        .try_fold(Changes::default(), |mut acc, (repo, other_changes)| {
            acc.add(repo)?;
            acc.add(other_changes)?;
            Ok::<_, MergeError>(acc)
        })?;

        let txn = Changes::txn(&self.db).read_write().build();
        self.persist_changes(&txn, changes).await?;

        // Mark sync as complete
        let txn = self
            .db
            .txn()
            .with_table::<InstallationInitialSyncStatus>()
            .read_write()
            .build();
        txn.table::<InstallationInitialSyncStatus>()
            .put(&InstallationInitialSyncStatus {
                status: InitialSyncStatusEnum::Full,
                id: *id,
            })
            .await
            .tse()?;

        Ok(())
    }
}
