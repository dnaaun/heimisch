use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use typesafe_idb::Store;

use crate::typed_db::Table;

#[derive(Debug, Ord, PartialOrd, Hash, PartialEq, Eq, Clone)]
pub struct SerializedId(String);

impl std::ops::Deref for SerializedId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SerializedId {
    pub fn new_from_row<S: Table>(row: &S) -> Self {
        Self(serde_json::to_string(&row.id()).unwrap())
    }

    pub fn new_from_id<S: Table>(id: &S::Id) -> Self {
        Self(serde_json::to_string(&id).expect("did not expect ids not to be json serializable?"))
    }

    pub fn to_unserialized_id<S: Table>(&self) -> S::Id {
        serde_json::from_str(&self.0).expect("did not expect ids not to be json de-serializable?")
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReactivityTrackers {
    /// The value of the hasmap is the serde serialized value of the id.
    pub stores_read_by_id: HashMap<&'static str, HashSet<SerializedId>>,
    /// This will include get_all() accesses and also access through indices.
    pub stores_read_in_bulk: HashSet<&'static str>,

    pub stores_modified: HashMap<&'static str, HashSet<SerializedId>>,
}

impl ReactivityTrackers {
    pub fn is_affected_by_writes_in(&self, other: &ReactivityTrackers) -> bool {
        self.stores_read_by_id.iter().any(|(store_name_a, ids_a)| {
            other
                .stores_modified
                .get(store_name_a)
                .map(|ids_b| ids_a.intersection(ids_b).count() > 0)
                .unwrap_or(false)
        }) || self
            .stores_read_in_bulk
            .iter()
            .any(|store_name_a| other.stores_modified.contains_key(store_name_a))
    }

    pub fn add_by_id_read(&mut self, store_name: &'static str, serialized_id: SerializedId) {
        self.stores_read_by_id
            .entry(store_name)
            .or_default()
            .insert(serialized_id);
    }

    pub fn add_bulk_read(&mut self, store_name: &'static str) {
        self.stores_read_in_bulk.insert(store_name);
    }

    pub fn add_modification(&mut self, store_name: &'static str, serialized_id: SerializedId) {
        self.stores_modified
            .entry(store_name)
            .or_default()
            .insert(serialized_id);
    }
}

pub type CommitListener = Rc<dyn Fn(&ReactivityTrackers)>;
