use shared::{backend_api_trait::BackendApi, endpoints::endpoint_client::EndpointClient};
use url::Url;

pub fn redirect_handler(url: Url) {
    leptos::prelude::location().set_href(url.as_str()).expect("");
}

thread_local! {
    pub static BACKEND_API: BackendApi = {
        let domain_name = Url::parse(env!("HEIMISCH_API_DOMAIN")).expect("");
        BackendApi::new(EndpointClient::new(redirect_handler, domain_name))
    };
}
