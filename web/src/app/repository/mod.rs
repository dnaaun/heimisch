use crate::signal_ext::*;
use leptos::{prelude::*, task::spawn_local};
use shared::{
    sync_engine::optimistic::db::MaybeOptimistic,
    types::{
        self,
        repository::Repository,
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
    app::{not_found::NotFound, routing::*},
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
            RootOwnerNameRepoName::Pulls(_) => Self::Pulls,
            RootOwnerNameRepoName::Empty => Self::Issues,
            RootOwnerNameRepoName::Issues(_) => Self::Issues,
        }
    }
}

impl From<Tab> for RootOwnerNameRepoName {
    fn from(val: Tab) -> Self {
        match val {
            Tab::Issues => RootOwnerNameRepoName::Issues(RootOwnerNameRepoNameIssues::Empty),
            Tab::Pulls => RootOwnerNameRepoName::Pulls(RootOwnerNameRepoNamePulls::Empty),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepositoryPageContextInner {
    repository: MaybeOptimistic<Repository>,
    user: MaybeOptimistic<User>,
}

pub type RepositoryPageContext = Memo<RepositoryPageContextInner>;

#[allow(non_snake_case)]
pub fn RepositoryPage(
    outlet: Outlet<RepositoryPageContext, impl IntoView + 'static>,
    RouteParams(params): RouteParams<ParamsOwnerNameRepoName>,
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

    let ParamsOwnerNameRepoName {
        owner_name,
        repo_name,
    } = params;

    let set_active_tab = move |new_active_tab: Tab| {
        set_pathname(&Root::OwnerName {
            owner_name: owner_name.get_untracked(),
            child: RootOwnerName::RepoName {
                repo_name: repo_name.get_untracked(),
                child: new_active_tab.into(),
            },
        });
    };

    let sync_engine = use_sync_engine();

    let repository_page_context =
        sync_engine.idb_signal(
            async |builder| {
                builder
                    .with_table::<User>()
                    .with_table::<Repository>()
                    .build()
                    .await
            },
            move |txn| async move {
                let user = txn
                    .table::<User>()
                    .index::<user::LoginIndex>()
                    .get_optimistically(&params.owner_name.read())
                    .await?;
                match user {
                    Some(user) => {
                        let user_id = user.id;
                        let repos = txn
                            .table::<Repository>()
                            .index::<types::repository::NameIndex>()
                            .get_all_optimistically(Some(&params.repo_name.read()))
                            .await?;

                        let repository = repos
                            .into_iter()
                            .find(|r| r.owner_id.map_ref(|o| o == &user_id).assume(false));

                        Ok(repository
                            .map(|repository| RepositoryPageContextInner { repository, user }))
                    }
                    None => Ok(None),
                }
            },
        );

    // Memo is necessary to make sure effect runs once for each repo
    let repository_page_context = Memo::new(move |_| repository_page_context.get());

    Effect::new(move || {
        let sync_engine = sync_engine.clone();
        if let Some(Ok(Some(context))) = repository_page_context.get() {
            spawn_local(async move {
                let _ = sync_engine
                    .ensure_initial_sync_repository(&context.repository.id, false)
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
                let repository_page_context = match repository_page_context.transpose() {
                    Some(r) => r.transpose().map_err(|s| s.get())?,
                    None => return Ok::<_, FrontendError>(None),
                };
                let repository_page_context = match repository_page_context.transpose() {
                    Some(r) => r,
                    None => {
                        return Ok(Some(view! { <NotFound /> }.into_any()));
                    }
                };
                let repository_id = Signal::derive(move || repository_page_context.get().repository.id);
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
                            <TopBar
                                repository_id
                                owner_name=params.owner_name
                                repo_name=params.repo_name
                            />
                            <div>
                                <Tabs
                                    tabs
                                    active_tab=active_tab
                                    set_active_tab=set_active_tab
                                    get_tab_label
                                />
                                <div class="flex items-center justify-center">
                                    <div class="m-5 max-w-screen-xl w-full">
                                        {outlet.call(repository_page_context)}
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
