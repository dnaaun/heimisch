mod auth;
mod authenticated_home_page;
mod flowbite;
mod home;
mod icon;
mod installation_id_sync;
mod not_found;
mod repository;
pub mod routing;
pub mod sidebar;
pub mod sync_engine_provider;
pub mod thirds;

use crate::app::sync_engine_provider::sync_engine_provided;
use std::rc::Rc;

use leptos::prelude::*;

use routing::Routed;
use sync_engine_provider::SyncEngine;

use crate::consts::ENDPOINT_CLIENT;

#[component]
pub fn App() -> impl IntoView {
    let sync_engine = LocalResource::new(move || async move {
        Rc::new(
            SyncEngine::new(ENDPOINT_CLIENT.with(|e| e.clone()))
                .await
                .unwrap(),
        )
    });

    let Routed = sync_engine_provided(Routed, sync_engine);
    Routed()
}
