#![feature(type_alias_impl_trait)]

pub mod app;
mod consts;
mod idb_signal;
mod idb_signal_from_sync_engine;
mod local_storage;
mod use_unwrapped_context;
mod utils;
mod websocket_updates;
mod frontend_error;
use hydration::{AutoReload, HydrationScripts};
use leptos::prelude::*;
use leptos::{config::LeptosOptions, *};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <link rel="stylesheet" id="leptos" href="/pkg/heimisch.css" />
            </head>
            <body>
                <App />
            </body>
        </html>
    }

}

pub use app::App;
