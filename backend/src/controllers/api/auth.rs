use std::ops::Deref;

use crate::{
    custom_github_api::get_user_access_token,
    db::{
        self, delete_csrf_token, does_csrf_token_exist, store_csrf_token, upsert_login_user,
        LoginUser,
    },
    error::Error,
    hookup_endpoint::hookup,
    utils::gen_rand_string,
};
use axum::{extract::State, response::Redirect, routing::get, Router};
use github_api::apis::users_api::users_slash_get_authenticated;
use http::StatusCode;
use shared::{
    endpoints::defns::api::auth::{
        finish::{AuthFinishEndpoint, AuthFinishPayload, AuthFinishResponse},
        initiate::AuthInitiateEndpoint,
    },
    types::user::UserId,
};

use crate::app_state::AppState;

pub fn finish(router: Router<AppState>) -> Router<AppState> {
    hookup(
        AuthFinishEndpoint,
        router,
        |mut auth_session, state, payload| async move {
            let AuthFinishPayload {
                state: csrf_token,
                code,
            } = payload;

            if !does_csrf_token_exist(&state, csrf_token.clone()).await? {
                return Ok((
                    StatusCode::UNAUTHORIZED,
                    AuthFinishResponse::Failure {
                        message: "state/csrf token not found".to_string(),
                    }
                    .into(),
                ));
            }

            delete_csrf_token(&state, csrf_token).await?;

            let access_token = get_user_access_token(
                &state.config.github_api.non_api_root,
                code.as_str(),
                &state.config.github_api.client_id,
                &state.config.github_api.client_secret,
            )
            .await?;

            let resp = users_slash_get_authenticated(
                &state
                    .config
                    .get_gh_api_conf_with_access_token(access_token.deref().clone()),
            )
            .await?;

            let (id, login, email) = match resp {
                github_api::models::UsersGetAuthenticated200Response::Private(private_user) => {
                    (private_user.id, private_user.login, private_user.email)
                }
                github_api::models::UsersGetAuthenticated200Response::Public(public_user) => {
                    (public_user.id, public_user.login, public_user.email)
                }
            };
            let id = UserId::from(id);

            upsert_login_user(
                state,
                db::UpsertLoginUser {
                    github_user_id: id,
                    github_username: login.clone(),
                    github_email: email.clone(),
                    github_access_token: access_token.clone(),
                },
            )
            .await?;

            auth_session
                .login(&LoginUser {
                    github_user_id: id,
                    github_username: login,
                    github_email: email,
                    github_access_token: access_token.clone(),
                    last_last_in_touch_at: None,
                })
                .await?;

            Ok::<_, Error>((
                StatusCode::OK,
                AuthFinishResponse::Success(access_token).into(),
            ))
        },
    )
}

pub fn initiate(router: Router<AppState>) -> Router<AppState> {
    router.route(
        AuthInitiateEndpoint::PATH,
        get(|State(state): State<AppState>| async move {
            let csrf_token = gen_rand_string(20);

            store_csrf_token(&state, csrf_token.clone()).await?;

            let mut redirect_uri = state.config.heimisch_domain_url.clone();
            redirect_uri.set_path("/auth");

            let mut github_oauth_url = state
                .config
                .github_api
                .non_api_root
                .join("/login/oauth/authorize")
                .expect("");
            github_oauth_url.query_pairs_mut().extend_pairs([
                ("state", csrf_token),
                ("client_id", state.config.github_api.client_id.to_owned()),
                ("redirect_uri", redirect_uri.as_str().to_owned()),
            ]);

            Ok::<_, Error>(Redirect::to(github_oauth_url.as_str()))
        }),
    )
}
