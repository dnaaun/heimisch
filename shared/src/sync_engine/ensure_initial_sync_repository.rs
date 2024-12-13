use crate::types::{
    repository::Repository,
    repository_initial_sync_status::{RepoSyncStatus, RepositoryInitialSyncStatus},
};

use super::{error::SyncResult, SyncEngine};

impl SyncEngine {
    /// `force` means we ignore the RepositoryInitialSyncStatus.
    pub async fn ensure_initial_sync_repository(
        &self,
        repo: &Repository,
        force: bool,
    ) -> SyncResult<()> {
        if !force {
            let txn = self
                .db
                .txn()
                .with_store::<RepositoryInitialSyncStatus>()
                .ro();
            let store = txn.object_store::<RepositoryInitialSyncStatus>()?;
            if let Some(RepoSyncStatus::Full) = store.get(&repo.id).await?.map(|r| r.status) {
                return Ok(());
            }
        }
        self.ensure_initial_sync_issues(&repo.id, &repo.installation_id)
            .await?;
        self.ensure_initial_sync_issue_comments(&repo.id, &repo.installation_id)
            .await?;
        let txn = self
            .db
            .txn()
            .with_store::<RepositoryInitialSyncStatus>()
            .rw();
        txn.object_store::<RepositoryInitialSyncStatus>()?
            .put(&RepositoryInitialSyncStatus {
                status: RepoSyncStatus::Full,
                id: repo.id,
            })
            .await?;

        Ok(())
    }
}
