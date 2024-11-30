mod websocket_updates;
mod sse_updates;

use github_api::apis::apps_api::apps_slash_get_installation;
use github_api::apis::configuration::Configuration;
use github_api::apis::users_api::users_slash_get_authenticated;
use github_webhook_body::WebhookBody;
use serde_json::Value;
use shared::endpoints::defns::api::installations::{
    GetInstallationAccessTokenEndpoint, GetInstallationAccessTokenPayload,
};
use shared::endpoints::utils::GetInstallationIdFromWebhookBody;
use std::time::SystemTime;

use crate::github_auth::WithAppAuth;
use crate::{
    axum_helpers::extractors,
    custom_github_api::{get_installation_access_token, get_user_access_token},
    db::{
        self, delete_csrf_token, does_csrf_token_exist, get_installation,
        insert_installation_if_not_exists, store_csrf_token, upsert_user, upsert_webhook,
    },
    error::{Error, ErrorSource},
    hookup_endpoint::HookupEndpoint,
};
use axum::{
    extract::State,
    response::Redirect,
    routing::{get, post},
    Json, Router,
};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use http::StatusCode;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use shared::endpoints::defns::api::{
    app_installs::create::{
        CreateAppInstallEndpoint, CreateAppInstallPayload, CreateAppInstallResponse,
    },
    auth::finish::{AuthFinishEndpoint, AuthFinishPayload, AuthFinishResponse},
};
use url::Url;

use crate::app_state::AppState;

pub fn api_auth_finish(router: Router<AppState>) -> Router<AppState> {
    router.hookup(AuthFinishEndpoint, |state, payload| async move {
        let AuthFinishPayload {
            state: csrf_token,
            code,
        } = payload;

        if !does_csrf_token_exist(&state, csrf_token.clone()).await? {
            return Ok((
                StatusCode::UNAUTHORIZED,
                AuthFinishResponse::Failure {
                    message: "state/csrf token not found".to_string(),
                },
            ));
        }

        delete_csrf_token(&state, csrf_token).await?;

        let user_access_token = get_user_access_token(
            code.as_str(),
            state.config.github_api.client_id.clone(),
            state.config.github_api.client_secret.clone(),
        )
        .await?;

        let resp = users_slash_get_authenticated(&github_api::apis::configuration::Configuration {
            user_agent: Some("Heimisch".into()),
            bearer_access_token: Some(user_access_token.clone()),
            ..Default::default()
        })
        .await?;

        let (id, login, email) = match resp {
            github_api::models::UsersGetAuthenticated200Response::Private(private_user) => {
                (private_user.id, private_user.login, private_user.email)
            }
            github_api::models::UsersGetAuthenticated200Response::Public(public_user) => {
                (public_user.id, public_user.login, public_user.email)
            }
        };

        upsert_user(
            state,
            db::LoginUser {
                github_user_id: id,
                github_username: login,
                github_email: email,
                github_access_token: user_access_token.clone(),
            },
        )
        .await?;

        Ok::<_, Error>((
            StatusCode::OK,
            AuthFinishResponse::Success { user_access_token },
        ))
    })
}

pub fn api_auth_initiate(router: Router<AppState>) -> Router<AppState> {
    router.route(
        "/api/auth/initiate",
        get(|State(state): State<AppState>| async move {
            let mut rng = StdRng::from_entropy();
            let csrf_token: String = (0..20).map(|_| rng.gen_range('a'..'z')).collect();

            store_csrf_token(&state, csrf_token.clone()).await?;

            let mut redirect_uri = state.config.heimisch_domain_url.clone();
            redirect_uri.set_path("/auth");

            let mut github_oauth_url =
                Url::parse("https://github.com/login/oauth/authorize").expect("");
            github_oauth_url.query_pairs_mut().extend_pairs([
                ("state", csrf_token),
                ("client_id", state.config.github_api.client_id.to_owned()),
                ("redirect_uri", redirect_uri.as_str().to_owned()),
            ]);

            Ok::<_, Error>(Redirect::to(github_oauth_url.as_str()))
        }),
    )
}

