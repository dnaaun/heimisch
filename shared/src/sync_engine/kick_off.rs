use crate::types::{
    installation::InstallationId,
    repository::{InstallationIdIndex, Repository},
};

use super::{SyncEngine, SyncResult};

impl SyncEngine {
    pub async fn kick_off(&self, id: &InstallationId) -> SyncResult<()> {
        self.fetch_repositorys_for_installation_id(id).await.unwrap();

        let txn = self.db.txn().with_store::<Repository>().rw();
        let repos = txn
            .object_store::<Repository>()?
            .index::<InstallationIdIndex>()?
            .get_all(Some(id))
            .await?;

        let test_repo = repos
            .into_iter()
            .find(|r| r.name == "test_for_heimisch")
            .unwrap();

        self.ensure_initial_sync_one_repository(&test_repo).await?;

        Ok(())
    }
}
