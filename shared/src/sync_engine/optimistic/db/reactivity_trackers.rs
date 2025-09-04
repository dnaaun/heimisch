use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use bon::bon;
use parking_lot::RwLock;
use typed_db::raw_traits::SerializedId;

#[derive(Debug, Default)]
pub struct ReactivityTrackersInner {
    /// The value of the hasmap is the serde serialized value of the id.
    stores_read_by_id: RwLock<HashMap<&'static str, HashSet<SerializedId>>>,
    /// This will include get_all() accesses and also access through indices.
    stores_read_in_bulk: RwLock<HashSet<&'static str>>,

    stores_modified: RwLock<HashMap<&'static str, HashSet<SerializedId>>>,
}

#[derive(Debug, Clone, Default, derive_more::Deref)]
pub struct ReactivityTrackers {
    pub inner: Arc<ReactivityTrackersInner>,
}

#[bon]
impl ReactivityTrackers {
    #[builder]
    pub fn new(
        stores_read_by_id: Option<HashMap<&'static str, HashSet<SerializedId>>>,
        stores_read_in_bulk: Option<HashSet<&'static str>>,
        stores_modified: Option<HashMap<&'static str, HashSet<SerializedId>>>,
    ) -> Self {
        Self {
            inner: Arc::new(ReactivityTrackersInner {
                stores_read_by_id: RwLock::new(stores_read_by_id.unwrap_or_default()),
                stores_read_in_bulk: RwLock::new(stores_read_in_bulk.unwrap_or_default()),
                stores_modified: RwLock::new(stores_modified.unwrap_or_default()),
            }),
        }
    }

    pub fn is_affected_by_writes_in(&self, other: &ReactivityTrackers) -> bool {
        ({
            self.stores_read_by_id
                .read()
                .iter()
                .any(|(store_name_a, ids_a)| {
                    other
                        .stores_modified
                        .read()
                        .get(store_name_a)
                        .map(|ids_b| ids_a.intersection(ids_b).count() > 0)
                        .unwrap_or(false)
                })
        } || {
            self.stores_read_in_bulk
                .read()
                .iter()
                .any(|store_name_a| other.stores_modified.read().contains_key(store_name_a))
        })
    }

    pub fn add_by_id_read(&self, store_name: &'static str, serialized_id: SerializedId) {
        self.stores_read_by_id
            .write()
            .entry(store_name)
            .or_default()
            .insert(serialized_id);
    }

    pub fn add_bulk_read(&self, store_name: &'static str) {
        self.stores_read_in_bulk.write().insert(store_name);
    }

    pub fn add_modification(&self, store_name: &'static str, serialized_id: SerializedId) {
        self.stores_modified
            .write()
            .entry(store_name)
            .or_default()
            .insert(serialized_id);
    }
}

pub type CommitListener = Arc<dyn Fn(&ReactivityTrackers) + Send + Sync>;
