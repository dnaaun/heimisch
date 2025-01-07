use std::rc::Rc;

use leptos::prelude::*;
use send_wrapper::SendWrapper;

use crate::typed_transport::MyWebSocket;
pub type SyncEngine = shared::sync_engine::SyncEngine<MyWebSocket>;
pub type SyncEngineContext = SendWrapper<Rc<SyncEngine>>;

pub fn sync_engine_provided<V>(
    children: impl Fn() -> V + Send + Clone + 'static,
    sync_engine: LocalResource<Rc<SyncEngine>>,
) -> impl Fn() -> AnyView + Send + Clone + 'static
where
    V: IntoView + 'static,
{
    move || {
        let children = children.clone();
        view! {
            <Transition>
                {move || {
                    let children = children.clone();
                    sync_engine
                        .get()
                        .map(|sync_engine| {
                            provide_context::<SyncEngineContext>(sync_engine);
                            children()
                        })
                }}
            </Transition>
        }
        .into_any()
    }
}

pub fn use_sync_engine() -> Rc<SyncEngine> {
    use_context::<SyncEngineContext>().expect("").take()
}
