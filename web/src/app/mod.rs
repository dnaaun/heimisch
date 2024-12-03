mod auth;
mod authenticated_home_page;
mod issues_tab;
mod not_found;
mod pull_requests_tab;
mod repository;
mod sync_engine_provider;
use auth::{Auth, USER_ACCESS_TOKEN_KEY};
use leptos_router::components::Routes;

use leptos::prelude::*;
use leptos_router::components::{Route, Router};
use leptos_router::path;
use not_found::NotFound;
use repository::RepositoryPage;
use sync_engine_provider::SyncEngineProvider;
use thaw::*;

use crate::local_storage::local_storage;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <SyncEngineProvider>
                <ConfigProvider>
                    <Router>
                        <Routes fallback=NotFound>
                            <Route path=path!("/auth") view=Auth />
                            <Route path=path!("/:owner_name/:repo_name") view=RepositoryPage />
                            <Route path=path!("/:owner_name/:repo_name/:tab") view=RepositoryPage />
                        </Routes>
                    </Router>
                </ConfigProvider>
            </SyncEngineProvider>
        </main>
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let login_dialog_open = RwSignal::new(false);
    Effect::new(move || {
        if local_storage()
            .get_item(USER_ACCESS_TOKEN_KEY)
            .expect("")
            .is_none()
        {
            login_dialog_open.set(true);
        }
    });

    view! { <LoginDialog open=login_dialog_open /> }
}

#[component]
pub fn LoginDialog(open: RwSignal<bool>) -> impl IntoView {
    view! {
        <Dialog open=open>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>"Dialog title"</DialogTitle>
                    <DialogContent>
                        <div>You need to login to Github!</div>
                    </DialogContent>
                    <DialogActions>
                        <Button appearance=ButtonAppearance::Primary on_click=|_| { todo!() }>
                            Click here to login
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>

        </Dialog>
    }
}
