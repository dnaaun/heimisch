use shared::endpoints::endpoint_client::EndpointClient;
use url::Url;

pub fn redirect_handler(url: Url) {
    leptos::prelude::location().set_href(url.as_str()).expect("");
}

thread_local! {
    pub static ENDPOINT_CLIENT: EndpointClient = {
        let domain_name = Url::parse(env!("HEIMISCH_API_DOMAIN")).expect("");
        EndpointClient::new(redirect_handler, domain_name)
    };
}
