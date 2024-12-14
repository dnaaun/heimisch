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
use sync_engine_provider::sync_engine_provided;

pub use leptos_router;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=NotFound>
                <Route path=path!("/auth") view=sync_engine_provided(Auth) />
                <ParentRoute path=path!("/") view=sync_engine_provided(Sidebar)>
                    <Route path=path!("") view=sync_engine_provided(Home) />
                    <Route
                        path=path!(":owner_name/:repo_name")
                        view=sync_engine_provided(RepositoryPage)
                    />
                    <Route
                        path=path!(":owner_name/:repo_name/:tab")
                        view=sync_engine_provided(RepositoryPage)
                    />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
