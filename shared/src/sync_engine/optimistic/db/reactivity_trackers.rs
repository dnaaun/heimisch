use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use typesafe_idb::{Store, StoreName};

#[derive(Debug, Ord, PartialOrd, Hash, PartialEq, Eq, Clone)]
pub struct SerializedId(String);

impl std::ops::Deref for SerializedId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SerializedId {
    pub fn new_from_row<S: Store>(row: &S) -> Self {
        Self(serde_json::to_string(&row.id()).unwrap())
    }

    pub fn new_from_id<S: Store>(id: &S::Id) -> Self {
        Self(serde_json::to_string(&id).unwrap())
    }
}
#[derive(Debug, Clone, Default)]
pub struct ReactivityTrackers {
    /// The value of the hasmap is the serde serialized value of the id.
    pub stores_accessed_by_id: HashMap<StoreName, HashSet<SerializedId>>,

    /// This will include get_all() accesses and also index accesses.
    /// It maybe good
    pub stores_accessed_in_bulk: HashSet<StoreName>,
}
impl ReactivityTrackers {
    pub fn overlaps(&self, other: &ReactivityTrackers) -> bool {
        self.stores_accessed_by_id
            .iter()
            .any(|(store_name_a, ids_a)| {
                other.stores_accessed_in_bulk.contains(store_name_a)
                    || other
                        .stores_accessed_by_id
                        .get(store_name_a)
                        .map(|ids_b| ids_a.intersection(ids_b).count() > 0)
                        .unwrap_or(false)
            })
            || self.stores_accessed_in_bulk.iter().any(|store_name_a| {
                other.stores_accessed_in_bulk.contains(store_name_a)
                    || other.stores_accessed_by_id.contains_key(store_name_a)
            })
    }

    pub fn add_by_id_access(&mut self, store_name: StoreName, serialized_id: SerializedId) {
        self.stores_accessed_by_id
            .entry(store_name)
            .or_default()
            .insert(serialized_id);
    }

    pub fn add_bulk_access(&mut self, store_name: StoreName) {
        self.stores_accessed_in_bulk.insert(store_name);
    }
}

pub type CommitListener = Rc<dyn Fn(&ReactivityTrackers)>;
