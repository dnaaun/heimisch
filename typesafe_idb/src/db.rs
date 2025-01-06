use std::{any::TypeId, collections::HashMap, marker::PhantomData, rc::Rc};

use idb::builder::ObjectStoreBuilder;

use crate::{chain::Chain, ReactivityTrackers, ReadOnly, Store, Txn, TxnBuilder};

pub type CommitListener = Rc<dyn Fn(&ReactivityTrackers)>;

pub struct TypesafeDb<TableMarkers> {
    markers: PhantomData<TableMarkers>,
    pub(crate) listener: Option<CommitListener>,
    pub(crate) inner: idb::Database,
}

pub struct TypesafeDbBuilder<TableMarkers> {
    name: String,
    markers: PhantomData<TableMarkers>,
    object_store_builders: HashMap<TypeId, ObjectStoreBuilder>,
    commit_listener: Option<CommitListener>,
}

impl TypesafeDb<()> {
    pub fn builder(name: String) -> TypesafeDbBuilder<Chain<(), ()>> {
        TypesafeDbBuilder {
            name,
            markers: PhantomData,
            object_store_builders: Default::default(),
            commit_listener: Default::default(),
        }
    }
}

impl<DbTableMarkers> TypesafeDb<DbTableMarkers> {
    pub fn txn(&self) -> TxnBuilder<'_, DbTableMarkers, Chain<(), ()>, ReadOnly> {
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
            commit_listener: self.commit_listener,
        }
    }

    pub fn with_commit_listener(self, commit_listener: Rc<dyn Fn(&ReactivityTrackers)>) -> Self {
        TypesafeDbBuilder {
            commit_listener: Some(commit_listener),
            ..self
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
            listener: self.commit_listener,
        })
    }
}
