use std::{ops::Deref, rc::Rc};

use leptos::{prelude::*, task::spawn_local};
use send_wrapper::SendWrapper;
use shared::{endpoints::endpoint_client::EndpointClient, sync_engine::SyncEngine};
use url::Url;

use crate::{consts::HEIMISCH_DOMAIN_URL, local_storage::get_installation_ids_from_local_storage};

pub type SyncEngineContext = SendWrapper<Rc<SyncEngine>>;
pub type EndpointClientContext = SendWrapper<EndpointClient>;

pub fn redirect_handler(path: Url) {
    location().set_href(path.as_str()).expect("");
    panic!("Received a redirect without a (valid) URL: {path:?}")
}

/// Will provide a context with the above `SyncEngineWrapper` type, and it will kick off.
#[component]
pub fn SyncEngineProvider(children: ChildrenFn) -> impl IntoView {
    let endpoint_client = SendWrapper::new(EndpointClient::new(
        redirect_handler,
        HEIMISCH_DOMAIN_URL.with(Clone::clone),
    ));
    let endpoint_client2 = endpoint_client.clone();
    let sync_engine = LocalResource::new(move || {
        let endpoint_client2 = endpoint_client2.clone();
        async move {
            Rc::new(
                shared::sync_engine::SyncEngine::new(endpoint_client2.take())
                    .await
                    .unwrap(),
            )
        }
    });

    view! {
        <Suspense>
            {move || {
                sync_engine
                    .get()
                    .map(|sync_engine| {
                        let sync_engine2 = sync_engine.clone();
                        Effect::new(move || {
                            get_installation_ids_from_local_storage()
                                .into_iter()
                                .for_each(|id| {
                                    let sync_engine3 = sync_engine2.clone();
                                    spawn_local(async move {
                                        sync_engine3.clone().kick_off(&id).await.unwrap()
                                    });
                                });
                        });
                        provide_context::<SyncEngineContext>(sync_engine.clone());
                        provide_context::<EndpointClientContext>(endpoint_client.clone());
                        children()
                    })
            }}
        </Suspense>
    }
}

pub fn use_sync_engine() -> impl Deref<Target = Rc<SyncEngine>> {
    use_context::<SyncEngineContext>().expect("")
}
