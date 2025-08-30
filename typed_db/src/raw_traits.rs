use super::{IndexSpec, Table};
use std::fmt::Debug;

#[allow(async_fn_in_trait)]
pub trait RawTableAccessTrait<R: Table> {
    type RawDb: RawDbTrait;

    async fn get(&self, id: &R::Id) -> Result<Option<R>, <Self::RawDb as RawDbTrait>::Error>;
    async fn get_all(&self) -> Result<Vec<R>, <Self::RawDb as RawDbTrait>::Error>;
    async fn put(&self, item: &R) -> Result<(), <Self::RawDb as RawDbTrait>::Error>;
    async fn delete(&self, id: &R::Id) -> Result<(), <Self::RawDb as RawDbTrait>::Error>;

    fn index(
        &self,
        name: &str,
    ) -> Result<<Self::RawDb as RawDbTrait>::RawIndex, <Self::RawDb as RawDbTrait>::Error>;
}

pub trait RawTxnTrait {
    type RawDb: RawDbTrait;

    fn commit(self) -> Result<(), <Self::RawDb as RawDbTrait>::Error>;
    fn abort(self) -> Result<(), <Self::RawDb as RawDbTrait>::Error>;
    fn get_table<R: Table>(
        &self,
        store_name: &str,
    ) -> Result<<Self::RawDb as RawDbTrait>::RawTableAccess<R>, <Self::RawDb as RawDbTrait>::Error>;
}

pub trait RawDbBuilderTrait {
    type RawDb: RawDbTrait;

    #[allow(async_fn_in_trait)]
    async fn build(self) -> Result<Self::RawDb, <Self::RawDb as RawDbTrait>::Error>;

    fn add_table(self, table_builder: <Self::RawDb as RawDbTrait>::RawTableBuilder) -> Self;
}

pub trait RawDbTrait {
    type Error: Debug;
    type RawTxn: RawTxnTrait<RawDb = Self>;
    type RawDbBuilder: RawDbBuilderTrait<RawDb = Self>;
    type RawIndex: RawIndexTrait<RawDb = Self>;
    type RawTableBuilder;
    type RawTableAccess<R: Table>: RawTableAccessTrait<R, RawDb = Self>;

    fn txn(&self, store_names: &[&str], read_write: bool) -> Result<Self::RawTxn, Self::Error>;
    fn builder(name: &str) -> Self::RawDbBuilder;

    fn table_builder<R: Table>() -> Self::RawTableBuilder;
}

#[allow(async_fn_in_trait)]
pub trait RawIndexTrait {
    type RawDb: RawDbTrait;

    async fn get<IS: IndexSpec>(
        &self,
        value: &IS::Type,
    ) -> Result<Option<IS::Table>, <Self::RawDb as RawDbTrait>::Error>;
    async fn get_all<IS: IndexSpec>(
        &self,
        value: Option<&IS::Type>,
    ) -> Result<Vec<IS::Table>, <Self::RawDb as RawDbTrait>::Error>;
}
