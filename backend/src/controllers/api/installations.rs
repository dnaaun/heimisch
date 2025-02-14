use crate::{
    custom_github_api::get_installation_access_token,
    db::{self, get_installation},
    error::{Error, ErrorSource},
    hookup_endpoint::hookup_get_authenticated,
};
use axum::Router;
use http::StatusCode;
use shared::{
    endpoints::defns::api::installations::{
        GetInstallationAccessTokenEndpoint, GetInstallationAccessTokenQueryParams,
        GetInstallationsEndpoint, GetInstallationsResponse,
    },
    types::installation::Installation,
};

use crate::app_state::AppState;

pub fn get_token(router: Router<AppState>) -> Router<AppState> {
    hookup_get_authenticated(
        GetInstallationAccessTokenEndpoint,
        router,
        |auth_user, state, query_params| async move {
            let GetInstallationAccessTokenQueryParams { installation_id } = query_params;
            let installation =
                get_installation(&state, installation_id)
                    .await?
                    .ok_or(Error::from(ErrorSource::InstallationIdNotFound(
                        installation_id,
                    )))?;
            if installation.github_user_id != auth_user.github_user_id {
                return Err(ErrorSource::AuthorizationFailed.into());
            }
            let signed_bearer_token = state
                .config
                .github_api
                .app_auth
                .generate_bearer_token()
                .expect("");
            let token = get_installation_access_token(
                &state.config.github_api.api_root,
                installation_id,
                &signed_bearer_token,
            )
            .await?;

            Ok::<_, Error>((StatusCode::OK, token))
        },
    )
}

pub fn get_installations(router: Router<AppState>) -> Router<AppState> {
    hookup_get_authenticated(
        GetInstallationsEndpoint,
        router,
        |auth_user, state, _| async move {
            let resp = GetInstallationsResponse {
                installations: db::get_installations(&state, auth_user.github_user_id)
                    .await
                    .map(|vec| {
                        vec.into_iter()
                            .map(|i| Installation {
                                id: i.id,
                                created_at: i.created_at,
                                github_user_id: i.github_user_id,
                            })
                            .collect()
                    })?,
            };
            Ok::<(http::StatusCode, GetInstallationsResponse), crate::error::Error>((
                StatusCode::OK,
                resp,
            ))
        },
    )
}
