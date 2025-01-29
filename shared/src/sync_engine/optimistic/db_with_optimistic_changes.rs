use std::sync::Arc;

use typesafe_idb::{
    Index, IndexSpec, ObjectStore, Present, ReadOnly, ReadWrite, Store, StoreMarker, Txn,
    TxnBuilder, TxnMode, TypesafeDb,
};

use super::optimistic_changes::OptimisticChanges;

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

#[derive(Clone)]
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
        let from_db = self.inner.get_all().await?;

        let from_db_filtered = from_db
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
        let mut all = Vec::from_iter(from_db_filtered);
        all.extend(
            self.optimistic_changes
                .creations
                .all_the_latest_downcasted(),
        );

        Ok(all)
    }

    pub fn index<IS: IndexSpec<Store = S>>(
        &self,
    ) -> Result<IndexWithOptimisticChanges<'_, IS>, typesafe_idb::Error> {
        Ok(IndexWithOptimisticChanges {
            inner: self.inner.index::<IS>()?,
            optimistic_changes: self.optimistic_changes.clone(),
        })
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
        self.optimistic_changes
            .remove_obsoletes_for_id::<S>(item.id());
        Ok(())
    }
}

pub struct IndexWithOptimisticChanges<'a, IS> {
    optimistic_changes: Arc<OptimisticChanges>,
    inner: Index<'a, IS>,
}
impl<IS: IndexSpec> IndexWithOptimisticChanges<'_, IS> {
    pub async fn get(&self, id: &IS::Type) -> Result<Option<IS::Store>, typesafe_idb::Error> {
        let row = match self.inner.get(id).await? {
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
            .or(Some(row)))
    }

    pub async fn get_all(
        &self,
        value: Option<&IS::Type>,
    ) -> Result<Vec<IS::Store>, typesafe_idb::Error> {
        let from_db = self.inner.get_all(value).await?;
        let from_db_filtered = from_db
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
                    .unwrap_or(r)
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
                    .filter(|row| IS::get_index_value(row) == value),
            );
        } else {
            all.extend(optimistic_creations)
        }
        Ok(all)
    }
}

pub struct TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, TxnTableMarkers, Mode> {
    inner: TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, Mode>,
    optimistic_updates: Arc<OptimisticChanges>,
}

impl<'db, DbTableMarkers, TxnTableMarkers, Mode>
    TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, TxnTableMarkers, Mode>
where
    TxnTableMarkers: Default,
{
    pub fn with_store<H2>(
        self,
    ) -> TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, (H2::Marker, TxnTableMarkers), Mode>
    where
        H2: Store,
        DbTableMarkers: StoreMarker<H2>,
    {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.with_store::<H2>(),
            optimistic_updates: self.optimistic_updates,
        }
    }

    pub fn read_write(
        self,
    ) -> TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, TxnTableMarkers, ReadWrite> {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.read_write(),
            optimistic_updates: self.optimistic_updates,
        }
    }

    pub fn read_only(
        self,
    ) -> TxnBuilderWithOptimisticChanges<'db, DbTableMarkers, TxnTableMarkers, ReadOnly> {
        TxnBuilderWithOptimisticChanges {
            inner: self.inner.read_only(),
            optimistic_updates: self.optimistic_updates,
        }
    }

    pub fn with_no_commit_listener(self) -> Self {
        Self {
            inner: self.inner.with_no_commit_listener(),
            optimistic_updates: self.optimistic_updates,
        }
    }
}

impl<TxnTableMarkers, DbTableMarkers, Mode>
    TxnBuilderWithOptimisticChanges<'_, DbTableMarkers, TxnTableMarkers, Mode>
where
    Mode: TxnMode,
{
    pub fn build(self) -> TxnWithOptimisticChanges<TxnTableMarkers, Mode> {
        TxnWithOptimisticChanges {
            optimistic_updates: self.optimistic_updates.clone(),
            inner: self.inner.build(),
        }
    }
}

pub struct DbWithOptimisticChanges<StoreMarkers> {
    inner: TypesafeDb<StoreMarkers>,
    optimistic_updates: Arc<OptimisticChanges>,
}

impl<StoreMarkers> DbWithOptimisticChanges<StoreMarkers> {
    pub fn new(inner: TypesafeDb<StoreMarkers>) -> Self {
        Self {
            inner,
            optimistic_updates: Arc::new(Default::default()),
        }
    }
}

impl<DbStoreMarkers> DbWithOptimisticChanges<DbStoreMarkers> {
    pub fn txn(&self) -> TxnBuilderWithOptimisticChanges<'_, DbStoreMarkers, (), ReadOnly> {
        TxnBuilderWithOptimisticChanges {
            optimistic_updates: self.optimistic_updates.clone(),
            inner: Txn::builder(&self.inner),
        }
    }
}
