use leptos::{prelude::*, task::spawn_local};

use crate::local_storage::get_installation_ids_from_local_storage;

use super::sync_engine_provider::use_sync_engine;

#[component]
pub fn NotFound() -> impl IntoView {
    let sync_engine = use_sync_engine();
    let on_click = move |_| {
        let sync_engine = sync_engine.clone();
        spawn_local(async move {
            futures::future::join_all(
                get_installation_ids_from_local_storage()
                    .iter()
                    .map(|id| sync_engine.fetch_repositorys_for_installation_id(id)),
            )
            .await;
        });
    };
    view! {
        <div class="h-screen w-screen flex flex-col gap-4 justify-center items-center">
            <div class="font-bold text-9xl">404</div>
            <button
                on:click=on_click
                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800"
            >
                REMOVE WHEN DONE WITH TESTING(BUT FETCH REPOS FOR NOW)
            </button>
        </div>
    }
}
