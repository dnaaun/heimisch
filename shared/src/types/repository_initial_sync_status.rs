use serde::{Deserialize, Serialize};

use super::repository::RepositoryId;

#[derive(macros::Table, Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryInitialSyncStatus {
    pub status: RepoSyncStatus,
    #[db(id)]
    pub id: RepositoryId,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum RepoSyncStatus {
    Full,
    NoSync,
}
