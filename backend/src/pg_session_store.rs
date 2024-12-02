use deadpool_diesel::postgres::Pool;
use shared::types::user::UserId;
use tower_sessions::session::Id;
use tower_sessions::session_store::{Error as TSError, Result as TSResult};
use tower_sessions::{session::Record, SessionStore};

use crate::db::get_session;
use crate::db::{create_session, delete_session, upsert_session, LoginUser};

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
            if get_session(&self, &record.id)
                .await
                .map_err(crate_err_to_sessions_err)?
                .is_none()
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
