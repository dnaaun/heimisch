use std::sync::Arc;

use axum::extract::FromRef;
use derive_more::derive::AsRef;

use crate::{config::Config, websocket_updates_bucket::WebsocketUpdatesBucket};

#[derive(Clone, FromRef, AsRef)]
pub struct AppState {
    pub pool: deadpool_diesel::postgres::Pool,
    pub config: Arc<Config>,
    pub websocket_updates_bucket: Arc<WebsocketUpdatesBucket>,
}
