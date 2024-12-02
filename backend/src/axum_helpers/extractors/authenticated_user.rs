use axum::{extract::FromRequestParts, response::IntoResponse};
use axum_login::{AuthSession, AuthnBackend};
use derive_more::derive::Deref;
use http::request::Parts;

use crate::error::Error;

#[derive(Debug)]
pub struct AuthenticationFailedError;

#[derive(Deref)]
pub struct AuthenticatedUser<Backend: AuthnBackend>(Backend::User);

#[async_trait::async_trait]
impl<S: Sync + Send, Backend: axum_login::AuthnBackend + 'static> FromRequestParts<S>
    for AuthenticatedUser<Backend>
{
    type Rejection = axum::response::Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_session = AuthSession::<Backend>::from_request_parts(parts, state)
            .await
            .map_err(|e| e.into_response())?;

        match auth_session.user {
            Some(user) => Ok(AuthenticatedUser(user)),
            None => Err(Error::from(AuthenticationFailedError).into_response()),
        }
    }
}
