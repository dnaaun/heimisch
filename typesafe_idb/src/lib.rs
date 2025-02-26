#![feature(marker_trait_attr)]
#![feature(type_alias_impl_trait)]

mod db;
mod error;
mod txn;
mod txn_valid_for_store;

mod index;
mod object_store;
pub mod serde_abstraction;
mod store;

pub use db::{TypesafeDb, TypesafeDbBuilder};
pub use error::Error;
pub use index::{Index, IndexSpec};
pub use store::Store;
pub use txn::*;
pub use txn_valid_for_store::StoreMarker;
pub use object_store::ObjectStore;
