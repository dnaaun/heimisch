use std::cell::RefCell;
use std::panic::Location;
use std::rc::Rc;
use typesafe_idb::Store;

use typed_db::{Index, IndexSpec, RawDbTrait};

use crate::sync_engine::optimistic::optimistic_changes::OptimisticChanges;

use super::reactivity_trackers::ReactivityTrackers;
use super::MaybeOptimistic;

#[derive(derive_more::Constructor)]
pub struct IndexWithOptimisticChanges<'txn, RawDb: RawDbTrait, IS: IndexSpec> {
    optimistic_changes: Rc<OptimisticChanges>,
    inner: Index<RawDb, IS>,
    pub(crate) reactivity_trackers: &'txn RefCell<ReactivityTrackers>,
    txn_location: &'static Location<'static>,
}
impl<RawDb: RawDbTrait, IS: IndexSpec> IndexWithOptimisticChanges<'_, RawDb, IS> {
    pub async fn get_optimistically(
        &self,
        id: &IS::Type,
    ) -> Result<Option<MaybeOptimistic<IS::Store>>, RawDb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_bulk_read(IS::Store::NAME);

        let row = match self.get(id).await? {
            Some(r) => r,
            None => return Ok(None),
        };
        let id = row.id();
        if self
            .optimistic_changes
            .deletes
            .latest::<IS::Store>(id)
            .is_some()
        {
            return Ok(None);
        }
        Ok(self
            .optimistic_changes
            .updates
            .latest_downcasted(id)
            .map(|o| MaybeOptimistic::new(o, true))
            .or(Some(MaybeOptimistic::new(row, false))))
    }

    pub(crate) async fn get(&self, id: &IS::Type) -> Result<Option<IS::Store>, RawDb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_bulk_read(IS::Store::NAME);

        self.inner.get(id).await
    }

    pub async fn get_all_optimistically(
        &self,
        value: Option<&IS::Type>,
    ) -> Result<Vec<MaybeOptimistic<IS::Store>>, RawDb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_bulk_read(IS::Store::NAME);

        let from_db_filtered = self
            .inner
            .get_all(value)
            .await?
            .into_iter()
            .filter(|r| {
                self.optimistic_changes
                    .deletes
                    .latest::<IS::Store>(r.id())
                    .is_none()
            })
            .map(|r| {
                self.optimistic_changes
                    .updates
                    .latest_downcasted(r.id())
                    .map(|o| MaybeOptimistic::new(o, true))
                    .unwrap_or(MaybeOptimistic::new(r, false))
            });
        let mut all = Vec::from_iter(from_db_filtered);

        let optimistic_creations = self
            .optimistic_changes
            .creations
            .all_the_latest_downcasted();
        if let Some(value) = value {
            all.extend(
                optimistic_creations
                    .into_iter()
                    .filter(|row| IS::get_index_value(row) == value)
                    .map(|o| MaybeOptimistic::new(o, true)),
            );
        } else {
            all.extend(
                optimistic_creations
                    .into_iter()
                    .map(|o| MaybeOptimistic::new(o, true)),
            );
        }
        Ok(all)
    }

    #[allow(dead_code)]
    pub(crate) async fn get_all(
        &self,
        value: Option<&IS::Type>,
    ) -> Result<Vec<IS::Store>, RawDb::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_bulk_read(IS::Store::NAME);

        self.inner.get_all(value).await
    }
}
