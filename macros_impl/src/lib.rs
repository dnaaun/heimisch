#![feature(result_flattening)]
#![feature(btree_set_entry)]

extern crate proc_macro;

mod avail_merge;
pub mod leptos_test_setup;
pub mod table;
mod typesafe_idb;
mod zwang_router;

pub use avail_merge::derive_avail_merge;
pub use typesafe_idb::derive_typesafe_idb;
pub use zwang_router::zwang_routes;
pub use zwang_router::zwang_url;
