use std::{ops::Deref, sync::Arc};

use leptos::prelude::*;
use leptos_router::{
    hooks::{use_navigate, use_params},
    params::Params,
};
use shared::types::{
    self,
    repository::Repository,
    user::{self, User},
};
use top_bar::TopBar;
mod top_bar;

use crate::{app::not_found::NotFound, idb_signal_from_sync_engine::IdbSignalFromSyncEngine};

use super::{
    flowbite::{Tab, Tabs},
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
    let RepositoryPageParams {
        owner_name,
        repo_name,
        ..
    } = params();
    let repository = use_sync_engine().idb_signal(
        |db| {
            db.txn()
                .with_store::<User>()
                .with_store::<Repository>()
                .ro()
        },
        move |txn| {
            let owner_name = owner_name.clone();
            let repo_name = repo_name.clone();
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

    move || {
        let repository = repository.read();
        let repository = match repository.deref().deref() {
            Some(r) => r.as_ref().unwrap(),
            None => return view! { <div>Loading...</div> }.into_any(),
        }
        .clone();

        let repository = match repository {
            Some(r) => r,
            None => return view! { <NotFound /> }.into_any(),
        };

        let tabs: Vec<Tab<_>> = vec![
            Tab {
                content_el: Arc::new(move || {
                    view! { <IssuesTab repository=repository.clone() /> }.into_any()
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

        view! {
            <TopBar
                owner_name=Box::new(move || params().owner_name)
                repo_name=Box::new(move || params().repo_name)
            />
            <Tabs
                tabs
                active_tab=Signal::derive(active_tab)
                set_active_tab=move |t| set_new_active_tab_str(t.to_url_segment())
            />
        }
        .into_any()
    }
}
