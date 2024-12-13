use std::{ops::Deref, rc::Rc};

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use shared::sync_engine::SyncEngine;

use crate::consts::ENDPOINT_CLIENT;

pub type SyncEngineContext = SendWrapper<Rc<SyncEngine>>;

/// Will provide a context with the above `SyncEngineContext` type, and it will kick off.
#[component]
pub fn SyncEngineProvider(children: ChildrenFn) -> impl IntoView {
    let sync_engine = LocalResource::new(move || async move {
        Rc::new(
            shared::sync_engine::SyncEngine::new(ENDPOINT_CLIENT.with(|e| e.clone()))
                .await
                .unwrap(),
        )
    });

    view! {
        <Suspense>
            {move || sync_engine
                .read()
                .as_ref()
                .map(|sync_engine| {
                    provide_context::<SyncEngineContext>(sync_engine.clone());
                    children()
                })}
        </Suspense>
    }
}

pub fn use_sync_engine() -> impl Deref<Target = Rc<SyncEngine>> {
    use_context::<SyncEngineContext>().expect("")
}
