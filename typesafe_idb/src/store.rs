use std::hash::Hash;
use serde::{de::DeserializeOwned, Serialize};

use crate::StoreName;

#[allow(async_fn_in_trait)]
pub trait Store: Serialize + DeserializeOwned {
    const NAME: StoreName;
    type Marker: Default;

    /// If you want "db reactivity" to work, the serde_json::to_string() should not change (ie, the
    /// id shouldn't be a hashmap/struct, which can serialize to different stirngs because of order
    /// of keys. But I don't think indexeddb supports "non-primitive" keys anyways?).
    type Id: Serialize + Hash;

    fn id(&self) -> &Self::Id;

    fn object_store_builder() -> idb::builder::ObjectStoreBuilder;
}
