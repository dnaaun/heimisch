#[cfg(feature = "sqlite")]
use rusqlite::ToSql;
use serde::{Deserialize, Serialize};

use super::Table;
use std::fmt::Debug;

/// THis could be `serde_json::Value`, except that we use this in optimistic code
/// and we hash it, and `serde_json::Value` doesn't implement `Hash`.
#[derive(
    Debug, Ord, PartialOrd, Hash, PartialEq, Eq, Clone, derive_more::Display, derive_more::Deref,
)]
pub struct SerializedId(pub(crate) String);

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

#[cfg(feature = "sqlite")]
impl ToSql for SerializedId {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display, derive_more::Deref)]
pub struct SerializedObject(pub(crate) String);

#[cfg(feature = "sqlite")]
impl ToSql for SerializedObject {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.0.to_sql()
    }
}

impl SerializedObject {
    pub fn from_row<S: Table>(row: &S) -> Result<Self, serde_json::Error> {
        Ok(Self(serde_json::to_string(&row)?))
    }

    pub fn to_row<S: Table>(&self) -> Result<S, serde_json::Error> {
        Ok(serde_json::from_str(&self.0)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedValue(String);

impl SerializedValue {
    pub fn from_value<T: Serialize>(value: &T) -> Result<Self, serde_json::Error> {
        Ok(Self(serde_json::to_string(value)?))
    }
}

pub trait RawTableAccessTrait: Send + Sync {
    type RawDb: RawDbTrait;

    fn get(
        &self,
        id: &SerializedId,
    ) -> impl Future<Output = Result<Option<SerializedObject>, <Self::RawDb as RawDbTrait>::Error>>
    + Send
    + Sync;
    fn get_all(
        &self,
    ) -> impl Future<Output = Result<Vec<SerializedObject>, <Self::RawDb as RawDbTrait>::Error>>
    + Send
    + Sync;
    fn put(
        &self,
        id: &SerializedId,
        item: &SerializedObject,
    ) -> impl Future<Output = Result<(), <Self::RawDb as RawDbTrait>::Error>> + Send + Sync;
    fn delete(
        &self,
        id: &SerializedId,
    ) -> impl Future<Output = Result<(), <Self::RawDb as RawDbTrait>::Error>> + Send + Sync;

    fn index(&self, name: &str) -> <Self::RawDb as RawDbTrait>::RawIndex;
}

pub trait RawTxnTrait: Send + Sync {
    type RawDb: RawDbTrait;

    fn commit(
        self,
    ) -> impl Future<Output = Result<(), <Self::RawDb as RawDbTrait>::Error>> + Send + Sync;
    fn abort(
        self,
    ) -> impl Future<Output = Result<(), <Self::RawDb as RawDbTrait>::Error>> + Send + Sync;
    fn get_table(&self, table_name: &str) -> <Self::RawDb as RawDbTrait>::RawTableAccess;
}

pub trait RawDbBuilderTrait {
    type RawDb: RawDbTrait;

    fn build(self)
    -> impl Future<Output = Result<Self::RawDb, <Self::RawDb as RawDbTrait>::Error>>;

    fn add_table(self, table_builder: <Self::RawDb as RawDbTrait>::RawTableBuilder) -> Self;
}

pub trait RawDbTrait: Send + Sync + 'static {
    type Error: Debug + Send + Sync;
    type RawTxn: RawTxnTrait<RawDb = Self>;
    type RawDbBuilder: RawDbBuilderTrait<RawDb = Self>;
    type RawIndex: RawIndexTrait<RawDb = Self>;
    type RawTableBuilder;
    type RawTableAccess: RawTableAccessTrait<RawDb = Self>;

    fn txn(
        &self,
        table_names: &[&str],
        read_write: bool,
    ) -> impl Future<Output = Self::RawTxn> + Send + Sync;
    fn builder(name: &str) -> Self::RawDbBuilder;

    fn table_builder<R: Table>() -> Self::RawTableBuilder;
}

#[allow(async_fn_in_trait)]
pub trait RawIndexTrait: Send + Sync {
    type RawDb: RawDbTrait;

    fn get(
        &self,
        value: &SerializedValue,
    ) -> impl Future<Output = Result<Option<SerializedObject>, <Self::RawDb as RawDbTrait>::Error>>
    + Send
    + Sync;
    fn get_all(
        &self,
        value: Option<&SerializedValue>,
    ) -> impl Future<Output = Result<Vec<SerializedObject>, <Self::RawDb as RawDbTrait>::Error>>
    + Send
    + Sync;
}
