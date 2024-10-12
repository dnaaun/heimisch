use crate::object_store::ObjectStore;
use crate::Store;
use crate::{chain::Chain, StoreMarker, TypesafeDb};
use std::cell::RefCell;
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

pub struct Present;
pub struct ReadWrite {}
pub struct ReadOnly {}

pub trait TxnMode {
    type SupportsReadOnly;
    type SupportsReadWrite;
    fn actual_mode() -> idb::TransactionMode;
}

impl TxnMode for ReadOnly {
    type SupportsReadOnly = Present;
    type SupportsReadWrite = ();
    fn actual_mode() -> idb::TransactionMode {
        idb::TransactionMode::ReadOnly
    }
}

impl TxnMode for ReadWrite {
    type SupportsReadOnly = Present;
    type SupportsReadWrite = Present;
    fn actual_mode() -> idb::TransactionMode {
        idb::TransactionMode::ReadWrite
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReactivityTrackers {
    /// The value of the hasmap is the serde serialized value of the id.
    pub stores_accessed_by_id: HashMap<&'static str, HashSet<String>>,

    /// This will include get_all() accesses and also index accesses.
    /// It maybe good
    pub stores_accessed_in_bulk: HashSet<&'static str>,
}

pub struct Txn<C, Mode> {
    #[allow(unused)]
    markers: C,
    /// RTI: Will be None if the transaction is committed or aborted.
    actual_txn: Option<idb::Transaction>,
    mode: PhantomData<Mode>,

    /// Could probably pass out &mut references istead of RefCell, but let's go for easy mode Rust.
    reactivity_trackers: Option<RefCell<ReactivityTrackers>>,
}

impl<Markers, Mode> Txn<Markers, Mode> {
    pub fn object_store<S>(&self) -> Result<ObjectStore<'_, S, Mode>, crate::Error>
    where
        S: Store,
        Markers: StoreMarker<S>,
    {
        let actual_object_store = self.actual_txn.as_ref().map(|t| t.object_store(S::NAME))
            .expect("Should be None ony if it's committed/aborted, which means a &self shouldn't be unobtainable.")?;

        Ok(ObjectStore {
            reactivity_trackers: self.reactivity_trackers.as_ref().expect("Should only be None only if it's committed, which means a &self should be unobtainable"),
            actual_object_store,
            _markers: PhantomData,
        })
    }

    pub async fn commit(mut self) -> Result<ReactivityTrackers, idb::Error> {
        self.actual_txn.take().expect("Should be None ony if it's committed/aborted, which means a &self shouldn't be unobtainable.")
            .commit()?;
        Ok(self.reactivity_trackers.take().expect("Should only be None only if it's committed, which means a &self should be unobtainable").into_inner())
    }

    pub fn reactivity_trackers(&self) -> ReactivityTrackers {
        self.reactivity_trackers
            .as_ref()
            .expect("Should only be None if ocmmitted, which means &self should be unbtainable")
            .borrow()
            .clone()
    }

    pub async fn abort(mut self) -> Result<(), idb::Error> {
        self.actual_txn.take().expect("Should be None ony if it's committed/aborted, which means a &self shouldn't be unobtainable.")
            .abort()?;
        Ok(())
    }
}

pub struct TxnBuilder<'db, TxnTableMarkers, DbTableMarkers> {
    db: &'db TypesafeDb<DbTableMarkers>,
    txn_table_markers: TxnTableMarkers,
    store_names: HashSet<&'static str>,
}

impl Txn<(), ()> {
    pub fn builder<'db, DbTableMarkers>(
        db: &'db TypesafeDb<DbTableMarkers>,
    ) -> TxnBuilder<'db, Chain<(), ()>, DbTableMarkers> {
        TxnBuilder {
            store_names: Default::default(),
            txn_table_markers: Chain::new(),
            db,
        }
    }
}

impl<C, Mode> Drop for Txn<C, Mode> {
    fn drop(&mut self) {
        if let Some(actual_txn) = self.actual_txn.take() {
            let _ = actual_txn.commit();
        }
    }
}

impl<'db, TxnTableMarkers, DbTableMarkers> TxnBuilder<'db, TxnTableMarkers, DbTableMarkers> {
    pub fn with_store<H2>(
        self,
    ) -> TxnBuilder<'db, Chain<H2::Marker, TxnTableMarkers>, DbTableMarkers>
    where
        H2: Store,
        DbTableMarkers: StoreMarker<H2>,
    {
        let new_markers = Chain::new::<H2::Marker, TxnTableMarkers>();
        let mut new_table_names = self.store_names;
        new_table_names.insert(H2::NAME);

        TxnBuilder {
            txn_table_markers: new_markers,
            store_names: new_table_names,
            db: self.db,
        }
    }

    /// Create a read write txn
    pub fn rw(self) -> Txn<TxnTableMarkers, ReadWrite> {
        let store_names = self.store_names.into_iter().collect::<Vec<_>>();
        Txn {
            markers: self.txn_table_markers,
            actual_txn: Some(
                match self
                    .db
                    .inner
                    .transaction(&store_names, ReadWrite::actual_mode())
                {
                    Ok(txn) => txn,
                    Err(err) => {
                        tracing::info!("existing store names: {:?}, trying to create a transaction for store names: {:?}",
                                self.db.inner.store_names(),
                                store_names,
                                );
                        Err(err).unwrap()
                    }
                },
            ),
            mode: PhantomData,
            reactivity_trackers: Some(Default::default()),
        }
    }

    /// Create a read only txn
    pub fn ro(self) -> Txn<TxnTableMarkers, ReadOnly> {
        let store_names = self.store_names.into_iter().collect::<Vec<_>>();
        Txn {
            markers: self.txn_table_markers,
            actual_txn: Some(
                match self
                    .db
                    .inner
                    .transaction(&store_names, ReadWrite::actual_mode())
                {
                    Ok(txn) => txn,
                    Err(err) => {
                        tracing::info!("existing store names: {:?}, trying to create a transaction for store names: {:?}",
                                self.db.inner.store_names(),
                                store_names,
                                );
                        Err(err).unwrap()
                    }
                },
            ),
            mode: PhantomData,
            reactivity_trackers: Some(Default::default()),
        }
    }
}
