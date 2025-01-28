#![feature(str_lines_remainder)]

use app_state::AppState;
use config::{init_config, Config};
use controllers::api::get_api_router;
use deadpool_diesel::postgres::Manager;
use http::{header, HeaderValue, Method};
use shared::endpoints::endpoint_client::CUSTOM_REDIRECT_HEADER_NAME;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

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

pub mod auth_backend;
pub mod pg_session_store;

#[cfg(test)]
mod tests;
pub mod utils;
mod websocket_updates_bucket;

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
async fn get_router(config: Config) -> Router<()> {
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
        websocket_updates_bucket: Default::default(),
    };

    get_api_router(state.clone())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

const HEADERS_TO_ALLOW_FOR_CORS: [http::HeaderName; 10] = [
    header::CONTENT_TYPE,
    header::CONTENT_ENCODING,
    header::AUTHORIZATION,
    CUSTOM_REDIRECT_HEADER_NAME,
    header::ACCEPT,
    header::ACCEPT_LANGUAGE,
    header::ACCEPT_ENCODING,
    header::USER_AGENT,
    header::SET_COOKIE,
    header::COOKIE,
];

#[tokio::main]
async fn main() {
    // setup tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(EnvFilter::new("INFO"))
        .init();

    let config = init_config().await;

    // run it with hyper
    tracing::info!("listening on {}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind((config.host, config.port))
        .await
        .unwrap();

    let router = get_router(config).await;

    // TODO: Hide this behind a dev/prod distinction (probably via feature flags).
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().expect(""))
        .allow_methods([Method::GET, Method::PUT, Method::POST])
        .allow_headers(HEADERS_TO_ALLOW_FOR_CORS)
        .expose_headers(HEADERS_TO_ALLOW_FOR_CORS)
        .allow_credentials(true);
    let router = router.layer(cors);
    axum::serve(listener, router).await.unwrap();
}
