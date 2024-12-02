#![feature(str_lines_remainder)]

use app_state::AppState;
use axum_login::AuthManagerLayerBuilder;
use config::{init_config, Config};
use deadpool_diesel::postgres::Manager;
use leptos::config::{get_configuration, ConfFile};
use leptos_axum::{generate_route_list, LeptosRoutes};
use session_and_auth::{AuthBackend, PgSessionStore};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tower_sessions::SessionManagerLayer;

use axum::Router;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod app_state;
mod axum_helpers;
mod config;
mod controllers;
pub mod custom_github_api;
mod db;
mod error;
pub mod hookup_endpoint;

pub mod session_and_auth;
#[cfg(test)]
mod tests;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

/// So I can use method chaining.
pub trait Then: Sized {
    fn then<B>(self, func: impl FnOnce(Self) -> B) -> B {
        func(self)
    }
}

impl<T> Then for T {}

/// `leptos_conf_file` is `Option`al because passing None (and consecuently building a
/// `Default::default()` was the only way I could get backend tests to pass).
async fn get_router(config: Config, leptos_conf_file: Option<ConfFile>) -> Router<()> {
    let leptos_conf_file = leptos_conf_file.unwrap_or_else(|| Default::default());
    let leptos_options = leptos_conf_file.leptos_options;
    // set up connection pool
    let manager = Manager::new(&config.db.url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    let state = AppState {
        pool,
        config: Arc::new(config),
        leptos_options: leptos_options.clone(),
    };

    let leptos_routes = generate_route_list(web::App);
    let leptos_options = state.leptos_options.clone();

    let auth_backend = AuthBackend::new(&state);
    let pg_session_store = PgSessionStore::new(&state);

    let session_layer = SessionManagerLayer::new(pg_session_store);
    let auth_layer = AuthManagerLayerBuilder::new(auth_backend, session_layer).build();

    Router::new()
        .leptos_routes(&state, leptos_routes, {
            move || web::shell(leptos_options.clone())
        })
        .then(controllers::api::auth::initiate)
        .then(controllers::api::auth::finish)
        .then(controllers::api::app_installs::create)
        .then(controllers::api::github_hooks::github_hooks)
        .then(controllers::api::installations::get_token)
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(
            web::shell,
        ))
        .layer(TraceLayer::new_for_http())
        .layer(auth_layer)
        .with_state(state)
}

#[tokio::main]
async fn main() {
    // setup tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::new("INFO"))
        .init();

    let config = init_config().await;

    let cargo_toml_path = None;
    let leptos_conf_file = leptos::config::get_configuration(cargo_toml_path).expect("");
    let leptos_options = leptos_conf_file.leptos_options;
    let addr = leptos_options.site_addr;

    // run it with hyper
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        get_router(config, Some(get_configuration(None).unwrap())).await,
    )
    .await
    .unwrap();
}
