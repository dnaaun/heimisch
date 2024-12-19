use serde::{Deserialize, Serialize};

use super::{issues_initial_sync_status::InitialSyncStatusEnum, repository::RepositoryId};

/// Serde internal tagging is necessary if we're going to index on `id` in IndexedDb.
#[derive(macros::TypesafeIdb, Debug, Serialize, Deserialize, Clone, Default)]
#[serde(tag = "type")]
pub struct IssueCommentsInitialSyncStatus {
    pub status: InitialSyncStatusEnum,
    #[idb(id)]
    pub id: RepositoryId,
}
