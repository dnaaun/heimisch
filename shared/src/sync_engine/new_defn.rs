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
use super::websocket_updates::transport::TransportTrait;
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

impl<W: TransportTrait, GithubApi> SyncEngine<W, GithubApi> {
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

        let db_subscriptions: Rc<Registry<DbSubscription>> = Default::default();
        let db_subscriptions2 = db_subscriptions.clone();
        let db = DbWithOptimisticChanges::new(
            db,
            Rc::new(move |reactivity_trackers| {
                let orig_trackers = db_subscriptions2.get();
                orig_trackers
                    .iter()
                    .filter(|sub| {
                        sub.original_reactivity_trackers
                            .is_affected_by_writes_in(reactivity_trackers)
                    })
                    .for_each(|sub| (sub.func)());
            }),
        ).await?;

        Ok(Self {
            db: Rc::new(db),
            db_subscriptions: SendWrapper::new(db_subscriptions),
            endpoint_client,
            _transport: PhantomData,
        })
    }
}

