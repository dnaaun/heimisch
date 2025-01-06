use std::env;

use reqwest::Url;

thread_local! {
    pub static HEIMISCH_API_DOMAIN: Url = Url::parse(&env::var("HEIMISCH_API_DOMAIN").expect("")).expect("");
    pub static HEIMISCH_FRONTEND_DOMAIN : Url = Url::parse(&env::var("HEIMISCH_FRONTEND_DOMAIN").expect("")).expect("");
}
