use crate::{
    custom_github_api::get_installation_access_token,
    db::get_installation,
    error::{Error, ErrorSource},
    hookup_endpoint::HookupEndpoint,
};
use axum::Router;
use http::StatusCode;
use shared::endpoints::defns::api::installations::{
    GetInstallationAccessTokenEndpoint, GetInstallationAccessTokenPayload,
};

use crate::app_state::AppState;

pub fn get_token(router: Router<AppState>) -> Router<AppState> {
    router.hookup(
        GetInstallationAccessTokenEndpoint,
        |_auth_session, state, payload| async move {
            let GetInstallationAccessTokenPayload { installation_id } = payload;
            get_installation(&state, installation_id)
                .await?
                .ok_or(Error::from(ErrorSource::InstallationIdNotFound(
                    installation_id,
                )))?;
            let signed_bearer_token = state
                .config
                .github_api
                .app_auth
                .generate_bearer_token()
                .expect("");
            let token =
                get_installation_access_token(installation_id, &signed_bearer_token).await?;

            Ok::<_, Error>((StatusCode::OK, token))
        },
    )
}
