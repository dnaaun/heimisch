#![feature(marker_trait_attr)]
#![feature(type_alias_impl_trait)]

mod chain;
mod db;
mod error;
mod txn;
mod txn_valid_for_store;

pub mod serde_abstraction;
mod store;
mod object_store;
mod index;

pub use chain::Chain;
pub use db::{TypesafeDb, TypesafeDbBuilder};
pub use error::Error;
pub use store::Store;
pub use txn::{Present, ReadOnly, ReadWrite, Txn, TxnBuilder, TxnMode, ReactivityTrackers};
pub use txn_valid_for_store::StoreMarker;
pub use index::{Index, IndexSpec};
