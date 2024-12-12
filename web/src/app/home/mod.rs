use futures::future::{join_all, OptionFuture};
use itertools::Itertools;
use leptos::prelude::*;
use shared::types::{repository::Repository, user::User};

use crate::{
    app::sync_engine_provider::use_sync_engine, frontend_error::FrontendError,
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

use super::icon::Icon;
use icondata;

#[component]
pub fn Home() -> impl IntoView {}

#[component]
pub fn Sidebar<T>(children: View<T>) -> impl IntoView
where
    T: Send + 'static,
    View<T>: Render + RenderHtml,
{
    let sync_engine = use_sync_engine();
    let repositorys_by_owner = sync_engine.idb_signal(
        |db| {
            db.txn()
                .with_store::<Repository>()
                .with_store::<User>()
                .ro()
        },
        |txn| async move {
            let repositorys = txn.object_store::<Repository>()?.get_all().await?;
            let user_store = txn.object_store::<User>()?;
            let users = join_all(repositorys.iter().map(|r| {
                OptionFuture::from(r.owner_id.clone().to_option().map(|user_id| {
                    let user_store = user_store.clone();
                    async move { user_store.get(&user_id).await }
                }))
            }))
            .await
            .into_iter()
            .map(|item| item.transpose().map(|inner_item| inner_item.flatten()))
            .collect::<Result<Vec<_>, _>>()?;

            Ok(repositorys
                .into_iter()
                .zip(users)
                .sorted_by_key(|(_, u)| u.as_ref().map(|u| u.id.clone()))
                .chunk_by(|(_, u)| u.as_ref().map(|u| u.id.clone()))
                .into_iter()
                .map(|(_, iter)| {
                    let mut iter = iter.peekable();
                    let user = iter
                        .peek()
                        .expect("why would there be a chunk with zero items?")
                        .1
                        .clone();
                    let repos = iter
                        .sorted_by_key(|(r, _)| r.name.to_lowercase())
                        .map(|(repo, _)| repo)
                        .collect::<Vec<_>>();
                    (user, repos)
                })
                .sorted_by_key(|(u, _)| u.as_ref().map(|u| u.login.to_lowercase()))
                .collect::<Vec<_>>())
        },
    );
    let repositorys_by_owner = move || (**repositorys_by_owner.read()).clone().transpose();
    Ok::<_, FrontendError>(Some(view! {
        <div>
            <button
                data-drawer-target="sidebar-multi-level-sidebar"
                data-drawer-toggle="sidebar-multi-level-sidebar"
                aria-controls="sidebar-multi-level-sidebar"
                type="button"
                class="inline-flex items-center p-2 mt-2 ms-3 text-sm text-gray-500 rounded-lg sm:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
            >
                <span class="sr-only">Open sidebar</span>
                <svg
                    class="w-6 h-6"
                    aria-hidden="true"
                    fill="currentColor"
                    viewBox="0 0 20 20"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        clip-rule="evenodd"
                        fill-rule="evenodd"
                        d="M2 4.75A.75.75 0 012.75 4h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 4.75zm0 10.5a.75.75 0 01.75-.75h7.5a.75.75 0 010 1.5h-7.5a.75.75 0 01-.75-.75zM2 10a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 10z"
                    ></path>
                </svg>
            </button>
            <aside
                id="sidebar-multi-level-sidebar"
                class="fixed top-0 left-0 z-40 w-64 h-screen transition-transform -translate-x-full sm:translate-x-0"
                aria-label="Sidebar"
            >
                <div class="h-full px-3 py-4 overflow-y-auto bg-gray-50 dark:bg-gray-800">
                    {move || {
                        repositorys_by_owner()
                            .map(|repositorys_by_owner| {
                                view! {
                                    <For
                                        each=move || {
                                            repositorys_by_owner.clone().into_iter().flatten()
                                        }
                                        key=move |(u, _)| u.as_ref().map(|u| u.id.clone())
                                        children=move |(user, repos)| {
                                            let user_is_none = user.is_none();
                                            let user_login = user.map(|u| u.login.clone());
                                            view! {
                                                <ul class="space-y-2 font-medium">
                                                    <li>
                                                        <button
                                                            type="button"
                                                            class="flex items-center w-full p-2 text-base text-gray-900 transition duration-75 rounded-lg group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
                                                            aria-controls="dropdown-example"
                                                            data-collapse-toggle="dropdown-example"
                                                        >
                                                            <span
                                                                class="flex-1 ms-3 text-left rtl:text-right whitespace-nowrap"
                                                                class=("italic", user_is_none)
                                                            >
                                                                {user_login.clone().unwrap_or("Unknown owner".to_owned())}
                                                            </span>
                                                            <Icon
                                                                icon=icondata::HiChevronDownSolidLg
                                                                class="w-6 h-6 fill-current"
                                                            />
                                                        </button>
                                                        <ul id="dropdown-example" class="py-2 space-y-2">
                                                            <For
                                                                each=move || repos.clone()
                                                                key=|r| r.id.clone()
                                                                children=move |repo| {
                                                                    let href = match &user_login {
                                                                        Some(u) => Some(format!("/{}/{}", u, repo.name)),
                                                                        None => todo!(),
                                                                    };

                                                                    view! {
                                                                        <li>
                                                                            <a
                                                                                href=href
                                                                                class="flex items-center w-full m-2 text-gray-900 transition duration-75 rounded-lg pl-11 group hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
                                                                            >
                                                                                {repo.name}
                                                                            </a>
                                                                        </li>
                                                                    }
                                                                }
                                                            />
                                                        </ul>
                                                    </li>
                                                </ul>
                                            }
                                        }
                                    />
                                }
                            })
                    }}
                </div>
            </aside>
            <main>{children}</main>
        </div>
    }))
}
