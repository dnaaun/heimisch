#![recursion_limit = "256"]
#![feature(type_alias_impl_trait)]
#![feature(marker_trait_attr)]
#![feature(async_fn_track_caller)]
#![feature(trait_alias)]

pub mod avail;
pub mod endpoints;
pub mod sync_engine;
pub mod types;
pub mod utils;
pub mod retry;
pub mod consts;
pub mod random;

#[cfg(test)]
pub mod test_setup;
pub mod github_api_trait;
pub mod backend_api_trait;
