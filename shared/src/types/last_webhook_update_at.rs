use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Default, PartialOrd, Ord,
)]
pub enum LastWebhookUpdateAtId {
    #[default]
    Singleton,
}

/// Serde internal tagging is necessary if we're going to index on `id` in IndexedDb.
#[derive(macros::Table, Debug, Serialize, Deserialize, Clone, Default)]
pub struct LastWebhookUpdateAt {
    pub at: Timestamp,
    #[db(id)]
    pub id: LastWebhookUpdateAtId,
}
