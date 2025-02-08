use crate::object_store::ObjectStore;
use crate::{Error, Store};
use crate::{StoreMarker, TypesafeDb};
use std::ops::Deref;
use std::{collections::HashSet, marker::PhantomData};

#[derive(Clone)]
pub struct Present;
#[derive(Clone)]
pub struct ReadWrite {}
#[derive(Clone)]
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct StoreName(pub &'static str);

impl Deref for StoreName {
    type Target = &'static str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Txn<C, Mode> {
    #[allow(unused)]
    markers: C,
    /// RTI: Will be None if and only if the transaction is committed or aborted.
    actual_txn: Option<idb::Transaction>,
    mode: PhantomData<Mode>,
}

impl<Markers, Mode> Txn<Markers, Mode> {
    pub fn object_store<S>(&self) -> Result<ObjectStore<S, Mode>, crate::Error>
    where
        S: Store,
        Markers: StoreMarker<S>,
    {
        let actual_object_store = self.actual_txn.as_ref().map(|t| t.object_store(&S::NAME))
            .expect("Should be None ony if it's committed/aborted, which means a &self shouldn't be unobtainable.")?;

        Ok(ObjectStore {
            actual_object_store,
            _markers: PhantomData,
        })
    }

    pub fn commit(mut self) -> Result<(), Error> {
        if let Some(actual_txn) = self.actual_txn.take() {
            actual_txn.commit()?;
        }

        Ok(())
    }

    pub fn abort(mut self) -> Result<(), Error> {
        self.actual_txn
            .take()
            .expect("Should be None ony if it's committed/aborted, which means a &self shouldn't be unobtainable.")
            .abort()?;
        Ok(())
    }
}

pub struct TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, Mode> {
    db: &'db TypesafeDb<DbTableMarkers>,
    txn_table_markers: TxnTableMarkers,
    store_names: HashSet<&'static str>,
    mode: PhantomData<Mode>,
}

impl Txn<(), ()> {
    pub fn builder<DbTableMarkers>(
        db: &TypesafeDb<DbTableMarkers>,
    ) -> TxnBuilder<'_, DbTableMarkers, (), ReadOnly> {
        TxnBuilder {
            store_names: Default::default(),
            txn_table_markers: Default::default(),
            db,
            mode: PhantomData,
        }
    }
}

impl<C, Mode> Drop for Txn<C, Mode> {
    fn drop(&mut self) {
        // If it's still Some(), means one hasn't called .commit() or .abort()
        if let Some(actual_txn) = self.actual_txn.take() {
            actual_txn.commit().expect("Couldnt' commit indexeddb txn.");
        }
    }
}

impl<'db, DbTableMarkers, TxnTableMarkers, Mode>
    TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, Mode>
where
    TxnTableMarkers: Default,
{
    pub fn with_store<H2>(
        self,
    ) -> TxnBuilder<'db, DbTableMarkers, (H2::Marker, TxnTableMarkers), Mode>
    where
        H2: Store,
        DbTableMarkers: StoreMarker<H2>,
    {
        let new_markers = (H2::Marker::default(), TxnTableMarkers::default());
        let mut new_table_names = self.store_names;
        new_table_names.insert(&H2::NAME);

        TxnBuilder {
            txn_table_markers: new_markers,
            store_names: new_table_names,
            db: self.db,
            mode: self.mode,
        }
    }

    pub fn read_write(self) -> TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, ReadWrite> {
        TxnBuilder {
            txn_table_markers: self.txn_table_markers,
            store_names: self.store_names,
            db: self.db,
            mode: PhantomData,
        }
    }

    pub fn read_only(self) -> TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, ReadOnly> {
        TxnBuilder {
            txn_table_markers: self.txn_table_markers,
            store_names: self.store_names,
            db: self.db,
            mode: PhantomData,
        }
    }
}

impl<TxnTableMarkers, DbTableMarkers, Mode> TxnBuilder<'_, DbTableMarkers, TxnTableMarkers, Mode>
where
    Mode: TxnMode,
{
    pub fn build(self) -> Txn<TxnTableMarkers, Mode> {
        let store_names = self.store_names.into_iter().collect::<Vec<_>>();
        Txn {
            markers: self.txn_table_markers,
            actual_txn: Some(
                self.db
                    .inner
                    .transaction(&store_names, Mode::actual_mode())
                    .expect(""),
            ),
            mode: PhantomData,
        }
    }
}
