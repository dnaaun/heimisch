#![allow(async_fn_in_trait)]

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::types::{repository::{Repository, RepositoryId}, user::{User, UserId}};

#[marker]
pub trait TableMarkerFor<Table> {}

impl<T, M1, M2> TableMarkerFor<T> for (M1, M2) where M1: TableMarkerFor<T> {}
impl<T, M1, M2> TableMarkerFor<T> for (M1, M2) where M2: TableMarkerFor<T> {}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct StoreName(pub &'static str);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct SerializedId(pub String);

pub struct ReactivityTrackers {
    pub tables_accessed_by_id: HashMap<StoreName, HashSet<SerializedId>>,
    pub tables_accessed_in_bulk: HashSet<StoreName>,
}

pub trait DbBuilder {
    type Error;
    type Db<DbTableMarkers>: Db<DbTableMarkers, Error = Self::Error>;

    async fn new() -> Self;
    fn with_commit_listener(self, commit_listener: Rc<dyn Fn(&ReactivityTrackers)>) -> Self;
    async fn build(
        self,
    ) -> Result<Self::Db<impl TableMarkerFor<User> + TableMarkerFor<Repository>>, Self::Error>;
}

pub struct ReadOnly;
pub struct ReadWrite;

// NOTE: Seal this trait.
pub trait SupportsRead {}
// NOTE: Seal this trait.
pub trait SupportsWrite {}

impl SupportsRead for ReadOnly {}
impl SupportsWrite for ReadWrite {}

pub trait Db<TableMarkers> {
    type Error;
    type TxnBuilder<TableMarkersFromDb, TxnTableMarkers, Mode>: TxnBuilder<
        TableMarkersFromDb,
        TxnTableMarkers,
        Mode,
    >;

    fn txn_builder(&self) -> Self::TxnBuilder<TableMarkers, (), ReadOnly>;
}

pub trait TxnBuilder<TableMarkersFromDb, TableMarkers, Mode> {
    type Txn<TxnTableMarers>;
    fn with_table<U>(self) -> (impl TableMarkerFor<U>, TableMarkers)
    where
        TableMarkersFromDb: TableMarkerFor<U>;

    fn build(self) -> Self::Txn<TableMarkers>;

    fn read_only(self) -> impl TxnBuilder<TableMarkersFromDb, TableMarkers, ReadOnly>;
    fn read_write(self) -> impl TxnBuilder<TableMarkersFromDb, TableMarkers, ReadWrite>;
}

pub trait Txn<TableMarkers, Mode> {
    type Error;
    type TableManager<T, TableManagerMode>: TableManager<T, TableManagerMode>;
    async fn get_table<T>(&self) -> Result<Self::TableManager<T, Mode>, Self::Error>
    where
        TableMarkers: TableMarkerFor<T>;
}

pub trait TableManager<T, Mode> {
    type Error;

    async fn get(&self, id: T::Id) -> Result<Vec<T>, Self::Error>
    where
        T: Table,
        Mode: SupportsRead;

    async fn get_all(&self) -> Result<Vec<T>, Self::Error>
    where
        Mode: SupportsRead;

    async fn delete(&self, id: &T::Id) -> Result<(), Self::Error>
    where
        T: Table,
        Mode: SupportsWrite;

    async fn put(&self, item: &T) -> Result<(), Self::Error>
    where
        T: Table,
        Mode: SupportsWrite;
}

pub trait Table: Sized {
    type Id;

    fn id(&self) -> &Self::Id;
}

impl Table for User {
    type Id = UserId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

impl Table for Repository {
    type Id = RepositoryId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
