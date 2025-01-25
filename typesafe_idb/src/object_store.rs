use std::{cell::RefCell, marker::PhantomData};

use crate::{
    serde_abstraction, Index, IndexSpec, Present, ReactivityTrackers, SerializedId, Store, TxnMode,
};

#[derive(Clone)]
pub struct ObjectStore<'txn, Store, Mode> {
    pub(crate) reactivity_trackers: &'txn RefCell<ReactivityTrackers>,
    pub(crate) actual_object_store: idb::ObjectStore,

    pub(crate) _markers: PhantomData<(Store, Mode)>,
}

impl<S, Mode> ObjectStore<'_, S, Mode>
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadOnly = Present>,
{
    pub async fn get(&self, id: &S::Id) -> Result<Option<S>, crate::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_by_id_access(S::NAME, SerializedId::new_from_id::<S>(id));

        Ok(self
            .actual_object_store
            .get(idb::Query::Key(serde_abstraction::to_value(id)?))?
            .await?
            .map(|i| serde_abstraction::from_value(i))
            .transpose()?)
    }

    pub async fn get_all(&self) -> Result<Vec<S>, crate::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_bulk_access(S::NAME);
        Ok(self
            .actual_object_store
            .get_all(None, None)?
            .await?
            .into_iter()
            .map(|i| serde_abstraction::from_value(i))
            .collect::<Result<Vec<_>, _>>()?)
    }

    pub fn index<IS: IndexSpec<Store = S>>(&self) -> Result<Index<'_, IS>, crate::Error> {
        Ok(Index {
            reactivity_trackers: self.reactivity_trackers,
            actual_index: self.actual_object_store.index(&IS::NAME)?,
            _markers: PhantomData,
        })
    }
}

impl<S, Mode> ObjectStore<'_, S, Mode>
where
    S: Store + 'static,
    Mode: TxnMode<SupportsReadWrite = Present>,
{
    /// Only mut for consistency with the read-only methods.
    pub async fn delete(&self, id: &S::Id) -> Result<(), crate::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_by_id_access(S::NAME, SerializedId::new_from_id::<S>(id));

        Ok(self
            .actual_object_store
            .delete(idb::Query::Key(serde_abstraction::to_value(&id)?))?
            .await?)
    }

    /// Only mut for consistency with the read-only methods.
    pub async fn put(&self, item: &S) -> Result<(), crate::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_by_id_access(S::NAME, SerializedId::new_from_row(item));

        self.actual_object_store
            .put(&serde_abstraction::to_value(item)?, None)?
            .await?;
        Ok(())
    }
}
