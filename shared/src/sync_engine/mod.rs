use kick_off::JsonSerdeToBinaryCodec;
use parking_lot::Mutex;
use registry::Registry;
use std::{fmt::Debug, marker::PhantomData};
use typesafe_idb::{ReactivityTrackers, TypesafeDb};
mod conversions;
mod ensure_initial_sync_issues;
mod ensure_initial_sync_repository;
mod fetch_repositorys_for_installation_id;

mod typed_websocket_client;
pub use typed_websocket_client::TypedWebsocketClient;

pub mod changes;
mod ensure_initial_sync_issue_comments;
pub mod error;
mod kick_off;
mod registry;

use std::{cmp::Ordering, rc::Rc, sync::Arc};

use crate::{
    endpoints::{
        defns::api::{
            installations::{
                GetInstallationAccessTokenEndpoint, GetInstallationAccessTokenPayload,
            },
            websocket_updates::{ClientMsg, ServerMsg},
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

pub trait WSClient: TypedWebsocketClient<ClientMsg, ServerMsg, JsonSerdeToBinaryCodec> {}
impl<W> WSClient for W
where
    W: TypedWebsocketClient<ClientMsg, ServerMsg, JsonSerdeToBinaryCodec>,
    W::Error: Debug,
{
}

/// Without this isolation, our `impl` definition for the `DbStoreMarkers` type will not have one
/// "defining use."
mod isolate_db_store_markers_impl_type {
    use std::rc::Rc;
    use std::{fmt::Debug, marker::PhantomData};

    use crate::{
        endpoints::endpoint_client::EndpointClient,
        types::{
            github_app::GithubApp, installation_access_token_row::InstallationAccessTokenRow,
            issue::Issue, issue_comment::IssueComment,
            issue_comment_initial_sync_status::IssueCommentsInitialSyncStatus,
            issues_initial_sync_status::IssuesInitialSyncStatus, license::License,
            milestone::Milestone, repository::Repository,
            repository_initial_sync_status::RepositoryInitialSyncStatus, user::User,
        },
    };
    use typesafe_idb::{StoreMarker, TypesafeDb};

    use super::{error::SyncResult, SyncEngine, WSClient};

    pub type DbStoreMarkers = impl StoreMarker<IssueCommentsInitialSyncStatus>
        + StoreMarker<RepositoryInitialSyncStatus>
        + StoreMarker<IssueComment>
        + StoreMarker<InstallationAccessTokenRow>
        + StoreMarker<IssuesInitialSyncStatus>
        + StoreMarker<License>
        + StoreMarker<Milestone>
        + StoreMarker<Repository>
        + StoreMarker<GithubApp>
        + StoreMarker<User>
        + StoreMarker<Issue>;

    impl<W: WSClient> SyncEngine<W> {
        pub async fn new(endpoint_client: EndpointClient) -> SyncResult<Self, W::Error>
        where
            W::Error: Debug,
        {
            let db = TypesafeDb::builder("heimisch".into())
                .with_store::<Issue>()
                .with_store::<User>()
                .with_store::<GithubApp>()
                .with_store::<Repository>()
                .with_store::<Milestone>()
                .with_store::<License>()
                .with_store::<IssuesInitialSyncStatus>()
                .with_store::<InstallationAccessTokenRow>()
                .with_store::<IssueComment>()
                .with_store::<IssueCommentsInitialSyncStatus>()
                .with_store::<RepositoryInitialSyncStatus>()
                .build()
                .await?;

            Ok(Self {
                db: Rc::new(db),
                idb_notifiers: Default::default(),
                endpoint_client,
                _ws_client: PhantomData,
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

pub struct SyncEngine<WSClient> {
    pub db: Rc<TypesafeDb<DbStoreMarkers>>,
    pub idb_notifiers: Arc<Mutex<Registry<IdbNotifier>>>,
    endpoint_client: EndpointClient,
    _ws_client: PhantomData<WSClient>,
}

const MAX_PER_PAGE: i32 = 100;

impl<W: WSClient> SyncEngine<W> {
    async fn get_api_conf(
        &self,
        id: &InstallationId,
    ) -> SyncResult<github_api::apis::configuration::Configuration, W::Error> {
        let bearer_access_token = Some(self.get_valid_iac(id).await?.token);
        let conf = github_api::apis::configuration::Configuration {
            user_agent: Some("Heimisch".into()),
            client: self.endpoint_client.client.clone(),
            bearer_access_token,
            base_path: "https://api.github.com".parse().expect(""),
        };
        Ok(conf)
    }

    async fn get_valid_iac(
        &self,
        id: &InstallationId,
    ) -> SyncResult<InstallationAccessToken, W::Error> {
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
