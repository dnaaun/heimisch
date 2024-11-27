use crate::types::repository::Repository;

use super::{error::SyncResult, SyncEngine};

impl SyncEngine {
    pub async fn ensure_initial_sync_one_repository(&self, repo: &Repository) -> SyncResult<()> {
        self.ensure_initial_sync_issues(&repo.id, &repo.installation_id)
            .await?;
        self.ensure_initial_sync_issue_comments(&repo.id, &repo.installation_id)
            .await?;
        Ok(())
    }
}
