/// Without this isolation, our `impl` definition for the `DbStoreMarkers` type will not have one
/// "defining use."

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

use super::optimistic::db::DbWithOptimisticChanges;
use super::registry::Registry;
use super::websocket_updates::typed_transport::TypedTransportTrait;
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
    + StoreMarker<LastWebhookUpdateAt>
    + Default;

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
        let db = DbWithOptimisticChanges::new(
            db.build().await?,
            Rc::new(move |reactivity_trackers| {
                let orig_trackers = db_change_notifiers2.get();
                orig_trackers
                    .iter()
                    .filter(|sub| {
                        sub.original_reactivity_trackers
                            .is_affected_by_writes_in(reactivity_trackers)
                    })
                    .for_each(|sub| (sub.func)());
            }),
        );

        Ok(Self {
            db: Rc::new(db),
            db_subscriptions: SendWrapper::new(db_change_notifiers),
            endpoint_client,
            _transport: PhantomData,
        })
    }
}

