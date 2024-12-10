use serde::{Deserialize, Serialize};

use super::repository::RepositoryId;

/// Serde internal tagging is necessary if we're going to index on `id` in IndexedDb.
#[derive(macros::TypesafeIdb, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub struct IssuesInitialSyncStatus {
    pub status: InitialSyncStatusEnum,
    #[idb(id)]
    pub id: RepositoryId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InitialSyncStatusEnum {
    Full,
    Partial,
    NoSync,
}
