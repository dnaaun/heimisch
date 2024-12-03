use std::cell::LazyCell;

use shared::endpoints::endpoint_client::EndpointClient;
use url::Url;

pub fn redirect_handler(path: Url) {
    leptos::prelude::location()
        .set_href(path.as_str())
        .expect("");
}

pub const ENDPOINT_CLIENT: LazyCell<EndpointClient> = LazyCell::new(|| {
    let domain_name = Url::parse(env!("HEIMISCH_DOMAIN_NAME")).expect("");
    EndpointClient::new(redirect_handler, domain_name)
});
