pub mod idb_impl;

use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use serde::{de::DeserializeOwned, Serialize};

#[marker]
pub trait TableMarker<T> {}

impl<Head, Tail, T> TableMarker<T> for (Head, Tail) where Tail: TableMarker<T> {}

impl<Head, Tail, T> TableMarker<T> for (Head, Tail) where Head: TableMarker<T> {}

impl<T: Table> TableMarker<T> for T::Marker {}

pub trait Table: DeserializeOwned + Serialize {
    const NAME: &'static str;

    /// I _feel_ like I'm going to need this.
    type Marker: Default;

    /// I need the `Serialize` here to make it easier to integrate with idb.
    type Id: serde::Serialize;

    fn id(&self) -> &Self::Id;
}

#[allow(async_fn_in_trait)]
pub trait RawTableAccessTrait<R: Table> {
    type Error;

    async fn get(&self, id: &R::Id) -> Result<Option<R>, Self::Error>;
    async fn get_all(&self) -> Result<Vec<R>, Self::Error>;
    async fn put(&self, item: &R) -> Result<(), Self::Error>;
    async fn delete(&self, id: &R::Id) -> Result<(), Self::Error>;
}

pub trait RawTxnTrait {
    type Error;
    type RawTableAccess<R: Table>: RawTableAccessTrait<R, Error = Self::Error>;

    fn commit(self) -> Result<(), Self::Error>;
    fn abort(self) -> Result<(), Self::Error>;
    fn get_table<R: Table>(&self, store_name: &str)
        -> Result<Self::RawTableAccess<R>, Self::Error>;
}

pub trait RawDbBuilderTrait {
    type Error;
    type Db: RawDbTrait<Error = Self::Error>;

    #[allow(async_fn_in_trait)]
    async fn build(self) -> Result<Self::Db, Self::Error>;

    fn add_table(self, table_builder: <Self::Db as RawDbTrait>::RawTableBuilder) -> Self;
}

pub trait RawDbTrait {
    type Error;
    type RawTxn: RawTxnTrait<Error = Self::Error>;
    type RawDbBuilder: RawDbBuilderTrait<Error = Self::Error, Db = Self>;
    type RawIndex: RawIndexTrait<Error = Self::Error>;
    type RawTableBuilder;

    fn txn(&self, store_names: &[&str], read_write: bool) -> Result<Self::RawTxn, Self::Error>;
    fn builder(name: &str) -> Self::RawDbBuilder;

    fn table_builder<R: Table>() -> Self::RawTableBuilder;
}

pub struct Db<RawDb, TableMarkers> {
    markers: PhantomData<TableMarkers>,
    pub(crate) raw: RawDb,
}

pub struct DbBuilder<RawDb: RawDbTrait, TableMarkers> {
    name: String,
    markers: PhantomData<TableMarkers>,
    table_builders: HashMap<TypeId, RawDb::RawTableBuilder>,
}

impl<RawDb: RawDbTrait, TableMarkers> Db<RawDb, TableMarkers> {
    pub fn builder(name: String) -> DbBuilder<RawDb, ()> {
        DbBuilder {
            name,
            markers: PhantomData,
            table_builders: Default::default(),
        }
    }
}

impl<RawDb: RawDbTrait, TableMarkers> DbBuilder<RawDb, TableMarkers> {
    pub fn with_table<R: Table + 'static>(mut self) -> DbBuilder<RawDb, (R::Marker, TableMarkers)> {
        self.table_builders
            .insert(TypeId::of::<R>(), RawDb::table_builder::<R>());
        DbBuilder {
            name: self.name,
            markers: PhantomData,
            table_builders: self.table_builders,
        }
    }

    pub async fn build(self) -> Result<Db<RawDb, TableMarkers>, RawDb::Error> {
        let db_builder = self.table_builders.into_iter().fold(
            RawDb::builder(&self.name),
            |db_builder, (_, table_builder)| db_builder.add_table(table_builder),
        );

        let db = db_builder.build().await?;

        Ok(Db {
            markers: PhantomData,
            raw: db,
        })
    }
}

impl<RawDb: RawDbTrait, DbTableMarkers> Db<RawDb, DbTableMarkers> {
    pub fn txn(&self) -> TxnBuilder<'_, RawDb, DbTableMarkers, (), ReadOnly> {
        TxnBuilder {
            store_names: Default::default(),
            txn_table_markers: Default::default(),
            db: self,
            mode: PhantomData,
        }
    }
}

pub struct Present;

pub struct ReadOnly;
pub struct ReadWrite;

pub trait TxnMode {
    const IS_READ_WRITE: bool;

    // I need this because Rust's support for constraining on associated consts is incomplete.
    // See #92827 <https://github.com/rust-lang/rust/issues/92827>
    type SupportsReadWrite;
}

impl TxnMode for ReadOnly {
    const IS_READ_WRITE: bool = false;
    type SupportsReadWrite = ();
}

impl TxnMode for ReadWrite {
    const IS_READ_WRITE: bool = true;
    type SupportsReadWrite = Present;
}
pub struct TxnBuilder<'db, RawDb: RawDbTrait, DbTableMarkers, TxnTableMarkers, Mode> {
    db: &'db Db<RawDb, DbTableMarkers>,
    txn_table_markers: TxnTableMarkers,
    store_names: HashSet<&'static str>,
    mode: PhantomData<Mode>,
}

impl<'db, Db: RawDbTrait, DbTableMarkers, TxnTableMarkers, Mode>
    TxnBuilder<'db, Db, DbTableMarkers, TxnTableMarkers, Mode>
