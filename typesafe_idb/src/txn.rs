use crate::object_store::ObjectStore;
use crate::Store;
use crate::{chain::Chain, StoreMarker, TypesafeDb};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

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

#[derive(Debug, Clone, Default)]
pub struct ReactivityTrackers {
    /// The value of the hasmap is the serde serialized value of the id.
    pub stores_accessed_by_id: HashMap<&'static str, HashSet<String>>,

    /// This will include get_all() accesses and also index accesses.
    /// It maybe good
    pub stores_accessed_in_bulk: HashSet<&'static str>,
}
impl ReactivityTrackers {
    pub fn overlaps(&self, other: &ReactivityTrackers) -> bool {
        self.stores_accessed_by_id
            .iter()
            .any(|(store_name_a, ids_a)| {
                other.stores_accessed_in_bulk.contains(store_name_a)
                    || other
                        .stores_accessed_by_id
                        .get(store_name_a)
                        .map(|ids_b| ids_a.intersection(ids_b).count() > 0)
                        .unwrap_or(false)
            })
            || self.stores_accessed_in_bulk.iter().any(|store_name_a| {
                other.stores_accessed_in_bulk.contains(store_name_a)
                    || other.stores_accessed_by_id.contains_key(store_name_a)
            })
    }
}

pub struct Txn<C, Mode> {
    #[allow(unused)]
    markers: C,
    /// RTI: Will be None if the transaction is committed or aborted.
    actual_txn: Option<idb::Transaction>,
    mode: PhantomData<Mode>,

    /// Could probably pass out &mut references istead of RefCell, but let's go for easy mode Rust.
    reactivity_trackers: RefCell<ReactivityTrackers>,

    commit_listener: Option<Rc<dyn Fn(&ReactivityTrackers)>>,
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
            reactivity_trackers: &self.reactivity_trackers,
            actual_object_store,
            _markers: PhantomData,
        })
    }

    pub async fn commit(mut self) -> Result<ReactivityTrackers, idb::Error> {
        commit_logic(self.actual_txn.take().expect("Should not be None if not committed/aborted before, which should ahve required a mut self"), &mut self.reactivity_trackers, &self.commit_listener)?;
        Ok(self.reactivity_trackers.take())
    }

    pub fn reactivity_trackers(&self) -> ReactivityTrackers {
        self.reactivity_trackers.borrow().clone()
    }

    pub async fn abort(mut self) -> Result<(), idb::Error> {
        self.actual_txn.take().expect("Should be None ony if it's committed/aborted, which means a &self shouldn't be unobtainable.")
            .abort()?;
        Ok(())
    }
}

pub struct TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, Mode> {
    db: &'db TypesafeDb<DbTableMarkers>,
    txn_table_markers: TxnTableMarkers,
    store_names: HashSet<&'static str>,
    commit_listener: Option<Rc<dyn Fn(&ReactivityTrackers)>>,
    mode: PhantomData<Mode>,
}

impl Txn<(), ()> {
    pub fn builder<DbTableMarkers>(
        db: &TypesafeDb<DbTableMarkers>,
    ) -> TxnBuilder<'_, DbTableMarkers, Chain<(), ()>, ReadOnly> {
        TxnBuilder {
            store_names: Default::default(),
            txn_table_markers: Chain::new(),
            db,
            commit_listener: db.listener.clone(),
            mode: PhantomData,
        }
    }
}

fn commit_logic(
    actual_txn: idb::Transaction,
    reactivity_trackers: &RefCell<ReactivityTrackers>,
    commit_listener: &Option<Rc<dyn Fn(&ReactivityTrackers)>>,
) -> Result<(), idb::Error> {
    let _ = actual_txn.commit()?;
    if let Some(listener) = commit_listener {
        listener(reactivity_trackers.borrow().deref());
    };
    Ok(())
}

impl<C, Mode> Drop for Txn<C, Mode> {
    fn drop(&mut self) {
        // If it's still Some(), means one hasn't called .commit() or .abort()
        if let Some(actual_txn) = self.actual_txn.take() {
            _ = commit_logic(
                actual_txn,
                &mut self.reactivity_trackers,
                &self.commit_listener,
            );
        }
    }
}

impl<'db, DbTableMarkers, TxnTableMarkers, Mode>
    TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, Mode>
{
    pub fn with_store<H2>(
        self,
    ) -> TxnBuilder<'db, DbTableMarkers, Chain<H2::Marker, TxnTableMarkers>, Mode>
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
            commit_listener: self.commit_listener,
            mode: self.mode,
        }
    }

    pub fn read_write(self) -> TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, ReadWrite> {
        TxnBuilder {
            txn_table_markers: self.txn_table_markers,
            store_names: self.store_names,
            db: self.db,
            commit_listener: self.commit_listener,
            mode: PhantomData,
        }
    }

    pub fn read_only(self) -> TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, ReadOnly> {
        TxnBuilder {
            txn_table_markers: self.txn_table_markers,
            store_names: self.store_names,
            db: self.db,
            commit_listener: self.commit_listener,
            mode: PhantomData,
        }
    }

    pub fn with_no_commit_listener(self) -> Self {
        Self {
            commit_listener: None,
            ..self
        }
    }
}

impl<'db, TxnTableMarkers, DbTableMarkers, Mode>
    TxnBuilder<'db, DbTableMarkers, TxnTableMarkers, Mode>
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
            reactivity_trackers: Default::default(),
            commit_listener: self.commit_listener,
        }
    }
}
