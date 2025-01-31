#![feature(result_flattening)]
#![feature(btree_set_entry)]

extern crate proc_macro;

mod avail_merge;
mod zwang_router;
mod typesafe_idb;
pub mod tracing_to_console_log;

pub use avail_merge::derive_avail_merge;
pub use typesafe_idb::derive_typesafe_idb;
pub use zwang_router::zwang_routes;
pub use zwang_router::zwang_url;
