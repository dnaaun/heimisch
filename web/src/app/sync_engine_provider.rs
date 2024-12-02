use std::{ops::Deref, rc::Rc};

use leptos::{prelude::*, task::spawn_local};
use send_wrapper::SendWrapper;
use shared::sync_engine::SyncEngine;

use crate::{consts::HEIMISCH_DOMAIN_URL, local_storage::get_installation_ids_from_local_storage};

pub type SyncEngineContext = SendWrapper<Rc<SyncEngine>>;

/// Will provide a context with the above `SyncEngineWrapper` type, and it will kick off.
#[component]
pub fn SyncEngineProvider(children: ChildrenFn) -> impl IntoView {
    let sync_engine = LocalResource::new(|| async {
        Rc::new(
            shared::sync_engine::SyncEngine::new(HEIMISCH_DOMAIN_URL.with(Clone::clone))
                .await
                .unwrap(),
        )
    });

    view! {
        <Suspense>
        {
            move || sync_engine.get().map(|sync_engine| {
                 let sync_engine2 = sync_engine.clone();
                Effect::new(move || {
                    // TODO: I'll probaly need to implemetn some sort of rate limiting on
                    // synchronization.
                    get_installation_ids_from_local_storage()
                        .into_iter().for_each(|id| {
                            let sync_engine3 = sync_engine2.clone();
                            spawn_local(async move { sync_engine3.clone().kick_off(&id).await.unwrap() });
                        });
                });
                provide_context::<SyncEngineContext>(sync_engine.clone());
                children()
            })
        }
        </Suspense>
    }
}

pub fn use_sync_engine() -> impl Deref<Target = Rc<SyncEngine>> {
    use_context::<SyncEngineContext>().expect("")
}
