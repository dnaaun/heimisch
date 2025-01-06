use registry::Registry;
use send_wrapper::SendWrapper;
use std::{marker::PhantomData, sync::Arc};
use typed_transport::TypedTransportTrait;
use typesafe_idb::{ReactivityTrackers, TypesafeDb};
mod conversions;
mod ensure_initial_sync_issues;
mod ensure_initial_sync_repository;
mod fetch_repositorys_for_installation_id;

pub mod typed_transport;

pub mod changes;
mod ensure_initial_sync_issue_comments;
pub mod error;
mod registry;
mod websocket_updates;

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

/// Why? Because `JsonSerdeCodec` from `codee` encodes to / decodes from str, and I want to be able
/// to be able to interpret web socket messages that are in "binary frames" (or whatever the
/// correct terminology) to also be decoded as JSON.
pub struct JsonSerdeToBinaryCodec;

impl<T: serde::de::DeserializeOwned> codee::Decoder<T> for JsonSerdeToBinaryCodec {
    type Error = serde_json::Error;

    type Encoded = [u8];

    fn decode(val: &Self::Encoded) -> Result<T, Self::Error> {
        serde_json::from_slice(val)
    }
}

impl<T: serde::Serialize> codee::Encoder<T> for JsonSerdeToBinaryCodec {
    type Error = serde_json::Error;

    type Encoded = Vec<u8>;

    fn encode(val: &T) -> Result<Self::Encoded, Self::Error> {
        serde_json::to_vec(val)
    }
}

#[derive(Clone)]
pub struct DbSubscription {
    pub original_reactivity_trackers: ReactivityTrackers,
    pub func: Arc<dyn Fn()>,
}
/// Without this isolation, our `impl` definition for the `DbStoreMarkers` type will not have one
/// "defining use."
mod isolate_db_store_markers_impl_type {
    use std::marker::PhantomData;
    use std::rc::Rc;

    use crate::types::label::Label;
    use crate::types::last_webhook_update_at::LastWebhookUpdateAt;
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
    use send_wrapper::SendWrapper;
    use typesafe_idb::{StoreMarker, TypesafeDb};

    use super::registry::Registry;
    use super::typed_transport::TypedTransportTrait;
    use super::DbSubscription;
    use super::{error::SyncResult, SyncEngine};

    pub type DbStoreMarkers = impl StoreMarker<IssueCommentsInitialSyncStatus>
        + StoreMarker<RepositoryInitialSyncStatus>
        + StoreMarker<IssueComment>
        + StoreMarker<InstallationAccessTokenRow>
        + StoreMarker<IssuesInitialSyncStatus>
        + StoreMarker<License>
        + StoreMarker<Label>
        + StoreMarker<Milestone>
        + StoreMarker<Repository>
        + StoreMarker<GithubApp>
        + StoreMarker<User>
        + StoreMarker<Issue>
        + StoreMarker<LastWebhookUpdateAt>;

    impl<W: TypedTransportTrait> SyncEngine<W> {
        pub async fn new(endpoint_client: EndpointClient) -> SyncResult<Self, W> {
            let db = TypesafeDb::builder("heimisch".into())
                .with_store::<Issue>()
                .with_store::<User>()
                .with_store::<GithubApp>()
                .with_store::<Repository>()
                .with_store::<Milestone>()
                .with_store::<License>()
                .with_store::<Label>()
                .with_store::<IssuesInitialSyncStatus>()
                .with_store::<InstallationAccessTokenRow>()
                .with_store::<IssueComment>()
                .with_store::<LastWebhookUpdateAt>()
                .with_store::<IssueCommentsInitialSyncStatus>()
                .with_store::<RepositoryInitialSyncStatus>();

            let db_change_notifiers: Rc<Registry<DbSubscription>> = Default::default();
            let db_change_notifiers2 = db_change_notifiers.clone();
            let db = db
                .with_commit_listener(Rc::new(move |reactivity_trackers| {
                    let orig_trackers = db_change_notifiers2.get();
                    orig_trackers
                        .iter()
                        .filter(|sub| {
                            sub.original_reactivity_trackers
                                .overlaps(reactivity_trackers)
                        })
                        .for_each(|sub| (sub.func)());
                }))
                .build()
                .await?;

            Ok(Self {
                db: Rc::new(db),
                db_subscriptions: SendWrapper::new(db_change_notifiers),
                endpoint_client,
                _transport: PhantomData,
            })
        }
    }
}

pub use isolate_db_store_markers_impl_type::DbStoreMarkers;

pub struct SyncEngine<Transport> {
    pub db: Rc<TypesafeDb<DbStoreMarkers>>,
    pub db_subscriptions: SendWrapper<Rc<Registry<DbSubscription>>>,
    endpoint_client: EndpointClient,
    _transport: PhantomData<Transport>,
}

const MAX_PER_PAGE: i32 = 100;

impl<W: TypedTransportTrait> SyncEngine<W> {
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
