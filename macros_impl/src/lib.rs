#![feature(result_flattening)]
#![feature(btree_set_entry)]

extern crate proc_macro;

mod avail_merge;
pub mod leptos_test_setup;
pub mod typed_db;
mod zwang_router;

pub use avail_merge::derive_avail_merge;
pub use typed_db::derive_table;
pub use zwang_router::zwang_routes;
pub use zwang_router::zwang_url;
