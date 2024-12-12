mod auth;
mod authenticated_home_page;
pub mod error_component;
mod flowbite;
mod home;
mod icon;
mod issues_tab;
mod not_found;
mod pull_requests_tab;
mod repository;
mod sync_engine_provider;

use auth::Auth;
use home::{Home, Sidebar};
use leptos_router::components::{ParentRoute, Routes};

use leptos::prelude::*;
use leptos_router::components::{Route, Router};
use leptos_router::path;
use not_found::NotFound;
use repository::RepositoryPage;
use sync_engine_provider::SyncEngineProvider;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <SyncEngineProvider>
            <Router>
                <Routes fallback=NotFound>
                    <Route path=path!("/auth") view=Auth />
                    <ParentRoute path=path!("/") view=Sidebar >
                        <Route path=path!("") view=Home />
                        <Route path=path!(":owner_name/:repo_name") view=RepositoryPage />
                        <Route path=path!(":owner_name/:repo_name/:tab") view=RepositoryPage />
                    </ParentRoute>
                </Routes>
            </Router>
        </SyncEngineProvider>
    }
}
