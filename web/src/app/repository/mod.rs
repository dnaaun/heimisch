use crate::signal_ext::*;
use leptos::{prelude::*, task::spawn_local};
use shared::{
    types::{
        self,
        repository::{Repository, RepositoryId},
        user::{self, User},
    },
    utils::LogErr,
};
use top_bar::TopBar;
use zwang_router::{set_pathname, Outlet, ParsedPath, RouteParams};
pub mod issues_tab;
pub mod pull_requests_tab;
mod top_bar;

use crate::{
    app::{
        installation_id_sync::use_sync_installation_ids_and_recv_websocket_updates,
        not_found::NotFound, routing::*,
    },
    frontend_error::FrontendError,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

use super::{
    flowbite::{Spinner, Tabs},
    sync_engine_provider::use_sync_engine,
};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum Tab {
    Issues,
    Pulls,
}

impl From<RootOwnerNameRepoName> for Tab {
    fn from(value: RootOwnerNameRepoName) -> Self {
        match value {
            RootOwnerNameRepoName::Pulls => Self::Pulls,
            RootOwnerNameRepoName::Empty => Self::Issues,
            RootOwnerNameRepoName::Issues(_) => Self::Issues,
        }
    }
}

impl Into<RootOwnerNameRepoName> for Tab {
    fn into(self) -> RootOwnerNameRepoName {
        match self {
            Tab::Issues => {
                RootOwnerNameRepoName::Issues(RootOwnerNameRepoNameIssues::Empty)
            }
            Tab::Pulls => RootOwnerNameRepoName::Pulls,
        }
    }
}

#[allow(non_snake_case)]
pub fn RepositoryPage(
    outlet: Outlet<Signal<RepositoryId>, impl IntoView + 'static>,
    params: RouteParams<ParamsOwnerNameRepoName>,
) -> impl IntoView {
    let parsed_path = use_context::<ParsedPath<Root>>().expect("");
    let active_tab = Memo::new(move |_| {
        Tab::from(match parsed_path.get() {
            Ok(Root::OwnerName {
                child:
                    RootOwnerName::RepoName {
                        child: child_parts, ..
                    },
                    ..
            }) => child_parts,
            _ => panic!("Component shouldn't be rendered if this isn't the path."),
        })
    });
    let new_active_tab = RwSignal::new(active_tab.get_untracked());

    Effect::new(move || {
        let active_tab = active_tab.get_untracked();
        let new_active_tab = new_active_tab.get();
        if active_tab == new_active_tab {
            return ();
        }

        set_pathname(Root::OwnerName {
            owner_name: params.owner_name.get_untracked(),
            child: RootOwnerName::RepoName {
                repo_name: params.repo_name.get_untracked(),
                child: new_active_tab.into(),
            },
        });
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
                .get(&params.owner_name.read())
                .await?;
            match user {
                Some(user) => {
                    let user_id = user.id;
                    let repos = txn
                        .object_store::<Repository>()?
                        .index::<types::repository::NameIndex>()?
                        .get_all(Some(&params.repo_name.read()))
                        .await?;

                    let repo = repos
                        .into_iter()
                        .find(|r| r.owner_id.map_ref(|o| o == &user_id).assume(false));

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
            view! { <div class="min-w-min h-screen">asdfasdf <Spinner /></div> }
        }>
            {move || {
                let repository_id = Signal::derive(move || repository_id.get());
                let repository_id = match repository_id.transpose() {
                    Some(r) => r.transpose().map_err(|s| s.get())?,
                    None => return Ok::<_, FrontendError>(None),
                };
                let repository_id = match repository_id.transpose() {
                    Some(r) => r,
                    None => {
                        return Ok(Some(view! { <NotFound /> }.into_any()));
                    }
                };
                let tabs = vec![Tab::Issues, Tab::Pulls];
                let get_tab_label = |key: &Tab| {
                    match key {
                        Tab::Issues => "Issues",
                        Tab::Pulls => "Pulls",
                    }
                        .to_owned()
                };
                Ok(
                    Some(
                        view! {
                            <TopBar repository_id owner_name=params.owner_name repo_name=params.repo_name />
                            <div>
                                <Tabs
                                    tabs
                                    active_tab=active_tab
                                    set_active_tab=move |t| *new_active_tab.write() = t
                                    get_tab_label
                                />
                                <div class="flex items-center justify-center">
                                    <div class="m-5 max-w-screen-xl w-full">
                                        {outlet.call(repository_id)}
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
