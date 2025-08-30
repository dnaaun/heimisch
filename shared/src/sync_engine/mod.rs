use optimistic::db::{DbWithOptimisticChanges, ReactivityTrackers};
use registry::Registry;
use send_wrapper::SendWrapper;
use std::{future::Future, pin::Pin, sync::Arc};
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

use std::{cmp::Ordering, rc::Rc};

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
    pub func: Arc<dyn Fn()>,
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

pub use new_defn::DbStoreMarkers;

pub struct SyncEngine<
    RawDb: RawDbTrait,
    BackendApi: BackendApiTrait,
    Transport: TransportTrait,
    GithubApi,
> {
    pub db: Rc<DbWithOptimisticChanges<RawDb, DbStoreMarkers>>,
    pub db_subscriptions: SendWrapper<Rc<Registry<DbSubscription>>>,
    backend_api: Rc<BackendApi>,
    github_api: Rc<GithubApi>,
    make_transport: Rc<
        dyn Fn(Url) -> Pin<Box<dyn Future<Output = Result<Transport, Transport::TransportError>>>>,
    >,
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
            make_transport: self.make_transport.clone(),
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
            .tse()?
            .table::<InstallationAccessTokenRow>()
            .tse()?
            .get_all()
            .await
            .tse()?
            .into_iter()
            .filter(|iac| {
                if &iac.installation_id != id {
                    return false;
                }

                matches!(
                    (iac.token.expires_at - Timestamp::now())
                        .compare(1.minute())
                        .unwrap(),
                    Ordering::Greater
                )
            })
            .map(|i| i.token)
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
                txn.tse()?
                    .table::<InstallationAccessTokenRow>()
                    .tse()?
                    .put(&InstallationAccessTokenRow {
                        token: resp.clone(),
                        installation_id: *id,
                    })
                    .await
                    .tse()?;

                Ok(resp)
            }
        }
    }
}
