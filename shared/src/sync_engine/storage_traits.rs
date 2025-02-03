#![allow(async_fn_in_trait)]

use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::types::{
    repository::{Repository, RepositoryId},
    user::{User, UserId},
};

#[marker]
pub trait TableMarkerFor<Table> {}

impl<T, M1, M2> TableMarkerFor<T> for (M1, M2) where M1: TableMarkerFor<T> {}
impl<T, M1, M2> TableMarkerFor<T> for (M1, M2) where M2: TableMarkerFor<T> {}

impl<T: Table> TableMarkerFor<T> for T::Marker {}

pub struct ReactivityTrackers {
    pub tables_accessed_by_id: HashMap<&'static str, HashSet<String>>,
    pub tables_accessed_in_bulk: HashSet<&'static str>,
}

pub trait DbBuilder {
    type Error;
    type Db: Db<Error = Self::Error>;

    async fn new() -> Self;
    fn with_commit_listener(self, commit_listener: Rc<dyn Fn(&ReactivityTrackers)>) -> Self;
    async fn build(self) -> Result<Self::Db, Self::Error>;
}

pub struct ReadOnly;
pub struct ReadWrite;

// NOTE: Seal this trait.
pub trait SupportsRead {}
// NOTE: Seal this trait.
pub trait SupportsWrite {}

impl SupportsRead for ReadOnly {}
impl SupportsWrite for ReadWrite {}

pub trait Db {
    type Error;
    type TableMarkers: TableMarkerFor<User> + TableMarkerFor<Repository>;
    type TxnBuilder<'db, TxnTableMarkers, Mode>
    where
        TxnTableMarkers: 'db,
        Mode: 'db,
        Self: 'db;

    fn txn_builder(&self) -> Self::TxnBuilder<'_, (), ReadOnly>;
}

pub trait TxnBuilder<'db, TableMarkersFromDb, TableMarkers, Mode>
where
    TableMarkersFromDb: Default,
    TableMarkers: Default,
{
    type Marker<U>
    where
        U: Table;

    type Myself<S, T, M>: TxnBuilder<'db, S, T, M>
    where
        S: 'db + Default,
        T: 'db + Default,
        M: 'db;

    type Txn: Txn<TableMarkers, Mode>;
    fn with_table<U: Table>(
        self,
    ) -> Self::Myself<TableMarkersFromDb, (impl TableMarkerFor<U> + Default, TableMarkers), Mode>
    where
        TableMarkersFromDb: TableMarkerFor<U>;

    fn build(self) -> Self::Txn;

    fn read_only(self) -> impl TxnBuilder<'db, TableMarkersFromDb, TableMarkers, ReadOnly>;
    fn read_write(self) -> impl TxnBuilder<'db, TableMarkersFromDb, TableMarkers, ReadWrite>;
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
    type Marker;

    fn id(&self) -> &Self::Id;
}

pub struct UserMarker;
impl Table for User {
    type Id = UserId;
    type Marker = UserMarker;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

pub struct RepositoryMarker;
impl Table for Repository {
    type Id = RepositoryId;
    type Marker = RepositoryMarker;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
