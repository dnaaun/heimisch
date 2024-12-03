mod auth;
mod authenticated_home_page;
mod flowbite;
mod issues_tab;
mod not_found;
mod pull_requests_tab;
mod repository;
mod sync_engine_provider;

use auth::Auth;
use leptos_router::components::Routes;

use leptos::prelude::*;
use leptos_router::components::{Route, Router};
use leptos_router::path;
use not_found::NotFound;
use repository::RepositoryPage;
use sync_engine_provider::SyncEngineProvider;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <Router>
                <Routes fallback=NotFound>
                    <Route path=path!("/auth") view=Auth />
                    <Route path=path!("/:owner_name/:repo_name") view={ move || view! { <SyncEngineProvider><RepositoryPage /></SyncEngineProvider> } } />
                    <Route path=path!("/:owner_name/:repo_name/:tab") view={ move || view! { <SyncEngineProvider><RepositoryPage /></SyncEngineProvider> } } />
                </Routes>
            </Router>
        </main>
    }
}
