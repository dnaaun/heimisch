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

use crate::{app::sync_engine_provider::sync_engine_provided, consts::BACKEND_API};
use std::rc::Rc;

use leptos::prelude::*;

use routing::Routed;
use shared::sync_engine::Transport;
use sync_engine_provider::SyncEngine;

#[component]
pub fn App() -> impl IntoView {
    let sync_engine = LocalResource::new(move || async move {
        Rc::new(
            SyncEngine::new(
                Rc::new(BACKEND_API.with(|e| e.clone())),
                |url| async { Transport::new(url).await },
                Rc::new(shared::github_api_trait::GithubApi),
            )
            .await
            .unwrap(),
        )
    });

    let Routed = sync_engine_provided(Routed, sync_engine);
    Routed()
}
