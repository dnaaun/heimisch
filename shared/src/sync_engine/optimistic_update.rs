#![allow(dead_code)]
#![allow(clippy::type_complexity)]

use std::{any::Any, collections::HashMap, future::Future, hash::Hash, sync::Arc};

use parking_lot::RwLock;
use typesafe_idb::{
    Index, IndexSpec, ObjectStore, Present, SerializedId, Store, StoreMarker, StoreName, Txn,
    TxnMode,
};

use crate::types::user::User;

/// Hashed by store name and hash.
#[derive(Debug, derive_more::From)]
struct OptimisticUpdateRow<S: Store>(S);

#[derive(Hash, Debug, derive_more::From)]
enum OptimisticUpdateRowEnum {
    User(OptimisticUpdateRow<User>),
}

impl<S: Store> Hash for OptimisticUpdateRow<S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (S::NAME, self.0.id()).hash(state)
    }
}

/// RTI: In any (key, value) pair,  the `StoreName` in the key determines the right type that
/// we can downcast `dyn Any` from in the value.
pub struct OptimisticUpdates {
    updates: RwLock<
        HashMap<
            StoreName,
            HashMap<
                SerializedId,
                // The `bool` indicates whether the future that is associated with the optimistic update
                // succeeded.
                (bool, Box<dyn Any>),
            >,
        >,
    >,
}

impl OptimisticUpdates {
    /// When `update_fut` is completed, if it's Ok(_), it will mark the optimistic update as having
    /// been successful The removal of that update from `OptimisticUpdates` will happen only when
    /// we get a webhook that matches the row that the optimistc update pertains to.
    pub async fn register<S: Store + 'static, T, E>(
        &self,
        row: S,
        update_fut: impl Future<Output = Result<T, E>>,
    ) -> Result<T, E> {
        let serialized_id = SerializedId::new_from_row(&row);
        self.updates
            .write()
            .entry(S::NAME)
            .or_default()
            .insert(serialized_id.clone(), (false, Box::new(row)));
        match update_fut.await {
            Ok(t) => {
                if let Some((future_completed, _)) = self
                    .updates
                    .write()
                    .get_mut(&S::NAME)
                    .expect(
                        "Should have been inserted when `OptimisticUpdates::registered` was run.",
                    )
                    .get_mut(&serialized_id)
                {
                    *future_completed = true
                }

                Ok(t)
            }
            Err(e) => {
                self.updates
                    .write()
                    .get_mut(&S::NAME)
                    .expect(
                        "Should have been inserted when `OptimisticUpdates::registered` was run.",
                    )
                    .remove(&serialized_id);
                Err(e)
            }
        }
    }
}

pub struct TxnWithOptimisticUpdates<C, Mode> {
    optimistic_updates: Arc<OptimisticUpdates>,
    inner: Txn<C, Mode>,
}

impl<Markers, Mode> TxnWithOptimisticUpdates<Markers, Mode> {
    pub fn object_store<S>(
        &self,
    ) -> Result<ObjectStoreWithOptimisticUpdates<'_, S, Mode>, typesafe_idb::Error>
    where
        S: Store,
        Markers: StoreMarker<S>,
    {
        Ok(ObjectStoreWithOptimisticUpdates {
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

pub struct ObjectStoreWithOptimisticUpdates<'a, S, Mode> {
    optimistic_updates: Arc<OptimisticUpdates>,
    inner: ObjectStore<'a, S, Mode>,
}

impl<S, Mode> ObjectStoreWithOptimisticUpdates<'_, S, Mode>
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadOnly = Present>,
{
    pub async fn get(&self, id: &S::Id) -> Result<Option<S>, typesafe_idb::Error> {
        todo!()
    }

    pub async fn get_all(&self) -> Result<Vec<S>, typesafe_idb::Error> {
        todo!()
    }

    pub fn index<IS: IndexSpec<Store = S>>(&self) -> Result<Index<'_, IS>, typesafe_idb::Error> {
        todo!()
    }
}
