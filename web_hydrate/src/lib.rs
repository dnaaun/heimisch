use leptos::mount::hydrate_body;
use tracing_wasm::WASMLayerConfigBuilder;
use web::App;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
        WASMLayerConfigBuilder::new()
            .set_max_level(tracing::Level::DEBUG)
            .build(),
    );
    hydrate_body(App);
}
