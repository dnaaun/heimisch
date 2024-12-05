use axum::Router;
use axum_login::AuthManagerLayerBuilder;
use tower_sessions::SessionManagerLayer;

use crate::{
    app_state::AppState, auth_backend::AuthBackend, pg_session_store::PgSessionStore, Then,
};

mod app_installs;
mod auth;
mod github_hooks;
mod installations;
mod websocket_updates;

pub fn get_api_router(state: AppState) -> Router<AppState> {
    let auth_backend = AuthBackend::new(&state);
    let pg_session_store = PgSessionStore::new(&state);

    let session_layer = SessionManagerLayer::new(pg_session_store);
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

    let router = Router::new()
        .then(auth::initiate)
        .then(auth::finish)
        .then(app_installs::create)
        .then(github_hooks::github_hooks)
        .then(installations::get_token)
        .then(websocket_updates::api_websocket_updates)
        .layer(auth_layer);

    router
}
