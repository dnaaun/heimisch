mod auth;
mod authenticated_home_page;
pub mod error_component;
mod flowbite;
mod home;
mod icon;
mod not_found;
mod repository;
mod sync_engine_provider;

use std::rc::Rc;

use auth::Auth;
use home::{Home, Sidebar};
use leptos_router::components::{ParentRoute, Routes};

use leptos::prelude::*;
use leptos_router::components::{Route, Router};
use leptos_router::path;
use not_found::NotFound;
use repository::issues_tab::IssuesTab;
use repository::pull_requests_tab::PullRequestsTab;
use repository::RepositoryPage;
use shared::utils::LogErr;
use sync_engine_provider::sync_engine_provided;

pub use leptos_router;
use wasm_bindgen_futures::spawn_local;

use crate::consts::ENDPOINT_CLIENT;
use crate::typed_websocket_client::TypedWebsocketClient;

#[component]
pub fn App() -> impl IntoView {
    let sync_engine = LocalResource::new(move || async move {
        Rc::new(
            shared::sync_engine::SyncEngine::<TypedWebsocketClient>::new(
                ENDPOINT_CLIENT.with(|e| e.clone()),
            )
            .await
            .unwrap(),
        )
    });

    Effect::new(move || {
        if let Some(sync_engine) = sync_engine.get() {
            spawn_local(async move {
                let _ = sync_engine.recv_websocket_updates().await.log_err();
            })
        }
    });

    view! {
        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/auth") view=sync_engine_provided(Auth, sync_engine) />
                <ParentRoute path=path!("/") view=sync_engine_provided(Sidebar, sync_engine)>
                    <Route path=path!("") view=sync_engine_provided(Home, sync_engine) />
                    <ParentRoute
                        path=path!(":owner_name/:repo_name")
                        view=sync_engine_provided(RepositoryPage, sync_engine)
                    >
                        <Route
                            path=path!("/")
                            view=sync_engine_provided(IssuesTab, sync_engine)
                        />
                        <Route
                            path=path!("/issues")
                            view=sync_engine_provided(IssuesTab, sync_engine)
                        />
                        <Route
                            path=path!("/pulls")
                            view=sync_engine_provided(PullRequestsTab, sync_engine)
                        />
                    </ParentRoute>
                    <Route
                        path=path!(":owner_name/:repo_name/:tab")
                        view=sync_engine_provided(RepositoryPage, sync_engine)
                    />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
