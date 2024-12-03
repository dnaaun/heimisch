use serde::{Deserialize, Serialize};

use super::{issue_initial_sync_status::InitialSyncStatusEnum, repository::RepositoryId};

/// Serde internal tagging is necessary if we're going to index on `id` in IndexedDb.
#[derive(macros::TypesafeIdb, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub struct PullRequestInitialSyncStatus {
    status: InitialSyncStatusEnum,
    #[idb(id)]
    id: RepositoryId,
}
