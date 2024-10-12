use std::{any::TypeId, collections::HashMap, marker::PhantomData};

use idb::builder::ObjectStoreBuilder;

use crate::{Chain, Store, Txn, TxnBuilder};

pub struct TypesafeDb<TableMarkers> {
    markers: PhantomData<TableMarkers>,
    pub(crate) inner: idb::Database,
}

pub struct TypesafeDbBuilder<TableMarkers> {
    name: String,
    markers: PhantomData<TableMarkers>,
    object_store_builders: HashMap<TypeId, ObjectStoreBuilder>,
}

impl TypesafeDb<()> {
    pub fn builder(name: String) -> TypesafeDbBuilder<Chain<(), ()>> {
        TypesafeDbBuilder {
            name,
            markers: PhantomData,
            object_store_builders: Default::default(),
        }
    }
}


impl<DbTableMarkers> TypesafeDb<DbTableMarkers> {
    pub fn txn<'db>(&'db self) -> TxnBuilder<'db, Chain<(), ()>, DbTableMarkers> {
        Txn::builder(self)
    }
}

impl<DbTableMarkers> TypesafeDbBuilder<DbTableMarkers> {
    pub fn with_store<S: Store + 'static>(
        mut self,
    ) -> TypesafeDbBuilder<Chain<S::Marker, DbTableMarkers>> {
        self.object_store_builders
            .insert(TypeId::of::<S>(), S::object_store_builder());
        TypesafeDbBuilder {
            markers: PhantomData,
            object_store_builders: self.object_store_builders,
            name: self.name,
        }
    }

    pub async fn build(self) -> Result<TypesafeDb<DbTableMarkers>, crate::Error> {
        let db = self
            .object_store_builders
            .into_iter()
            .fold(
                idb::Database::builder(&self.name),
                |db_builder, (_, obj_store_builder)| db_builder.add_object_store(obj_store_builder),
            )
            .build()
            .await?;

        Ok(TypesafeDb {
            markers: PhantomData,
            inner: db,
        })
    }
}
