use std::sync::Arc;

use axum::extract::FromRef;
use derive_more::derive::AsRef;
use leptos::config::LeptosOptions;

use crate::config::Config;

#[derive(Clone, FromRef, AsRef)]
pub struct AppState {
    pub pool: deadpool_diesel::postgres::Pool,
    pub config: Arc<Config>,
    pub leptos_options: LeptosOptions,
}
