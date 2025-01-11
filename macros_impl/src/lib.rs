extern crate proc_macro;

mod avail_merge;
mod typesafe_idb;
mod typed_router;

pub use avail_merge::derive_avail_merge;
pub use typesafe_idb::derive_typesafe_idb;
