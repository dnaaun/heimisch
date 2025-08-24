#![recursion_limit = "256"]
#![feature(type_alias_impl_trait)]
#![feature(marker_trait_attr)]
#![feature(async_fn_track_caller)]
#![feature(trait_alias)]

pub mod avail;
pub mod consts;
pub mod endpoints;
pub mod random;
pub mod retry;
pub mod sync_engine;
pub mod types;
pub mod utils;

pub mod backend_api_trait;
pub mod github_api_trait;
#[cfg(test)]
pub mod test_setup;
pub mod typed_db;
