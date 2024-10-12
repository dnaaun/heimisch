use serde::{Deserialize, Serialize};

use super::{installation::InstallationId, issue_initial_sync_status::InitialSyncStatusEnum};

/// Serde internal tagging is necessary if we're going to index on `id` in IndexedDb.
#[derive(macros::TypesafeIdb)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub struct RepositoryInitialSyncStatus {
    pub status: InitialSyncStatusEnum,
    #[idb(id)]
    pub id: InstallationId,
}
