use serde::{Deserialize, Serialize};

use super::repository::RepositoryId;

/// Serde internal tagging is necessary if we're going to index on `id` in IndexedDb.
#[derive(macros::Table, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub struct IssuesInitialSyncStatus {
    pub status: InitialSyncStatusEnum,
    #[db(id)]
    pub id: RepositoryId,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum InitialSyncStatusEnum {
    Full,
    Partial,
    #[default]
    NoSync,
}
