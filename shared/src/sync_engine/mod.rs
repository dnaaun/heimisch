use futures::future::BoxFuture;
use optimistic::db::{DbWithOptimisticChanges, ReactivityTrackers};
use parking_lot::Mutex;
use registry::Registry;
use std::{marker::PhantomData, sync::Arc};
use url::Url;
pub use websocket_updates::transport::*;
mod conversions;
mod initial_sync;

pub mod changes;
pub mod error;
pub mod mutations;
mod new_defn;
pub mod optimistic;
mod registry;
pub mod websocket_updates;

#[cfg(test)]
pub mod tests;

use std::cmp::Ordering;

use crate::{
    backend_api_trait::BackendApiTrait,
    endpoints::defns::api::installations::GetInstallationAccessTokenQueryParams,
    sync_engine::error::RawDbErrorToSyncError,
    types::{
        installation::InstallationId,
        installation_access_token_row::{InstallationAccessToken, InstallationAccessTokenRow},
    },
};
use error::SyncResult;
use jiff::{Timestamp, ToSpan};
use typed_db::RawDbTrait;

#[derive(Clone)]
pub struct DbSubscription {
    pub original_reactivity_trackers: ReactivityTrackers,
    pub func: Arc<dyn Fn() + Send + Sync>,
}

impl std::fmt::Debug for DbSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbSubscription")
            .field(
                "original_reactivity_trackers",
                &self.original_reactivity_trackers,
            )
            .field("func", &"some func yo")
            .finish()
    }
}

pub use new_defn::DbTableMarkers;

pub struct SyncEngine<
    RawDb: RawDbTrait,
    BackendApi: BackendApiTrait,
    Transport: TransportTrait,
    GithubApi,
> {
    pub db: Arc<DbWithOptimisticChanges<RawDb, DbTableMarkers>>,
    pub db_subscriptions: Registry<DbSubscription>,
    backend_api: Arc<BackendApi>,
    github_api: Arc<GithubApi>,
    _transport: PhantomData<Transport>,
}

impl<RawDb: RawDbTrait, BackendApi: BackendApiTrait, Transport: TransportTrait, GithubApi> Clone
    for SyncEngine<RawDb, BackendApi, Transport, GithubApi>
{
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            db_subscriptions: self.db_subscriptions.clone(),
            backend_api: self.backend_api.clone(),
            github_api: self.github_api.clone(),
            _transport: PhantomData,
        }
    }
}

const MAX_PER_PAGE: i32 = 100;

impl<RawDb: RawDbTrait, BackendApi: BackendApiTrait, Transport: TransportTrait, GithubApi>
    SyncEngine<RawDb, BackendApi, Transport, GithubApi>
{
    async fn get_api_conf(
        &self,
        id: &InstallationId,
    ) -> SyncResult<github_api::apis::configuration::Configuration, Transport, RawDb> {
        let bearer_access_token = Some(self.get_valid_iac(id).await?.token);
        let conf = github_api::apis::configuration::Configuration {
            user_agent: Some("Heimisch".into()),
            client: Default::default(),
            bearer_access_token,
            base_path: "https://api.github.com".parse().expect(""),
        };
        Ok(conf)
    }

    async fn get_valid_iac(
        &self,
        id: &InstallationId,
    ) -> SyncResult<InstallationAccessToken, Transport, RawDb> {
        let txn = self
            .db
            .txn()
            .with_table::<InstallationAccessTokenRow>()
            .build();

        let iac = txn
            .table::<InstallationAccessTokenRow>()
            .get_all()
            .await
            .tse()?
            .into_iter()
            .filter(|iac| {
                if &iac.installation_id != id {
                    return false;
                }

                matches!(
                    (iac.expires_at - Timestamp::now())
                        .compare(1.minute())
                        .unwrap(),
                    Ordering::Greater
                )
            })
            .map(|i| InstallationAccessToken {
                token: i.token,
                expires_at: i.expires_at,
            })
            .next();

        match iac {
            Some(iac) => Ok(iac),
            None => {
                let query_params = GetInstallationAccessTokenQueryParams {
                    installation_id: *id,
                };
                let resp = self
                    .backend_api
                    .get_installation_access_token(query_params)
                    .await?;

                let txn = self
                    .db
                    .txn()
                    .with_table::<InstallationAccessTokenRow>()
                    .read_write()
                    .build();
                txn.table::<InstallationAccessTokenRow>()
                    .put(&InstallationAccessTokenRow {
                        installation_id: *id,
                        expires_at: resp.expires_at.clone(),
                        token: resp.token.clone(),
                    })
                    .await
                    .tse()?;

                Ok(resp)
            }
        }
    }
}
