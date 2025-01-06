use leptos::{prelude::*, task::spawn_local};
use leptos_router::{
    components::Outlet,
    hooks::{use_location, use_navigate, use_params},
    params::Params,
};
use shared::{
    types::{
        self,
        repository::{Repository, RepositoryId},
        user::{self, User},
    },
    utils::LogErr,
};
use top_bar::TopBar;
pub mod issues_tab;
pub mod pull_requests_tab;
mod top_bar;

use crate::{
    app::{
        installation_id_sync::use_sync_installation_ids_and_recv_websocket_updates,
        not_found::NotFound,
    },
    frontend_error::FrontendError,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

use super::{
    flowbite::{Spinner, Tabs},
    sync_engine_provider::use_sync_engine,
};

#[derive(Params, PartialEq, Clone, Debug)]
struct RepositoryPageParams {
    owner_name: String,
    repo_name: String,
}

#[derive(PartialEq, Clone, Eq, Hash, Debug)]
enum TabName {
    Issues,
    Pulls,
}

impl TabName {
    fn to_url_segment(&self) -> String {
        match self {
            TabName::Issues => "issues",
            TabName::Pulls => "pulls",
        }
        .into()
    }

    fn from_url_segment(segment: &str) -> Result<Self, ()> {
        match segment.to_lowercase().as_str() {
            "issues" => Ok(TabName::Issues),
            "pulls" => Ok(TabName::Pulls),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TabName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TabName::Issues => "Issues",
            TabName::Pulls => "Pull Requests",
        })
    }
}

#[derive(Clone)]
pub struct RepositoryPageParentRouteContext(Signal<RepositoryId>);

#[component]
pub fn RepositoryPage() -> impl IntoView {
    let params = Signal::derive(|| {
        use_params::<RepositoryPageParams>()
            .read()
            .clone()
            .expect("RepositoryPage should be mounted only if these params are available")
    });

    let active_tab = Signal::derive(|| {
        use_location()
            .pathname
            .get()
            .split("/")
            .last()
            .map(TabName::from_url_segment)
            .transpose()
            .ok()
            .flatten()
            .unwrap_or(TabName::Issues)
    });
    let new_active_tab = RwSignal::new(active_tab.get_untracked());
    let navigate = use_navigate();

    Effect::new(move || {
        let params_untracked = params.get_untracked();
        let active_tab = active_tab.get_untracked();

        let new_active_tab = new_active_tab.get();
        if active_tab != new_active_tab {
            navigate(
                &format!(
                    "/{}/{}/{}",
                    params_untracked.owner_name,
                    params_untracked.repo_name,
                    new_active_tab.to_url_segment()
                ),
                Default::default(),
            )
        }
    });

    use_sync_installation_ids_and_recv_websocket_updates();

    let sync_engine = use_sync_engine();

    let repository_id = sync_engine.idb_signal(
        |builder| {
            builder
                .with_store::<User>()
                .with_store::<Repository>()
                .build()
        },
        move |txn| {
            let RepositoryPageParams {
                owner_name,
                repo_name,
            } = params.get();
            async move {
                let user = txn
                    .object_store::<User>()?
                    .index::<user::LoginIndex>()?
                    .get(&owner_name)
                    .await?;
                match user {
                    Some(user) => {
                        let user_id = user.id;
                        let repo = txn
                            .object_store::<Repository>()?
                            .index::<types::repository::NameIndex>()?
                            .get_all(Some(&repo_name))
                            .await?
                            .into_iter()
                            .find(|r| r.owner_id.map_ref(|o| o == &user_id).unwrap_or(false));

                        Ok(repo.map(|r| r.id))
                    }
                    None => Ok(None),
                }
            }
        },
    );

    // Memo is necessary to make sure effect runs once for each repo
    let repository_id = Memo::new(move |_| repository_id.get());

    Effect::new(move || {
        let sync_engine = sync_engine.clone();
        if let Some(Ok(Some(repository_id))) = repository_id.get() {
            spawn_local(async move {
                let _ = sync_engine
                    .ensure_initial_sync_repository(&repository_id, false)
                    .await
                    .log_err();
            })
        };
    });

    view! {
        <Transition fallback=|| {
            view! {
                <div class="min-w-min h-screen">
                    <Spinner />
                </div>
            }
        }>
            {move || {
                let repository_id = match repository_id.get() {
                    Some(r) => r?,
                    None => return Ok::<_, FrontendError>(None),
                };
                let repository_id = match repository_id {
                    Some(r) => r,
                    None => {
                        return Ok(Some(view! { <NotFound /> }.into_any()));
                    }
                };
                provide_context(RepositoryPageParentRouteContext(repository_id.into()));
                let tabs = vec![TabName::Issues, TabName::Pulls];
                Ok(
                    Some(

                        view! {
                            <TopBar
                                repository_id
                                owner_name=Memo::new(move |_| params().owner_name)
                                repo_name=Memo::new(move |_| params().repo_name)
                            />
                            <div>
                                <Tabs
                                    tabs
                                    active_tab=Signal::derive(active_tab)
                                    set_active_tab=move |t| *new_active_tab.write() = t
                                />
                                <div class="flex items-center justify-center">
                                    <div class="m-5 max-w-screen-xl w-full">
                                        <Outlet />
                                    </div>
                                </div>
                            </div>
                        }
                            .into_any(),
                    ),
                )
            }}
        </Transition>
    }
}
