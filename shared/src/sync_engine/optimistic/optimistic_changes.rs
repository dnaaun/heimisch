#![allow(dead_code)]
#![allow(clippy::type_complexity)]

use std::{any::Any, future::Future, hash::Hash, rc::Rc};

use any_spawner::Executor;
use typesafe_idb::Store;

use crate::types::user::User;

use super::{db::SerializedId, optimistic_change_map::OptimisticChangeMap};

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

#[derive(Default)]
pub struct OptimisticChanges {
    pub updates: OptimisticChangeMap<Rc<dyn Any>>,
    pub creations: OptimisticChangeMap<Rc<dyn Any>, SerializedId>,
    pub deletes: OptimisticChangeMap<()>,
}

impl OptimisticChanges {
    pub fn update<S: Store + 'static>(
        &self,
        row: S,
        update_fut: impl Future<Output = Result<(), ()>> + 'static,
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

    pub fn create<S: Store + 'static>(
        &self,
        row: S,
        // The future must resolve to the id of whatever is created.
        create_fut: impl Future<Output = Result<S::Id, ()>> + 'static,
    ) {
        let id = row.id().clone();
        let creations = self.creations.clone();
        let time = creations.insert::<S>(&id, Rc::new(row));

        Executor::spawn_local(async move {
            match create_fut.await {
                Ok(actual_id) => {
                    creations.mark_successful::<S>(
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

    pub fn delete<S: Store>(
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
                    deletes.mark_successful::<S>(&id, &time, ());
                }
                Err(_) => {
                    deletes.remove_pending::<S>(&id, &time);
                }
            }
        });
    }

    pub fn remove_obsoletes_for_id<S: Store>(&self, id: &S::Id) {
        tracing::info!("Called remove obseletes_for_id: {id:?}");
        self.deletes.remove_all_successful::<S>(id, &());
        self.updates.remove_all_successful::<S>(id, &());
        self.creations
            .remove_all_successful::<S>(id, &SerializedId::new_from_id::<S>(id));
    }
}
