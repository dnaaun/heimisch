use deadpool_diesel::postgres::Pool;
use shared::endpoints::defns::api::auth::finish::GithubAccessToken;
use shared::types::user::UserId;
use tower_sessions::session::Id;
use tower_sessions::session_store::{Error as TSError, Result as TSResult};
use tower_sessions::{session::Record, SessionStore};

use crate::db::get_session;
use crate::db::{create_session, delete_session, get_login_user, upsert_session, LoginUser};

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

#[derive(Clone)]
pub struct PgSessionStore {
    pool: Pool,
}

opaque_debug::implement!(PgSessionStore);

impl PgSessionStore {
    pub fn new(pool: impl AsRef<Pool>) -> Self {
        Self {
            pool: pool.as_ref().clone(),
        }
    }
}

impl AsRef<Pool> for PgSessionStore {
    fn as_ref(&self) -> &Pool {
        &self.pool
    }
}

fn crate_err_to_sessions_err(err: crate::error::Error) -> TSError {
    TSError::Backend(format!("{err:?}"))
}

#[async_trait::async_trait]
impl SessionStore for PgSessionStore {
    async fn create(&self, record: &mut Record) -> TSResult<()> {
        loop {
            if let None = get_session(&self, &record.id)
                .await
                .map_err(crate_err_to_sessions_err)?
            {
                break;
            }
            record.id = Default::default(); // this actually invokes a randomness generator.
        }

        create_session(&self, record)
            .await
            .map_err(crate_err_to_sessions_err)
    }

    async fn save(&self, record: &Record) -> TSResult<()> {
        upsert_session(&self, record)
            .await
            .map_err(crate_err_to_sessions_err)
    }

    async fn load(&self, id: &Id) -> TSResult<Option<Record>> {
        get_session(&self, id)
            .await
            .map_err(crate_err_to_sessions_err)
    }

    async fn delete(&self, id: &Id) -> TSResult<()> {
        delete_session(&self, id)
            .await
            .map_err(crate_err_to_sessions_err)
    }
}

impl axum_login::AuthUser for LoginUser {
    type Id = UserId;

    fn id(&self) -> Self::Id {
        self.github_user_id
    }

    fn session_auth_hash(&self) -> &[u8] {
        // NOTE: Maybe I should hash this?
        self.github_access_token.as_ref().as_bytes()
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
