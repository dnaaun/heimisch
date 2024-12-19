use leptos::prelude::*;
use shared::types::{
    repository::RepositoryId, repository_initial_sync_status::RepositoryInitialSyncStatus,
};

use crate::{
    app::{flowbite::Spinner, sync_engine_provider::use_sync_engine},
    idb_signal_from_sync_engine::IdbSignalFromSyncEngine,
};

#[component]
pub fn AppHeaderContextItem(
    text: Signal<String>,
    on_click: Box<dyn Fn()>,
    #[prop(optional)] bold: bool,
) -> impl IntoView {
    view! {
        <a
            on:click=move |_| on_click()
            class="flex items-center min-w-xs leading-6 text-inherit no-underline rounded-md px-1.5 py-1 cursor-pointer hover:bg-gray-200"
            class=("font-semibold", bold)
        >
            <span>{text}</span>
        </a>
    }
}

#[component]
pub fn TopBar(
    #[prop(into)] repository_id: Signal<RepositoryId>,
    #[prop(into)] owner_name: Signal<String>,
    #[prop(into)] repo_name: Signal<String>,
) -> impl IntoView {
    let initial_sync_done = use_sync_engine().idb_signal(
        move |builder| builder.with_store::<RepositoryInitialSyncStatus>().build(),
        move |txn| async move {
            Ok(
                match txn
                    .object_store::<RepositoryInitialSyncStatus>()?
                    .get(&repository_id.get())
                    .await?
                {
                    Some(RepositoryInitialSyncStatus {
                        status: shared::types::repository_initial_sync_status::RepoSyncStatus::Full,
                        ..
                    }) => true,
                    _ => false,
                },
            )
        },
    );
    let initial_sync_done = Memo::new(move |_| initial_sync_done.get());

    let status_el = move || match initial_sync_done.get() {
        Some(result) => match result {
            Ok(is_loaded) => {
                if is_loaded {
                    ().into_any()
                } else {
                    view! { <Spinner width="16px" /> }.into_any()
                }
            }
            Err(_) => "Error doing initial sync".into_any(),
        },
        None => view! { <Spinner width="16px" /> }.into_any(),
    };

    view! {
        <div class="pl-4 pr-4 pt-4 pb-2 bg-gray-50 flex items-center flex-nowrap">
            <div class="w-8"></div>
            <nav>
                <ul class="list-none m-0 p-0 flex items-center">
                    <li class="flex items-center">
                        <AppHeaderContextItem text=owner_name on_click=Box::new(|| ()) />
                        <span>/</span>
                    </li>
                    <li>
                        <AppHeaderContextItem bold=true text=repo_name on_click=Box::new(|| ()) />
                    </li>
                </ul>
            </nav>
            <div class="pl-2 flex flex-nowrap justify-center items-center gap-1 text-gray-300 text-xs">{status_el}</div>
        </div>
    }
}
