use std::rc::Rc;

use leptos::prelude::*;
use send_wrapper::SendWrapper;
use shared::sync_engine::Transport;
use shared::{backend_api_trait::BackendApi, github_api_trait::GithubApi};

use crate::typed_transport::BinaryTransport;
pub type SyncEngine = shared::sync_engine::SyncEngine<
    SendWrapper<idb::Database>,
    BackendApi,
    Transport<BinaryTransport>,
    GithubApi,
>;
pub type SyncEngineContext = SendWrapper<SyncEngine>;

pub fn sync_engine_provided<V>(
    children: impl Fn() -> V + Send + Clone + 'static,
    sync_engine: Resource<SyncEngine>,
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

pub fn use_sync_engine() -> SyncEngine {
    use_context::<SyncEngineContext>().expect("").take()
}
