use url::Url;

thread_local! {
    pub static HEIMISCH_DOMAIN_URL: Url = Url::parse(env!("HEIMISCH_DOMAIN_NAME")).expect("");
}
