use std::{
    future::{pending, Future},
    ops::Deref,
    rc::Rc,
    sync::Arc,
};

use leptos::{prelude::*, task::spawn_local};
use leptos_router::{
    hooks::{use_navigate, use_params},
    params::Params,
};
use shared::{
    types::{
        self,
        repository::{Repository, RepositoryId},
        repository_initial_sync_status::{RepoSyncStatus, RepositoryInitialSyncStatus},
        user::{self, User},
    },
    utils::LogErr,
};
use top_bar::TopBar;
mod top_bar;

use crate::{
    app::not_found::NotFound, frontend_error::FrontendError,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

use super::{
    flowbite::{Spinner, Tab, Tabs},
    issues_tab::IssuesTab,
    pull_requests_tab::PullRequestsTab,
    sync_engine_provider::use_sync_engine,
};

#[derive(Params, PartialEq, Clone, Debug)]
struct RepositoryPageParams {
    owner_name: String,
    repo_name: String,
    tab: Option<String>,
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

/// Essentailly implemnets Result::map or Option::map equivalent for AsyncDerived.
fn map_unsync_async_derived<S, T1, Fut>(
    async_derived: AsyncDerived<T1, S>,
    func: impl Fn(T1) -> Fut + 'static,
) -> AsyncDerived<Fut::Output, LocalStorage>
where
    Fut: Future,
    Fut::Output: 'static,
    S: Storage<ArcAsyncDerived<T1>>,
    T1: Clone + 'static,
{
    let func = Rc::new(func);
    AsyncDerived::new_unsync(move || {
        let func = func.clone();
        async move {
            async_derived.ready().await;
            let value = async_derived
                .read()
                .deref()
                .clone()
                .expect("The ready() should take care of him.");
            func(value).await
        }
    })
}

/// Will return an AsyncDerived that resolves only when the RepositoryInitialSyncStatus
/// corresponding to the repository is Full. It will also resolve to an error if an error is
/// encountered.
fn use_repo_initial_sync_is_done<S>(
    id: AsyncDerived<Result<Option<RepositoryId>, FrontendError>, S>,
) -> AsyncDerived<Result<RepoSyncStatus, FrontendError>>
where
    S: Storage<ArcAsyncDerived<Result<Option<RepositoryId>, FrontendError>>>,
{
    let sync_engine = use_sync_engine().clone();
    let repo_initial_sync_is_done = AsyncDerived::new(pending);

    sync_engine.idb_signal(
        |db| db.txn().with_store::<RepositoryInitialSyncStatus>().ro(),
        move |txn| async move {
            let id = match id.read().clone() {
                Some(id) => id,
                None => return Ok(()),
            }?;

            let id = match id {
                Some(id) => id,
                None => return Ok(()),
            };
            let status = txn
                .object_store::<RepositoryInitialSyncStatus>()?
                .get(&id)
                .await?
                .map(|r| r.status);

            if let Some(status) = status {
                *repo_initial_sync_is_done.write() = Some(Ok(status));
            };
            Ok(())
        },
    );
    repo_initial_sync_is_done
}

/// RTI: URL params of shape `RepositoryPageParams`.
#[component]
pub fn RepositoryPage() -> impl IntoView {
    let params = || {
        use_params::<RepositoryPageParams>()
            .read()
            .clone()
            .expect("RepositoryPage should be mounted only if these params are available")
    };
    let params_untracked = || {
        use_params::<RepositoryPageParams>()
            .read_untracked()
            .clone()
            .expect("RepositoryPage should be mounted only if these params are available")
    };

    let active_tab = move || {
        params()
            .tab
            .as_ref()
            .and_then(|i| TabName::from_url_segment(i).ok())
            .unwrap_or(TabName::Issues)
    };
    let active_tab_str = Memo::new(move |_| active_tab().to_url_segment());
    let (new_active_tab_str, set_new_active_tab_str) = signal(active_tab_str());

    Effect::new(move || {
        let params_untracked = params_untracked();
        let new_active_tab = new_active_tab_str.read().clone();

        if active_tab_str.read_untracked() != new_active_tab {
            let navigate = use_navigate();
            navigate(
                &format!(
                    "/{}/{}/{}",
                    params_untracked.owner_name, params_untracked.repo_name, new_active_tab
                ),
                Default::default(),
            );
        }
    });

    let sync_engine = use_sync_engine();
    let repository = sync_engine.idb_signal(
        |db| {
            db.txn()
                .with_store::<User>()
                .with_store::<Repository>()
                .ro()
        },
        move |txn| {
            let RepositoryPageParams {
                owner_name,
                repo_name,
                ..
            } = params();
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

                        Ok(repo)
                    }
                    None => Ok(None),
                }
            }
        },
    );

    let repository_id =
        map_unsync_async_derived(repository.inner(), |r| async { r.map(|r| r.map(|r| r.id)) });
    Effect::new(move || {
        let sync_engine = sync_engine.clone();
        if let Some(Ok(Some(repository))) = repository.read().clone() {
            spawn_local(async move {
                let _ = sync_engine
                    .ensure_initial_sync_repository(&repository, false)
                    .await
                    .log_err();
            })
        };
    });

    let repo_initial_sync_is_done = use_repo_initial_sync_is_done(repository_id);

    view! {
        <Suspense fallback=|| {
            view! {
                <div class="min-w-min h-screen">
                    <Spinner />
                </div>
            }
        }>
            {move || {
                let repository = match repository.read().as_ref() {
                    Some(r) => r.as_ref()?,
                    None => return Ok::<_, FrontendError>(None),
                }
                    .clone();
                let repository = match repository {
                    Some(r) => r,
                    None => {
                        return Ok(
                            Some(
                                // The Option<> is to wait for the idb load.
                                // early return for idb error.

                                // The Option<> is to if repo is not found in idb.
                                view! { <NotFound /> }
                                    .into_any(),
                            ),
                        );
                    }
                };
                if let Some(Ok(RepoSyncStatus::NoSync)) = repo_initial_sync_is_done.read().clone() {
                    return Ok(None);
                }
                let tabs: Vec<Tab<_>> = vec![
                    Tab {
                        content_el: Arc::new(move || {

                            // Will trigger the fallback on the <Suspense> until repo initial sync is done.

                            view! { <IssuesTab repository=repository.clone() /> }
                                .into_any()
                        }),
                        key: TabName::Issues,
                    },
                    Tab {
                        content_el: Arc::new(move || {
                            view! { <PullRequestsTab _repository_id=42.into() /> }.into_any()
                        }),
                        key: TabName::Pulls,
                    },
                ];
                Ok(
                    Some(

                        view! {
                            <TopBar
                                owner_name=Memo::new(move |_| params().owner_name)
                                repo_name=Memo::new(move |_| params().repo_name)
                            />
                            <Tabs
                                tabs
                                active_tab=Signal::derive(active_tab)
                                set_active_tab=move |t| set_new_active_tab_str(t.to_url_segment())
                            />
                        }
                            .into_any(),
                    ),
                )
            }}
        </Suspense>
    }
}
