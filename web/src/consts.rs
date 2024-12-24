use shared::endpoints::endpoint_client::EndpointClient;
use url::Url;

pub fn redirect_handler(path: Url) {
    leptos::prelude::location()
        .set_href(path.as_str())
        .expect("");
}

thread_local! {
    pub static ENDPOINT_CLIENT: EndpointClient = {
        let domain_name = Url::parse(env!("HEIMISCH_DOMAIN_NAME")).expect("");
        tracing::info!("YOOO: {domain_name}");
        EndpointClient::new(redirect_handler, domain_name)
    };
}
