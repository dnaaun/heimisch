use std::env;

use reqwest::Url;

thread_local! {
    pub static HEIMISCH_DOMAIN_URL: Url = Url::parse(&env::var("HEIMISCH_DOMAIN_NAME").expect("")).expect("");
}
