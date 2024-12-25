use crate::types::{
    repository::{Repository, RepositoryId},
    repository_initial_sync_status::{RepoSyncStatus, RepositoryInitialSyncStatus},
};

use super::{error::SyncResult, typed_transport, SyncEngine};

impl<T: typed_transport::TypedTransportTrait> SyncEngine<T> {
    /// `force_initial_sync` means we ignore the RepositoryInitialSyncStatus. This will come into
    /// play when we implement the "if the last time we were in touch is less than 7 days, do a
    /// full resync."
    pub async fn ensure_initial_sync_repository(
        &self,
        id: &RepositoryId,
        force_initial_sync: bool,
    ) -> SyncResult<(), T> {
        if !force_initial_sync {
            let txn = self
                .db
                .txn()
                .with_store::<RepositoryInitialSyncStatus>()
                .build();
            let store = txn.object_store::<RepositoryInitialSyncStatus>()?;
            if let Some(RepoSyncStatus::Full) = store.get(&id).await?.map(|r| r.status) {
                return Ok(());
            }
        }
        let repo = self
            .db
            .txn()
            .with_store::<Repository>()
            .build()
            .object_store::<Repository>()?
            .get(&id)
            .await?
            .expect("expected repo to be present if an id is passed");

        self.ensure_initial_sync_issues(&id, &repo.installation_id)
            .await?;
        self.ensure_initial_sync_issue_comments(&id, &repo.installation_id)
            .await?;
        let txn = self
            .db
            .txn()
            .with_store::<RepositoryInitialSyncStatus>()
            .read_write()
            .build();
        txn.object_store::<RepositoryInitialSyncStatus>()?
            .put(&RepositoryInitialSyncStatus {
                status: RepoSyncStatus::Full,
                id: *id,
            })
            .await?;

        Ok(())
    }
}
