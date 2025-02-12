use std::rc::Rc;

use github_api::github_api_trait::GithubApi;
use leptos::prelude::*;
use send_wrapper::SendWrapper;
use shared::sync_engine::Transport;

use crate::typed_transport::BinaryTransport;
pub type SyncEngine = shared::sync_engine::SyncEngine<Transport<BinaryTransport>, GithubApi>;
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
