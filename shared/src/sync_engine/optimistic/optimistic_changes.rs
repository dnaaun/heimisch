#![allow(dead_code)]
#![allow(clippy::type_complexity)]

use std::{any::Any, future::Future, sync::Arc};

use any_spawner::Executor;

use typed_db::Table;

use super::{db::SerializedId, optimistic_change_map::OptimisticChangeMap};

#[derive(Default, Clone)]
pub struct OptimisticChanges {
    pub updates: OptimisticChangeMap<Arc<dyn Any + Send + Sync>>,
    pub creations: OptimisticChangeMap<Arc<dyn Any + Send + Sync>, SerializedId>,
    pub deletes: OptimisticChangeMap<()>,
}

impl OptimisticChanges {
    pub fn update<S: Table + 'static>(
        &self,
        row: S,
        update_fut: impl Future<Output = Result<(), ()>> + 'static,
    ) {
        let updates = self.updates.clone();
        let id = row.id().clone();
        let now = updates.insert::<S>(&id, Arc::new(row));

        Executor::spawn_local(async move {
            match update_fut.await {
                Ok(_) => {
                    updates.mark_realistic::<S>(&id, &now, ());
                }
                Err(_) => {
                    updates.remove_pending::<S>(&id, &now);
                }
            }
        });
    }

    pub fn create<S: Table + 'static>(
        &self,
        row: S,
        // The future must resolve to the id of whatever is created.
        create_fut: impl Future<Output = Result<S::Id, ()>> + Send + Sync + 'static,
    ) {
        let id = row.id().clone();
        let creations = self.creations.clone();
        let time = creations.insert::<S>(&id, Arc::new(row));

        Executor::spawn(async move {
            match create_fut.await {
                Ok(actual_id) => {
                    creations.mark_realistic::<S>(
                        &id,
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

    pub fn delete<S: Table>(
        &self,
        id: &S::Id,
        delete_fut: impl Future<Output = Result<(), ()>> + 'static,
    ) {
        let deletes = self.deletes.clone();
        let time = deletes.insert::<S>(id, ());
        let id = id.clone();

        Executor::spawn_local(async move {
            match delete_fut.await {
                Ok(_) => {
                    deletes.mark_realistic::<S>(&id, &time, ());
                }
                Err(_) => {
                    deletes.remove_pending::<S>(&id, &time);
                }
            }
        });
    }

    /// This can be refactored (along with mark_realistic).
    pub fn remove_successful_for_id<S: Table>(&self, id: &S::Id) {
        self.deletes.remove_all_realistic::<S>(&());
        self.updates.remove_all_realistic::<S>(&());
        self.creations
            .remove_all_realistic::<S>(&SerializedId::new_from_id::<S>(id));
    }

    pub fn get_realistic_to_optimistic_for_creations<S: Table>(
        &self,
        realistic_id: &S::Id,
    ) -> Option<S::Id> {
        self.creations
            .get_realistic_to_optimistic(&SerializedId::new_from_id::<S>(realistic_id))
            .map(|id| id.to_unserialized_id::<S>())
    }

    pub fn get_optimistic_to_realistic_for_creations<S: Table>(
        &self,
        optimistic_id: &S::Id,
    ) -> Option<S::Id> {
        self.creations
            .get_optimistic_to_realistic(&SerializedId::new_from_id::<S>(optimistic_id))
            .map(|id| id.to_unserialized_id::<S>())
    }
}
