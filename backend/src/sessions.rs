use tower_sessions::session::Id;
use tower_sessions::session_store::{Error as TSError, Result as TSResult};
use tower_sessions::{session::Record, SessionStore};

use crate::db::{create_session, delete_session, upsert_session};
use crate::{app_state::AppState, db::get_session};

fn crate_err_to_sessions_err(err: crate::error::Error) -> TSError {
    TSError::Backend(format!("{err:?}"))
}

#[async_trait::async_trait]
impl SessionStore for AppState {
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
