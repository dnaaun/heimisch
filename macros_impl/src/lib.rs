#![feature(btree_set_entry)]

extern crate proc_macro;

mod avail_merge;
mod typed_router;
mod typesafe_idb;

pub use avail_merge::derive_avail_merge;
pub use typesafe_idb::derive_typesafe_idb;
