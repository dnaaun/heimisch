use std::{cell::LazyCell, env};

use reqwest::Url;

pub const HEIMISCH_DOMAIN_URL: LazyCell<Url> = LazyCell::new(|| {
    dotenvy::dotenv().ok();
    Url::parse(&env::var("HEIMISCH_DOMAIN_NAME").expect("")).expect("")
});
