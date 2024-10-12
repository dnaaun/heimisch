use parking_lot::Mutex;
use registry::Registry;
use typesafe_idb::{Chain, ReactivityTrackers, TypesafeDb};
mod conversions;
mod ensure_initial_sync_issues;
mod ensure_initial_sync_one_repository;
mod ensure_initial_sync_repositories;
mod kick_off;

pub mod changes;
pub mod error;
mod registry;

use std::{cmp::Ordering, rc::Rc, sync::Arc};

use crate::{
    endpoints::{
        defns::api::installations::{
            GetInstallationAccessTokenEndpoint, GetInstallationAccessTokenPayload,
        },
        endpoint_request::EndpointRequest,
    },
    types::{
        github_app::{GithubApp, GithubAppStoreMarker},
        installation::InstallationId,
        installation_access_token_row::{
            InstallationAccessToken, InstallationAccessTokenRow,
            InstallationAccessTokenRowStoreMarker,
        },
        issue::{Issue, IssueStoreMarker},
        issue_initial_sync_status::{IssueInitialSyncStatus, IssueInitialSyncStatusStoreMarker},
        license::{License, LicenseStoreMarker},
        milestone::{Milestone, MilestoneStoreMarker},
        repository::{Repository, RepositoryStoreMarker},
        repository_initial_sync_status::{
            RepositoryInitialSyncStatus, RepositoryInitialSyncStatusStoreMarker,
        },
        user::{User, UserStoreMarker},
    },
};
use error::SyncResult;
use jiff::{Timestamp, ToSpan};
use url::Url;

pub type DbStoreMarkers = Chain<
    InstallationAccessTokenRowStoreMarker,
    Chain<
        RepositoryInitialSyncStatusStoreMarker,
        Chain<
            IssueInitialSyncStatusStoreMarker,
            Chain<
                LicenseStoreMarker,
                Chain<
                    MilestoneStoreMarker,
                    Chain<
                        RepositoryStoreMarker,
                        Chain<
                            GithubAppStoreMarker,
                            Chain<UserStoreMarker, Chain<IssueStoreMarker, Chain<(), ()>>>,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;

pub enum IdbNotification {
    BulkStoreUpdate {
        store_name: &'static str,
    },
    SingleRecoreUpdate {
        store_name: &'static str,

        /// Will be a serde_json-serialized value.
        id: String,
    },
}

impl IdbNotification {
    pub fn matches_triggered_trackers(&self, reactivity_trackers: &ReactivityTrackers) -> bool {
        let ReactivityTrackers {
            stores_accessed_by_id,
            stores_accessed_in_bulk,
        } = reactivity_trackers;
        match self {
            IdbNotification::BulkStoreUpdate { store_name } => {
                stores_accessed_by_id.contains_key(store_name)
                    || stores_accessed_in_bulk.contains(store_name)
            }
            IdbNotification::SingleRecoreUpdate { store_name, id } => stores_accessed_by_id
                .get(store_name)
                .map(|ids| ids.contains(id))
                .unwrap_or(false),
        }
    }
}

pub type IdbNotifier = Box<dyn Fn(IdbNotification)>;

#[allow(unused)]
pub struct SyncEngine {
    pub db: Rc<TypesafeDb<DbStoreMarkers>>,
    domain: Url,
    client: reqwest::Client,
    pub idb_notifiers: Arc<Mutex<Registry<IdbNotifier>>>,
}

const MAX_PER_PAGE: i32 = 100;

#[allow(unused)]
impl SyncEngine {
    pub async fn new(domain: Url) -> SyncResult<Self> {
        let db = TypesafeDb::builder("heimisch".into())
            .with_store::<Issue>()
            .with_store::<User>()
            .with_store::<GithubApp>()
            .with_store::<Repository>()
            .with_store::<Milestone>()
            .with_store::<License>()
            .with_store::<IssueInitialSyncStatus>()
            .with_store::<RepositoryInitialSyncStatus>()
            .with_store::<InstallationAccessTokenRow>()
            .build()
            .await?;

        Ok(Self {
            db: Rc::new(db),
            domain,
            idb_notifiers: Default::default(),
            client: Default::default(),
        })
    }

    async fn get_api_conf(
        &self,
        id: &InstallationId,
    ) -> SyncResult<github_api::apis::configuration::Configuration> {
        let bearer_access_token = Some(format!("Bearer {}", self.get_valid_iac(id).await?.token));
        let conf = github_api::apis::configuration::Configuration {
            user_agent: Some("Heimisch".into()),
            client: self.client.clone(),
            bearer_access_token,
            ..Default::default()
        };
        Ok(conf)
    }

    async fn get_valid_iac(&self, id: &InstallationId) -> SyncResult<InstallationAccessToken> {
        let txn = self
            .db
            .txn()
            .with_store::<InstallationAccessTokenRow>()
            .ro();
        let iac = txn
            .object_store::<InstallationAccessTokenRow>()?
            .get_all()
            .await?
            .into_iter()
            .filter(|iac| {
                if &iac.installation_id != id {
                    return false;
                }

                if let Ordering::Greater = (iac.token.expires_at - Timestamp::now())
                    .compare(1.minute())
                    .unwrap()
                {
                    return true;
                } else {
                    return false;
                }
            })
            .map(|i| i.token)
            .next();

        match iac {
            Some(iac) => Ok(iac),
            None => {
                let payload = GetInstallationAccessTokenPayload {
                    installation_id: *id,
                };
                let resp = GetInstallationAccessTokenEndpoint::make_request(
                    &self.domain,
                    &Default::default(),
                    payload,
                    (),
                )
                .await?;

                let txn = self
                    .db
                    .txn()
                    .with_store::<InstallationAccessTokenRow>()
                    .rw();
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
