use std::cell::LazyCell;

use url::Url;

pub const HEIMISCH_DOMAIN_URL: LazyCell<Url> =
    LazyCell::new(|| Url::parse(env!("HEIMISCH_DOMAIN_NAME")).expect(""));
