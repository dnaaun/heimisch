use std::{cell::RefCell, marker::PhantomData};

use idb::Query;
use serde::Serialize;

use crate::{
    serde_abstraction::{from_value, to_value},
    ReactivityTrackers, Store, StoreName,
};

pub trait IndexSpec {
    type Store: Store;
    const NAME: &'static str;
    type Type: Serialize;
}

pub struct Index<'txn, IS> {
    pub(crate) reactivity_trackers: &'txn RefCell<ReactivityTrackers>,
    pub(crate) actual_index: idb::Index,
    pub(crate) _markers: PhantomData<IS>,
}

impl<IS: IndexSpec> Index<'_, IS> {
    pub async fn get(&self, value: &IS::Type) -> Result<Option<IS::Store>, crate::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_bulk_access(StoreName(IS::Store::NAME));

        Ok(self
            .actual_index
            .get(Query::Key(to_value(value)?))?
            .await?
            .map(from_value)
            .transpose()?)
    }

    pub async fn get_all(&self, value: Option<&IS::Type>) -> Result<Vec<IS::Store>, crate::Error> {
        self.reactivity_trackers
            .borrow_mut()
            .add_bulk_access(StoreName(IS::Store::NAME));

        Ok(self
            .actual_index
            .get_all(
                value.map(|v| to_value(v).map(Query::Key)).transpose()?,
                None,
            )?
            .await?
            .into_iter()
            .map(from_value)
            .collect::<Result<Vec<_>, _>>()?)
    }
}
