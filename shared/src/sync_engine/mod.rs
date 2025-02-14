use optimistic::db::{DbWithOptimisticChanges, ReactivityTrackers};
use registry::Registry;
use send_wrapper::SendWrapper;
use url::Url;
use std::{future::Future, pin::Pin, sync::Arc};
pub use websocket_updates::transport::*;
mod conversions;
mod initial_sync;

pub mod changes;
pub mod error;
pub mod mutations;
pub mod optimistic;
mod registry;
pub mod storage_traits;
pub mod websocket_updates;

#[cfg(test)]
pub mod tests;

use std::{cmp::Ordering, rc::Rc};

use crate::{
    endpoints::{
        defns::api::installations::{
            GetInstallationAccessTokenEndpoint, GetInstallationAccessTokenQueryParams,
        },
        endpoint_client::EndpointClient,
    },
    types::{
        installation::InstallationId,
        installation_access_token_row::{InstallationAccessToken, InstallationAccessTokenRow},
    },
};
use error::SyncResult;
use jiff::{Timestamp, ToSpan};

#[derive(Clone)]
pub struct DbSubscription {
    pub original_reactivity_trackers: ReactivityTrackers,
    pub func: Arc<dyn Fn()>,
}
mod new_defn;

pub use new_defn::DbStoreMarkers;

pub struct SyncEngine<Transport: TransportTrait, GithubApi> {
    pub db: Rc<DbWithOptimisticChanges<DbStoreMarkers>>,
    pub db_subscriptions: SendWrapper<Rc<Registry<DbSubscription>>>,
    endpoint_client: EndpointClient,
    github_api: Rc<GithubApi>,
    make_transport: Rc<dyn Fn(Url) -> Pin<Box<dyn Future<Output = Result<Transport, Transport::TransportError>>>>>,
}

impl<Transport: TransportTrait, GithubApi> Clone for SyncEngine<Transport, GithubApi> {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            db_subscriptions: self.db_subscriptions.clone(),
            endpoint_client: self.endpoint_client.clone(),
            github_api: self.github_api.clone(),
            make_transport: self.make_transport.clone(),
        }
    }
}

const MAX_PER_PAGE: i32 = 100;

impl<W: TransportTrait, GithubApi> SyncEngine<W, GithubApi> {
    async fn get_api_conf(
        &self,
        id: &InstallationId,
    ) -> SyncResult<github_api::apis::configuration::Configuration, W> {
        let bearer_access_token = Some(self.get_valid_iac(id).await?.token);
        let conf = github_api::apis::configuration::Configuration {
            user_agent: Some("Heimisch".into()),
            client: self.endpoint_client.client.clone(),
            bearer_access_token,
            base_path: "https://api.github.com".parse().expect(""),
        };
        Ok(conf)
    }

    async fn get_valid_iac(&self, id: &InstallationId) -> SyncResult<InstallationAccessToken, W> {
        let txn = self
            .db
            .txn()
            .with_store::<InstallationAccessTokenRow>()
            .build();
        let iac = txn
            .object_store::<InstallationAccessTokenRow>()?
            .no_optimism_get_all()
            .await?
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
                    .endpoint_client
                    .make_get_request(GetInstallationAccessTokenEndpoint, query_params)
                    .await?;

                let txn = self
                    .db
                    .txn()
                    .with_store::<InstallationAccessTokenRow>()
                    .read_write()
                    .build();
                txn.object_store::<InstallationAccessTokenRow>()?
                    .put(&InstallationAccessTokenRow {
                        token: resp.clone(),
                        installation_id: *id,
                    })
                    .await?;

                Ok(resp)
            }
        }
    }
}