where
    TxnTableMarkers: Default,
    Mode: TxnMode,
{
    pub fn with_table<R: Table + 'static>(
        self,
    ) -> TxnBuilder<'db, Db, DbTableMarkers, (R::Marker, TxnTableMarkers), Mode> {
        let new_markers = (R::Marker::default(), TxnTableMarkers::default());
        let mut new_table_names = self.store_names;
        new_table_names.insert(&R::NAME);

        TxnBuilder {
            txn_table_markers: new_markers,
            store_names: new_table_names,
            db: self.db,
            mode: self.mode,
        }
    }

    pub fn read_write(self) -> TxnBuilder<'db, Db, DbTableMarkers, TxnTableMarkers, ReadWrite> {
        TxnBuilder {
            txn_table_markers: self.txn_table_markers,
            store_names: self.store_names,
            db: self.db,
            mode: PhantomData,
        }
    }

    pub fn read_only(self) -> TxnBuilder<'db, Db, DbTableMarkers, TxnTableMarkers, ReadOnly> {
        TxnBuilder {
            txn_table_markers: self.txn_table_markers,
            store_names: self.store_names,
            db: self.db,
            mode: PhantomData,
        }
    }

    pub async fn build(self) -> Result<Txn<Db, TxnTableMarkers, Mode>, Db::Error> {
        let raw_txn = self.db.raw.txn(
            &self.store_names.into_iter().collect::<Vec<_>>(),
            Mode::IS_READ_WRITE,
        )?;
        Ok(Txn {
            markers: self.txn_table_markers,
            raw_txn: Some(raw_txn),
            mode: PhantomData,
        })
    }
}

pub struct Txn<Db: RawDbTrait, TableMarkers, Mode> {
    #[allow(unused)]
    markers: TableMarkers,
    raw_txn: Option<Db::RawTxn>,
    mode: PhantomData<Mode>,
}

impl<Db: RawDbTrait, TableMarkers, Mode> Txn<Db, TableMarkers, Mode> {
    pub fn table<R>(&self) -> Result<TableAccess<Db, R, Mode>, Db::Error>
    where
        TableMarkers: TableMarker<R>,
        R: Table,
    {
        let raw_table = self
            .raw_txn
            .as_ref()
            .map(|t| t.get_table(&R::NAME))
            .expect(
            "Should be None only if committed/aborted, which means &self shouldn't be obtainable",
        )?;
        Ok(TableAccess {
            raw_table,
            mode: PhantomData,
        })
    }

    pub fn commit(mut self) -> Result<(), Db::Error> {
        self.raw_txn.take().expect("Should be None only if committed/aborted, which means &self shouldn't be obtainable").commit()?;
        Ok(())
    }

    pub fn abort(mut self) -> Result<(), Db::Error> {
        self.raw_txn.take().expect("Should be None only if committed/aborted, which means &self shouldn't be obtainable").abort()?;
        Ok(())
    }
}

pub struct TableAccess<Db: RawDbTrait, R: Table, Mode> {
    pub(crate) raw_table: <Db::RawTxn as RawTxnTrait>::RawTableAccess<R>,
    pub(crate) mode: PhantomData<(R, Mode)>,
}

impl<Db: RawDbTrait, R: Table, Mode> TableAccess<Db, R, Mode>
where
    Mode: TxnMode,
{
    pub async fn get(&self, id: &R::Id) -> Result<Option<R>, Db::Error> {
        self.raw_table.get(id).await
    }

    pub async fn get_all(&self) -> Result<Vec<R>, Db::Error> {
        self.raw_table.get_all().await
    }
}

impl<Db: RawDbTrait, R: Table, Mode> TableAccess<Db, R, Mode>
where
    Mode: TxnMode<SupportsReadWrite = Present>,
{
    pub async fn put(&self, item: &R) -> Result<(), Db::Error> {
        self.raw_table.put(item).await
    }

    pub async fn delete(&self, id: &R::Id) -> Result<(), Db::Error> {
        self.raw_table.delete(id).await
    }
}

pub trait IndexSpec {
    type Table: Table;
    const NAME: &'static str;

    // The `Eq` requirement is used when doing optimistic updates, and it's not really
    // unrealistic at all to expect things that indexed by indexed db have a `Eq` Rust
    // representation.
    type Type: Serialize + Eq;

    fn get_index_value(row: &Self::Table) -> &Self::Type;
}

#[allow(async_fn_in_trait)]
pub trait RawIndexTrait {
    type Error;

    async fn get<IS: IndexSpec>(&self, value: &IS::Type) -> Result<Option<IS::Table>, Self::Error>;
    async fn get_all<IS: IndexSpec>(
        &self,
        value: Option<&IS::Type>,
    ) -> Result<Vec<IS::Table>, Self::Error>;
}

pub struct Index<RawDb: RawDbTrait, IS> {
    pub(crate) actual_index: RawDb::RawIndex,
    pub(crate) _markers: PhantomData<IS>,
}

impl<RawDb: RawDbTrait, IS: IndexSpec> Index<RawDb, IS> {
    pub async fn get(&self, value: &IS::Type) -> Result<Option<IS::Table>, RawDb::Error> {
        self.actual_index.get::<IS>(value).await
    }

    pub async fn get_all(&self, value: Option<&IS::Type>) -> Result<Vec<IS::Table>, RawDb::Error> {
        self.actual_index.get_all::<IS>(value).await
    }
}
