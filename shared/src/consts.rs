use url::Url;

thread_local! {
    pub static HEIMISCH_API_DOMAIN: Url = Url::parse(env!("HEIMISCH_API_DOMAIN")).expect("");
    pub static HEIMISCH_FRONTEND_DOMAIN : Url = Url::parse(env!("HEIMISCH_FRONTEND_DOMAIN")).expect("");
}
