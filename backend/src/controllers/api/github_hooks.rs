use axum::{extract::State, routing::post, Json, Router};
use github_webhook_body::WebhookBody;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::endpoints::utils::GetInstallationIdFromWebhookBody;

use crate::{
    app_state::AppState,
    axum_helpers::extractors,
    db::{get_installation, upsert_webhook},
    error::{Error, ErrorSource},
};

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

pub fn github_hooks(router: Router<AppState>) -> Router<AppState> {
    // TODO: Do webhook verification.
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