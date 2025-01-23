#![feature(async_closure)]

use leptos::mount::mount_to_body;
use tracing_wasm::WASMLayerConfigBuilder;
pub mod app;
mod consts;
mod frontend_error;
mod idb_signal;
mod idb_signal_from_sync_engine;
mod local_storage;
pub mod signal_ext;
pub mod typed_transport;
mod use_unwrapped_context;

#[cfg(test)]
mod tests;
pub mod sync_engine_storage;

use app::App;

pub fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );
    mount_to_body(App);
}