pub fn api_app_installs_create(router: Router<AppState>) -> Router<AppState> {
    router.hookup(CreateAppInstallEndpoint, |state, payload| async move {
        let CreateAppInstallPayload {
            installation_id,
            user_access_token,
        } = payload;

        // Make sure the current user (verified via user access token) owns the current
        // installation.
        // NOTE: Assumption that user access token doesn't change.
        let stmt = db::schema::login_users::table
            .select(db::schema::login_users::github_user_id)
            .filter(db::schema::login_users::github_access_token.eq(user_access_token.clone()));
        let github_user_id = state
            .pool
            .get()
            .await?
            .interact(|c| stmt.first::<i64>(c))
            .await??;

        apps_slash_get_installation(
            &Configuration::default()
                .with_app_auth(state.config.github_api.app_auth.clone())
                .expect(""),
            *installation_id as i32,
        )
        .await?;

        insert_installation_if_not_exists(
            &state,
            crate::db::Installation {
                id: *installation_id,
                github_user_id,
                created_at: SystemTime::now(),
            },
        )
        .await?;

        Ok::<_, Error>((
            StatusCode::OK,
            CreateAppInstallResponse::Success { installation_id },
        ))
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubWebhookHeaders {
    #[serde(rename = "x-github-hook-id")]
    pub x_github_hook_id: String,

    #[serde(rename = "x-github-event")]
    pub x_github_event: String,

    #[serde(rename = "x-github-delivery")]
    pub x_github_delivery: String,

    #[serde(rename = "x-hub-signature")]
    pub x_hub_signature: Option<String>,

    #[serde(rename = "x-hub-signature-256")]
    pub x_hub_signature_256: Option<String>,

    #[serde(rename = "user-agent")]
    pub user_agent: String,

    #[serde(rename = "x-github-hook-installation-target-type")]
    pub x_github_hook_installation_target_type: String,

    #[serde(rename = "x-github-hook-installation-target-id")]
    pub x_github_hook_installation_target_id: String,
}

pub fn api_github_webhooks(router: Router<AppState>) -> Router<AppState> {
    router.route(
        "/api/github_webhooks",
        post(
            |State(state): State<AppState>,
             extractors::Header(header): extractors::Header<GitHubWebhookHeaders>,
             Json(value): Json<Value>| async move {
                let GitHubWebhookHeaders {
                    x_github_hook_id: webhook_id,
                    x_github_event,
                    ..
                } = header;

                let webhook_id = webhook_id.parse::<i64>().map_err(|_| {
                    ErrorSource::GithubWebhookHeaderError {
                        message: format!("webhook id not convertable to i64: {webhook_id}"),
                    }
                })?;

                // The `Webhook` enum is structured in this way.
                let value = Value::Object([(x_github_event.clone(), value)].into_iter().collect());
                let body = serde_json::from_value::<WebhookBody>(value)
                    .map_err(|e| ErrorSource::GithubWebhookBodyDeser(e))?;

                let installation_id = match body.get_installation_id() {
                    Some(installation_id) => installation_id,
                    None => return Err(ErrorSource::GithubWebhookNoInstallationId { body }.into()),
                };

                get_installation(&state, installation_id)
                    .await?
                    .ok_or_else(|| ErrorSource::GithubWebhookHeaderError {
                        message: format!("installation id not found in db: {installation_id}"),
                    })?;

                upsert_webhook(&state, webhook_id, installation_id, body).await?;

                Ok::<_, Error>(())
            },
        ),
    )
}

pub fn api_installations_get_token(router: Router<AppState>) -> Router<AppState> {
    router.hookup(
        GetInstallationAccessTokenEndpoint,
        |state, payload| async move {
            let GetInstallationAccessTokenPayload { installation_id } = payload;
            get_installation(&state, installation_id)
                .await?
                .ok_or(Error::from(ErrorSource::InstallationIdNotFound(
                    installation_id,
                )))?;
            let signed_bearer_token = state.config.github_api.app_auth.generate_bearer_token().expect("");
            let token =
                get_installation_access_token(installation_id, &signed_bearer_token).await?;

            Ok::<_, Error>((StatusCode::OK, token))
        },
    )
}
