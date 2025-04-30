use crate::consts::BACKEND_API;
use crate::local_storage::{
    add_installation_ids_to_local_storage, get_installation_ids_from_local_storage,
};
use leptos::prelude::*;

use shared::backend_api_trait::BackendApiTrait;
use shared::utils::LogErr;
use std::collections::HashSet;
use wasm_bindgen_futures::spawn_local;

use super::sync_engine_provider::use_sync_engine;

/// RTI: Uses `app::sync_engine_provided::SyncEngineContext` context.
pub fn use_sync_installation_ids_and_recv_websocket_updates() {
    let sync_engine = use_sync_engine();
    Effect::new(move || {
        let sync_engine2 = sync_engine.clone();
        spawn_local(async move {
            let _ = sync_engine2
                .clone()
                .recv_websocket_updates()
                .await
                .log_err();
        });

        let sync_engine2 = sync_engine.clone();
        spawn_local(async move {
            let sync_engine = sync_engine2.clone();

            // First, sync existing installation IDs
            let existing_ids = get_installation_ids_from_local_storage();
            for id in existing_ids.iter().cloned() {
                let sync_engine = sync_engine.clone();
                spawn_local(async move {
                    let _ = sync_engine
                        .fetch_repositorys_for_installation_id(&id)
                        .await
                        .log_err();
                });
            }

            // Then, get installations from backend and sync any new ones
            if let Ok(get_installations_resp) = BACKEND_API
                .with(|e| e.clone())
                .get_installations()
                .await
                .log_err()
            {
                let new_ids = get_installations_resp
                    .installations
                    .into_iter()
                    .map(|i| i.id)
                    .collect::<HashSet<_>>()
                    .difference(&existing_ids)
                    .cloned()
                    .collect::<HashSet<_>>();

                add_installation_ids_to_local_storage(&new_ids);

                for id in new_ids {
                    let sync_engine = sync_engine.clone();
                    spawn_local(async move {
                        let _ = sync_engine
                            .fetch_repositorys_for_installation_id(&id)
                            .await
                            .log_err();
                    });
                }
            };
        });
    });
}
