use std::{any::TypeId, collections::HashMap, marker::PhantomData};

use idb::builder::ObjectStoreBuilder;

use crate::{ReadOnly, Store, Txn, TxnBuilder};

pub struct TypesafeDb<StoreMarkers> {
    markers: PhantomData<StoreMarkers>,
    pub(crate) inner: idb::Database,
}

pub struct TypesafeDbBuilder<StoreMarkers> {
    name: String,
    markers: PhantomData<StoreMarkers>,
    object_store_builders: HashMap<TypeId, ObjectStoreBuilder>,
}

impl TypesafeDb<()> {
    pub fn builder(name: String) -> TypesafeDbBuilder<()> {
        TypesafeDbBuilder {
            name,
            markers: PhantomData,
            object_store_builders: Default::default(),
        }
    }
}

impl<DbStoreMarkers> TypesafeDb<DbStoreMarkers> {
    pub fn txn(&self) -> TxnBuilder<'_, DbStoreMarkers, (), ReadOnly> {
        Txn::builder(self)
    }
}

impl<DbStoreMarkers> TypesafeDbBuilder<DbStoreMarkers> {
    pub fn with_store<S: Store + 'static>(
        mut self,
    ) -> TypesafeDbBuilder<(S::Marker, DbStoreMarkers)> {
        self.object_store_builders
            .insert(TypeId::of::<S>(), S::object_store_builder());
        TypesafeDbBuilder {
            markers: PhantomData,
            object_store_builders: self.object_store_builders,
            name: self.name,
        }
    }

    pub async fn build(self) -> Result<TypesafeDb<DbStoreMarkers>, crate::Error> {
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
