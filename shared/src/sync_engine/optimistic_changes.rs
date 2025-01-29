#![allow(dead_code)]
#![allow(clippy::type_complexity)]

use std::{any::Any, future::Future, hash::Hash, rc::Rc, sync::Arc};

use any_spawner::Executor;
use typesafe_idb::{
    Index, IndexSpec, ObjectStore, Present, SerializedId, Store, StoreMarker, Txn, TxnMode,
};

use crate::types::user::User;

use super::{
    changes::{Changes, ExistingOrDeleted},
    optimistic_change_map::OptimisticChangeMap,
};

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
    creations: OptimisticChangeMap<Rc<dyn Any>, SerializedId>,
    deletes: OptimisticChangeMap<()>,
}

impl OptimisticChanges {
    pub fn register_update<S: Store + 'static, T, E>(
        &self,
        row: S,
        update_fut: impl Future<Output = Result<T, E>> + 'static,
    ) {
        let updates = self.updates.clone();
        let id = row.id().clone();
        let now = updates.insert::<S>(&id, Rc::new(row));

        Executor::spawn_local(async move {
            match update_fut.await {
                Ok(_) => {
                    updates.mark_successful::<S>(&id, &now, ());
                }
                Err(_) => {
                    updates.remove_pending::<S>(&id, &now);
                }
            }
        });
    }

    pub async fn register_create<S: Store + 'static, E>(
        &self,
        row: S,
        // The future must resolve to the id of whatever is created.
        create_fut: impl Future<Output = Result<S::Id, E>> + 'static,
    ) {
        let id = row.id().clone();
        let creations = self.creations.clone();
        let time = creations.insert::<S>(&id, Rc::new(row));

        Executor::spawn_local(async move {
            match create_fut.await {
                Ok(actual_id) => {
                    creations.mark_successful::<S>(
                        &actual_id,
                        &time,
                        SerializedId::new_from_id::<S>(&actual_id),
                    );
                }
                Err(_) => {
                    creations.remove_pending::<S>(&id, &time);
                }
            }
        });
    }

    pub async fn register_delete<S: Store + 'static, T, E>(
        &self,
        id: &S::Id,
        delete_fut: impl Future<Output = Result<T, E>> + 'static,
    ) {
        let deletes = self.deletes.clone();
        let time = deletes.insert::<S>(id, ());
        let id = id.clone();

        Executor::spawn_local(async move {
            match delete_fut.await {
                Ok(_) => {
                    deletes.mark_successful::<S>(&id, &time, ());
                }
                Err(_) => {
                    deletes.remove_pending::<S>(&id, &time);
                }
            }
        });
    }

    pub fn remove_obsoletes(&self, changes: &Changes) {
        let Changes {
            github_apps,
            issues,
            issue_comments,
            users,
            repositorys,
            licenses,
            milestones,
            labels,
        } = changes;
        self.remove_obsoletes_for_store(&mut labels.values());
        self.remove_obsoletes_for_store(&mut milestones.values());
        self.remove_obsoletes_for_store(&mut licenses.values());
        self.remove_obsoletes_for_store(&mut repositorys.values());
        self.remove_obsoletes_for_store(&mut users.values());
        self.remove_obsoletes_for_store(&mut issue_comments.values());
        self.remove_obsoletes_for_store(&mut issues.values());
        self.remove_obsoletes_for_store(&mut github_apps.values());
    }

    pub fn remove_obsoletes_for_store<S: Store>(
        &self,
        items: &mut dyn Iterator<Item = &ExistingOrDeleted<S>>,
    ) {
        for item in items {
            let id = match item {
                ExistingOrDeleted::Existing(item) => item.id(),
                ExistingOrDeleted::Deleted(id) => id,
            };

            self.remove_obsoletes_for_id::<S>(id);
        }
    }

    pub fn remove_obsoletes_for_id<S: Store>(&self, id: &S::Id) {
        self.deletes.remove_all_successful::<S>(id, &());
        self.updates.remove_all_successful::<S>(id, &());
        self.creations
            .remove_all_successful::<S>(id, &SerializedId::new_from_id::<S>(id));
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
            optimistic_changes: self.optimistic_updates.clone(),
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
    optimistic_changes: Arc<OptimisticChanges>,
    inner: ObjectStore<'a, S, Mode>,
}

impl<S, Mode> ObjectStoreWithOptimisticChanges<'_, S, Mode>
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadOnly = Present>,
{
    pub async fn get(&self, id: &S::Id) -> Result<Option<S>, typesafe_idb::Error> {
        if self.optimistic_changes.deletes.latest::<S>(id).is_some() {
            return Ok(None);
        }
        let optimistically_updated = self.optimistic_changes.updates.latest_downcasted::<S>(id);
        if let Some(o) = optimistically_updated {
            return Ok(Some(o));
        }
        let optimistically_created = self.optimistic_changes.creations.latest_downcasted::<S>(id);
        if let Some(o) = optimistically_created {
            return Ok(Some(o));
        };

        self.inner.get(id).await
    }

    pub async fn get_all(&self) -> Result<Vec<S>, typesafe_idb::Error> {
        let from_db = self
            .inner
            .get_all()
            .await?
            .into_iter()
            .filter(|r| {
                self.optimistic_changes
                    .deletes
                    .latest::<S>(r.id())
                    .is_none()
            })
            .map(|r| {
                self.optimistic_changes
                    .updates
                    .latest_downcasted(r.id())
                    .unwrap_or(r)
            });
        let mut all = Vec::from_iter(from_db);
        all.extend(
            self.optimistic_changes
                .creations
                .all_the_latest_downcasted(),
        );

        Ok(all)
    }

    pub fn index<IS: IndexSpec<Store = S>>(&self) -> Result<Index<'_, IS>, typesafe_idb::Error> {
        todo!()
    }
}

impl<S, Mode> ObjectStoreWithOptimisticChanges<'_, S, Mode>
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadWrite = Present>,
{
    pub async fn delete(&self, id: &S::Id) -> Result<(), typesafe_idb::Error> {
        self.inner.delete(id).await?;
        self.optimistic_changes.remove_obsoletes_for_id::<S>(id);
        Ok(())
    }

    pub async fn put(&self, item: &S) -> Result<(), typesafe_idb::Error> {
        self.inner.put(item).await?;
        self.optimistic_changes.remove_obsoletes_for_id::<S>(item.id());
        Ok(())
    }
}
