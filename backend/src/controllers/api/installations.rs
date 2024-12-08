use crate::{
    custom_github_api::get_installation_access_token,
    db::get_installation,
    error::{Error, ErrorSource},
    hookup_endpoint::hookup_authenticated,
};
use axum::Router;
use http::StatusCode;
use shared::endpoints::{
    defns::api::installations::{
        GetInstallationAccessTokenEndpoint, GetInstallationAccessTokenPayload,
    },
    endpoint_client::MaybePageRedirect,
};

use crate::app_state::AppState;

pub fn get_token(router: Router<AppState>) -> Router<AppState> {
    hookup_authenticated(
        GetInstallationAccessTokenEndpoint,
        router,
        |auth_user, state, payload| async move {
            let GetInstallationAccessTokenPayload { installation_id } = payload;
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

            Ok::<_, Error>((StatusCode::OK, MaybePageRedirect::NoRedirect(token)))
        },
    )
}
