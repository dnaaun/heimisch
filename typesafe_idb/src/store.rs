use serde::{de::DeserializeOwned, Serialize};

#[allow(async_fn_in_trait)]
pub trait Store: Sized + Serialize + DeserializeOwned {
    const NAME: &'static str;
    type Marker: Default;

    /// If you want "db reactivity" to work, the serde_json::to_string() should not change (ie, the
    /// id shouldn't be a hashmap/struct, which can serialize to different stirngs because of order
    /// of keys. But I don't think indexeddb supports "non-primitive" keys anyways?).
    type Id: Serialize;

    fn id(&self) -> &Self::Id;

    fn object_store_builder() -> idb::builder::ObjectStoreBuilder;
}
