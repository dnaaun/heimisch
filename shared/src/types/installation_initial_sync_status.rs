use serde::{Deserialize, Serialize};

use super::{installation::InstallationId, issues_initial_sync_status::InitialSyncStatusEnum};

/// Serde internal tagging is necessary if we're going to index on `id` in IndexedDb.
#[derive(macros::Table, Debug, Serialize, Deserialize, Clone, Default)]
#[serde(tag = "type")]
pub struct InstallationInitialSyncStatus {
    pub status: InitialSyncStatusEnum,
    #[db(id)]
    pub id: InstallationId,
}
