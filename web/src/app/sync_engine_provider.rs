use std::{rc::Rc, sync::Arc};

use crate::typed_websocket_client::TypedWebsocketClient;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
type SyncEngine = shared::sync_engine::SyncEngine<TypedWebsocketClient>;
pub type SyncEngineContext = SendWrapper<Rc<SyncEngine>>;

/// Will provide a context with the above `SyncEngineContext` type, and it will kick off.
#[component]
fn SyncEngineProvider(
    children: ChildrenFn,
    sync_engine: LocalResource<Rc<SyncEngine>>,
) -> impl IntoView {
    view! {
        <Transition>
            {move || {
                sync_engine
                    .read()
                    .as_ref()
                    .map(|sync_engine| {
                        provide_context::<SyncEngineContext>(sync_engine.clone());
                        children()
                    })
            }}
        </Transition>
    }
}

/// TODO: Refactor such that <SyncEngineProvider> is not a separate component since this function
/// is it's only usgae.
pub fn sync_engine_provided<V>(
    children: impl Fn() -> V + Send + Sync + 'static,
    sync_engine: LocalResource<Rc<SyncEngine>>,
) -> impl leptos_router::ChooseView
where
    V: IntoView + 'static,
{
    let children = Arc::new(children);
    move || {
        let children = children.clone();
        view! { <SyncEngineProvider sync_engine=sync_engine>{children()}</SyncEngineProvider> }
            .into_any()
    }
}

pub fn use_sync_engine() -> Rc<SyncEngine> {
    use_context::<SyncEngineContext>().expect("").take()
}
