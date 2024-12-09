use parking_lot::Mutex;
use registry::Registry;
use typesafe_idb::{ReactivityTrackers, TypesafeDb};
mod conversions;
mod ensure_initial_sync_issues;
mod ensure_initial_sync_one_repository;
mod ensure_initial_sync_repositories;
mod kick_off;

pub mod changes;
mod ensure_initial_sync_issue_comments;
pub mod error;
mod registry;

use std::{cmp::Ordering, rc::Rc, sync::Arc};

use crate::{
    endpoints::{
        defns::api::installations::{
            GetInstallationAccessTokenEndpoint, GetInstallationAccessTokenPayload,
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

/// Without this isolation, our `impl` definition for the `DbStoreMarkers` type will not have one
/// "defining use."
mod isolate_db_store_markers_impl_type {
    use std::rc::Rc;

    use crate::{
        endpoints::endpoint_client::EndpointClient,
        types::{
            github_app::GithubApp, installation_access_token_row::InstallationAccessTokenRow,
            issue::Issue, issue_comment::IssueComment,
            issue_comment_initial_sync_status::IssueCommentInitialSyncStatus,
            issue_initial_sync_status::IssueInitialSyncStatus, license::License,
            milestone::Milestone, repository::Repository,
            repository_initial_sync_status::RepositoryInitialSyncStatus, user::User,
        },
    };
    use typesafe_idb::{StoreMarker, TypesafeDb};

    use super::{error::SyncResult, SyncEngine};

    pub type DbStoreMarkers = impl StoreMarker<IssueCommentInitialSyncStatus>
        + StoreMarker<IssueComment>
        + StoreMarker<InstallationAccessTokenRow>
        + StoreMarker<RepositoryInitialSyncStatus>
        + StoreMarker<IssueInitialSyncStatus>
        + StoreMarker<License>
        + StoreMarker<Milestone>
        + StoreMarker<Repository>
        + StoreMarker<GithubApp>
        + StoreMarker<User>
        + StoreMarker<Issue>;

    impl SyncEngine {
        pub async fn new(endpoint_client: EndpointClient) -> SyncResult<Self> {
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
                .with_store::<IssueComment>()
                .with_store::<IssueCommentInitialSyncStatus>()
                .build()
                .await?;

            Ok(Self {
                db: Rc::new(db),
                idb_notifiers: Default::default(),
                endpoint_client,
            })
        }
    }
}

pub use isolate_db_store_markers_impl_type::DbStoreMarkers;

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
    pub idb_notifiers: Arc<Mutex<Registry<IdbNotifier>>>,
    endpoint_client: EndpointClient,
}

const MAX_PER_PAGE: i32 = 100;

impl SyncEngine {
    async fn get_api_conf(
        &self,
        id: &InstallationId,
    ) -> SyncResult<github_api::apis::configuration::Configuration> {
        let bearer_access_token = Some(self.get_valid_iac(id).await?.token);
        let conf = github_api::apis::configuration::Configuration {
            user_agent: Some("Heimisch".into()),
            client: self.endpoint_client.client.clone(),
            bearer_access_token,
            base_path: "https://api.github.com".parse().expect(""),
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
                let payload = GetInstallationAccessTokenPayload {
                    installation_id: *id,
                };
                let resp = self
                    .endpoint_client
                    .make_request(GetInstallationAccessTokenEndpoint, payload, ())
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
