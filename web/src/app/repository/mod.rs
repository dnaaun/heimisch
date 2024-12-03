use std::{ops::Deref, str::FromStr};

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
use thaw::*;

use crate::{app::not_found::NotFound, idb_signal_from_sync_engine::IdbSignalFromSyncEngine};

use super::{
    issues_tab::IssuesTab, pull_requests_tab::PullRequestsTab,
    sync_engine_provider::use_sync_engine,
};

#[derive(Params, PartialEq, Clone)]
struct RepositoryPageParams {
    owner_name: String,
    repo_name: String,
    tab: Option<String>,
}

#[derive(strum_macros::EnumString, strum_macros::Display, Clone, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum TabName {
    #[strum(to_string = "issues")]
    Issues,
    #[strum(to_string = "pulls")]
    Pulls,
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

    let active_tab_enum = Memo::new(move |_| {
        params()
            .tab
            .as_ref()
            .and_then(|i| TabName::from_str(i).ok())
            .unwrap_or(TabName::Issues)
    });
    let active_tab = Memo::new(move |_| active_tab_enum.read().to_string());

    let (new_active_tab, active_tab_setter) = signal(active_tab());

    Effect::new(move || {
        let params_untracked = params_untracked();
        let new_active_tab = new_active_tab.read().clone();
        if active_tab.read_untracked() != new_active_tab {
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
            Box::pin(async move {
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

                        Ok::<_, typesafe_idb::Error>(repo)
                    }
                    None => Ok(None),
                }
            })
        },
    );

    move || {
        tracing::info!("HEY");
        let repository = repository.read();
        let repository = match repository.deref().deref() {
            Some(r) => r.as_ref().unwrap(),
            None => return view! { <div>Loading...</div> }.into_any(),
        };

        let repository = match repository {
            Some(r) => r,
            None => return view! { <NotFound /> }.into_any(),
        };

        let active_tab_el = match *active_tab_enum.read() {
            TabName::Issues => view! { <IssuesTab repository=repository.clone() /> }.into_any(),
            TabName::Pulls => view! { <PullRequestsTab _repository_id=42.into() /> }.into_any(),
        };

        view! {
            <TopBar
                owner_name=Box::new(move || params().owner_name)
                repo_name=Box::new(move || params().repo_name)
            />
            <div class="pl-6">
                <TabList selected_value=(active_tab, active_tab_setter)>
                    <Tab value=TabName::Pulls.to_string()>Pull Requests</Tab>
                    <Tab value=TabName::Issues.to_string()>Issues</Tab>
                </TabList>
            </div>
            <div class="border-t border-t-gray-200">
                <div class="w-full max-w-screen-xl mx-auto py-4 px-6">{active_tab_el}</div>
            </div>
        }
        .into_any()
    }
}
