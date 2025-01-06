use crate::app_state::AppState;
use crate::db::{self, insert_installation_if_not_exists};
use crate::error::Error;
use crate::hookup_endpoint::hookup_post_authenticated;
use axum::Router;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use github_api::apis::apps_api::apps_slash_get_installation;
use http::StatusCode;
use shared::endpoints::defns::api::app_installs::create::{
    CreateAppInstallEndpoint, CreateAppInstallPayload, CreateAppInstallResponse,
};
use shared::types::user::UserId;
use std::time::SystemTime;

pub fn create(router: Router<AppState>) -> Router<AppState> {
    hookup_post_authenticated(
        CreateAppInstallEndpoint,
        router,
        |auth_user, state, payload| async move {
            let CreateAppInstallPayload { installation_id } = payload;

            // Make sure the current user (verified via user access token) owns the current
            // installation.
            // NOTE: Assumption that user access token doesn't change.
            let stmt = db::schema::login_users::table
                .select(db::schema::login_users::github_user_id)
                .filter(
                    db::schema::login_users::github_access_token
                        .eq(auth_user.github_access_token.clone()),
                );
            let github_user_id = state
                .pool
                .get()
                .await?
                .interact(|c| stmt.first::<UserId>(c))
                .await??;

            apps_slash_get_installation(
                &state.config.get_gh_api_conf_with_app_auth(),
                *installation_id as i32,
            )
            .await?;

            insert_installation_if_not_exists(
                &state,
                crate::db::Installation {
                    id: installation_id,
                    github_user_id,
                    created_at: SystemTime::now(),
                },
            )
            .await?;

            Ok::<_, Error>((
                StatusCode::OK,
                CreateAppInstallResponse::Success { installation_id }.into(),
            ))
        },
    )
}
