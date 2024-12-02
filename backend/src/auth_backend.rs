use deadpool_diesel::postgres::Pool;
use shared::{endpoints::defns::api::auth::finish::GithubAccessToken, types::user::UserId};

use crate::db::{get_login_user, LoginUser};

#[derive(Clone)]
pub struct AuthBackend {
    pool: Pool,
}

impl AuthBackend {
    pub fn new(pool: impl AsRef<Pool>) -> Self {
        Self {
            pool: pool.as_ref().clone(),
        }
    }
}

impl AsRef<Pool> for AuthBackend {
    fn as_ref(&self) -> &Pool {
        &self.pool
    }
}

pub struct Credentials {
    github_user_id: UserId,
    access_token: GithubAccessToken,
}

#[async_trait::async_trait]
impl axum_login::AuthnBackend for AuthBackend {
    type User = LoginUser;
    type Credentials = Credentials;

    /// An error which can occur during authentication and authorization.
    type Error = crate::error::Error;

    /// Authenticates the given credentials with the backend.
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let login_user = get_login_user(self, &creds.github_user_id).await;
        match &login_user {
            Ok(Some(user)) => {
                if user.github_access_token == creds.access_token {
                    login_user
                } else {
                    Ok(None)
                }
            }
            _ => login_user,
        }
    }

    async fn get_user(&self, user_id: &UserId) -> Result<Option<Self::User>, Self::Error> {
        get_login_user(self, user_id).await
    }
}
