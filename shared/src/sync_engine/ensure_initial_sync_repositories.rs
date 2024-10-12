use crate::types::{
    installation::InstallationId, issue_initial_sync_status::InitialSyncStatusEnum,
    repository_initial_sync_status::RepositoryInitialSyncStatus,
};

use super::{
    changes::Changes, conversions::from_repository::from_repository, SyncEngine, SyncResult,
    MAX_PER_PAGE,
};

impl SyncEngine {
    pub async fn ensure_initial_sync_repositories(&self, id: &InstallationId) -> SyncResult<()> {
        let txn = self
            .db
            .txn()
            .with_store::<RepositoryInitialSyncStatus>()
            .ro();

        if let Some(RepositoryInitialSyncStatus {
            status: InitialSyncStatusEnum::Full,
            id: _,
        }) = txn
            .object_store::<RepositoryInitialSyncStatus>()?
            .get(id)
            .await?
        {
            return Ok(());
        }

        let conf = self.get_api_conf(id).await?;

        let mut repos = vec![];
        let mut page = 1;
        loop {
            let repos_in_page =
                github_api::apis::apps_api::apps_slash_list_repos_accessible_to_installation(
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

        let changes = repos
            .into_iter()
            .map(|r| from_repository(r, *id).map(|r| r.1))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .try_fold(Changes::default(), |acc, new| acc.with_added(new))?;

        let txn = Changes::txn(&self.db)
            .with_store::<RepositoryInitialSyncStatus>()
            .rw();
        self.merge_and_upsert_changes(&txn, changes).await?;
        txn.object_store::<RepositoryInitialSyncStatus>()?
            .put(&RepositoryInitialSyncStatus {
                status: InitialSyncStatusEnum::Full,
                id: *id,
            })
            .await?;
        Ok(())
    }
}
