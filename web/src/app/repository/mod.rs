
use leptos::{prelude::*, task::spawn_local};
use leptos_router::params::Params;
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
        routing::{self, set_pathname},
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

#[component]
pub fn RepositoryPage(
    #[prop(into)] child_component: Signal<Box<dyn Fn(RepositoryId) -> AnyView + Send + Sync>>,
    #[prop(into)] path_so_far: Signal<routing::TopLevelEmptyOwnerName>,
) -> impl IntoView {
    let owner_name = Memo::new(move |_| path_so_far.get().captured.clone());
    let repo_name = Memo::new(move |_| path_so_far.get().child.captured.clone());
    let active_tab = Memo::new(move |_| path_so_far.get().child.child);
    let new_active_tab = RwSignal::new(active_tab.get_untracked());

    Effect::new(move || {
        let active_tab = active_tab.get_untracked();
        let new_active_tab = new_active_tab.get();
        if active_tab != new_active_tab {
            set_pathname(routing::TopLevel::Empty(routing::TopLevelEmpty::OwnerName(
                routing::TopLevelEmptyOwnerName {
                    captured: owner_name.get_untracked(),
                    child: routing::TopLevelEmptyOwnerNameRepoName {
                        captured: repo_name.get_untracked(),
                        child: new_active_tab,
                    },
                },
            )))
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
        move |txn| async move {
            let user = txn
                .object_store::<User>()?
                .index::<user::LoginIndex>()?
                .get(&owner_name.read())
                .await?;
            match user {
                Some(user) => {
                    let user_id = user.id;
                    let repos = txn
                        .object_store::<Repository>()?
                        .index::<types::repository::NameIndex>()?
                        .get_all(Some(&repo_name.read()))
                        .await?;

                    let repo = repos
                        .into_iter()
                        .find(|r| r.owner_id.map_ref(|o| o == &user_id).unwrap_or(false));

                    Ok(repo.map(|r| r.id))
                }
                None => Ok(None),
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
                let tabs = vec![
                    routing::TopLevelEmptyOwnerNameRepoNameChild::Issues,
                    routing::TopLevelEmptyOwnerNameRepoNameChild::Pulls,
                ];
                Ok(
                    Some(

                        view! {
                            <TopBar repository_id owner_name repo_name />
                            <div>
                                <Tabs
                                    tabs
                                    active_tab=Signal::derive(active_tab)
                                    set_active_tab=move |t| *new_active_tab.write() = t
                                />
                                <div class="flex items-center justify-center">
                                    <div class="m-5 max-w-screen-xl w-full">
                                        {child_component.read()(repository_id)}
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
