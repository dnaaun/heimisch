use crate::app_state::AppState;
use crate::db::{self, insert_installation_if_not_exists};
use crate::error::Error;
use crate::github_auth::WithAppAuth;
use crate::hookup_endpoint::HookupEndpoint;
use axum::Router;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use github_api::apis::apps_api::apps_slash_get_installation;
use github_api::apis::configuration::Configuration;
use http::StatusCode;
use shared::endpoints::defns::api::app_installs::create::{
    CreateAppInstallEndpoint, CreateAppInstallPayload, CreateAppInstallResponse,
};
use shared::types::user::UserId;
use std::time::SystemTime;

pub fn create(router: Router<AppState>) -> Router<AppState> {
    router.hookup(
        CreateAppInstallEndpoint,
        |_auth_session, state, payload| async move {
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
                .interact(|c| stmt.first::<UserId>(c))
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
        },
    )
}
