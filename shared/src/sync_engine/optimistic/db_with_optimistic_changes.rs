use std::sync::Arc;
use typesafe_idb::{Error as IdbError, ReactivityTrackers};
use idb::{Error as IdbError, TxnMode, Present};

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

    pub async fn commit(self) -> Result<ReactivityTrackers, IdbError> {
        self.inner.commit().await
     }

    pub fn reactivity_trackers(&self) -> ReactivityTrackers {
        self.inner.reactivity_trackers()
     }

    pub async fn abort(self) -> Result<(), IdbError> {
        self.inner.abort().await
     }
}

pub struct ObjectStoreWithOptimisticChanges<'a, S, Mode> 
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadWrite = Present>,
{
    optimistic_changes: Arc<OptimisticChanges>,
    inner: Index<'a, IS>,
}

impl<S, Mode> ObjectStoreWithOptimisticChanges<'_, S, Mode>
where
    S: Store +  'static,
    Mode: TxnMode<SupportsReadWrite = Present>,
{
    pub async fn delete(&self, id: &S::Id) -> Result<(), IdbError> {
        self.inner.delete(id).await?;
        self.optimistic_changes.remove_obsoletes_for_id::<S>(id);
        Ok(())
     }

    pub async fn put(&self, item: &S) -> Result<(), IdbError> {
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
    pub async fn get(&self, value: &IS::Type) -> Result<Option<IS::Store>, typesafe_idb::Error> {
        todo!()
     }

    pub async fn get_all(
         &self,
        value: Option<&IS::Type>,
     ) -> Result<Vec<IS::Store>, typesafe_idb::Error> {
        todo!()
     }
}
