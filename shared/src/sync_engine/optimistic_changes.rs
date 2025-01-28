#![allow(dead_code)]
#![allow(clippy::type_complexity)]

use std::{any::Any, future::Future, hash::Hash, rc::Rc, sync::Arc};

use typesafe_idb::{Index, IndexSpec, ObjectStore, Present, Store, StoreMarker, Txn, TxnMode};

use crate::types::user::User;

use super::optimistic_change_map::OptimisticChangeMap;

/// Hashed by store name and hash.
#[derive(Debug, derive_more::From)]
struct OptimisticChangeRow<S: Store>(S);

#[derive(Hash, Debug, derive_more::From)]
enum OptimisticChangeRowEnum {
    User(OptimisticChangeRow<User>),
}

impl<S: Store> Hash for OptimisticChangeRow<S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (S::NAME, self.0.id()).hash(state)
    }
}

pub struct OptimisticChanges {
    updates: OptimisticChangeMap<Rc<dyn Any>>,
    creations: OptimisticChangeMap<Rc<dyn Any>>,
    deletes: OptimisticChangeMap<()>,
}

impl OptimisticChanges {
    pub async fn register_update<S: Store + 'static, T, E>(
        &self,
        row: S,
        update_fut: impl Future<Output = Result<T, E>>,
    ) -> Result<T, E> {
        let id = row.id().clone();
        let now = self.updates.insert::<S>(&id, Rc::new(row));

        match update_fut.await {
            Ok(t) => {
                self.updates.mark_successful::<S>(&id, &now);
                Ok(t)
            }
            Err(e) => {
                self.updates.remove::<S>(&id, &now);
                Err(e)
            }
        }
    }

    pub async fn register_create<S: Store + 'static, T, E>(
        &self,
        row: S,
        delete_fut: impl Future<Output = Result<T, E>>,
    ) -> Result<T, E> {
        let id = row.id().clone();
        let time = self.creations.insert::<S>(&id, Rc::new(row));

        match delete_fut.await {
            Ok(t) => {
                self.deletes.mark_successful::<S>(&id, &time);
                Ok(t)
            }
            Err(e) => {
                self.deletes.remove::<S>(&id, &time);
                Err(e)
            }
        }
    }

    pub async fn register_delete<S: Store + 'static, T, E>(
        &self,
        id: &S::Id,
        delete_fut: impl Future<Output = Result<T, E>>,
    ) -> Result<T, E> {
        let time = self.deletes.insert::<S>(id, ());

        match delete_fut.await {
            Ok(t) => {
                self.deletes.mark_successful::<S>(id, &time);
                Ok(t)
            }
            Err(e) => {
                self.deletes.remove::<S>(id, &time);
                Err(e)
            }
        }
    }
}

pub struct TxnWithOptimisticChanges<C, Mode> {
    optimistic_updates: Arc<OptimisticChanges>,
    inner: Txn<C, Mode>,
}

impl<Markers, Mode> TxnWithOptimisticChanges<Markers, Mode> {
    pub fn object_store<S>(
        &self,
    ) -> Result<ObjectStoreWithOptimisticChanges<'_, S, Mode>, typesafe_idb::Error>
    where
        S: Store,
        Markers: StoreMarker<S>,
    {
        Ok(ObjectStoreWithOptimisticChanges {
            inner: self.inner.object_store::<S>()?,
            optimistic_updates: self.optimistic_updates.clone(),
        })
    }

    pub async fn commit(self) -> Result<typesafe_idb::ReactivityTrackers, idb::Error> {
        self.inner.commit().await
    }

    pub fn reactivity_trackers(&self) -> typesafe_idb::ReactivityTrackers {
        self.inner.reactivity_trackers()
    }

    pub async fn abort(self) -> Result<(), idb::Error> {
        self.inner.abort().await
    }
}

pub struct ObjectStoreWithOptimisticChanges<'a, S, Mode> {
    optimistic_updates: Arc<OptimisticChanges>,
    inner: ObjectStore<'a, S, Mode>,
}

impl<S, Mode> ObjectStoreWithOptimisticChanges<'_, S, Mode>
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadOnly = Present>,
{
    pub async fn get(&self, _id: &S::Id) -> Result<Option<S>, typesafe_idb::Error> {
        todo!()
    }

    pub async fn get_all(&self) -> Result<Vec<S>, typesafe_idb::Error> {
        todo!()
    }

    pub fn index<IS: IndexSpec<Store = S>>(&self) -> Result<Index<'_, IS>, typesafe_idb::Error> {
        todo!()
    }
}
