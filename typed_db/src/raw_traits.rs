use super::{IndexSpec, Table};
use std::fmt::Debug;

pub trait RawTableAccessTrait<R: Table>: Send + Sync {
    type RawDb: RawDbTrait;

    fn get(
        &self,
        id: &R::Id,
    ) -> impl Future<Output = Result<Option<R>, <Self::RawDb as RawDbTrait>::Error>> + Send + Sync;
    fn get_all(
        &self,
    ) -> impl Future<Output = Result<Vec<R>, <Self::RawDb as RawDbTrait>::Error>> + Send + Sync;
    fn put(
        &self,
        item: &R,
    ) -> impl Future<Output = Result<(), <Self::RawDb as RawDbTrait>::Error>> + Send + Sync;
    fn delete(
        &self,
        id: &R::Id,
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
    fn get_table<R: Table>(
        &self,
        store_name: &str,
    ) -> <Self::RawDb as RawDbTrait>::RawTableAccess<R>;
}

pub trait RawDbBuilderTrait {
    type RawDb: RawDbTrait;

    async fn build(self) -> Result<Self::RawDb, <Self::RawDb as RawDbTrait>::Error>;

    fn add_table(self, table_builder: <Self::RawDb as RawDbTrait>::RawTableBuilder) -> Self;
}

pub trait RawDbTrait: Send + Sync {
    type Error: Debug;
    type RawTxn: RawTxnTrait<RawDb = Self>;
    type RawDbBuilder: RawDbBuilderTrait<RawDb = Self>;
    type RawIndex: RawIndexTrait<RawDb = Self>;
    type RawTableBuilder;
    type RawTableAccess<R: Table>: RawTableAccessTrait<R, RawDb = Self>;

    fn txn(
        &self,
        store_names: &[&str],
        read_write: bool,
    ) -> impl Future<Output = Self::RawTxn> + Send + Sync;
    fn builder(name: &str) -> Self::RawDbBuilder;

    fn table_builder<R: Table>() -> Self::RawTableBuilder;
}

#[allow(async_fn_in_trait)]
pub trait RawIndexTrait: Send + Sync {
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
