use std::future::Future;
use std::pin::Pin;
/// Without this isolation, our `impl` definition for the `DbStoreMarkers` type will not have one
/// "defining use."
use std::rc::Rc;

use crate::backend_api_trait::BackendApiTrait;
use crate::types::label::Label;
use crate::types::last_webhook_update_at::LastWebhookUpdateAt;
use crate::types::{
    github_app::GithubApp, installation_access_token_row::InstallationAccessTokenRow, issue::Issue,
    issue_comment::IssueComment, issue_comment_initial_sync_status::IssueCommentsInitialSyncStatus,
    issues_initial_sync_status::IssuesInitialSyncStatus, license::License, milestone::Milestone,
    repository::Repository, repository_initial_sync_status::RepositoryInitialSyncStatus,
    user::User,
};
use send_wrapper::SendWrapper;
use typesafe_idb::{StoreMarker, TypesafeDb};
use url::Url;

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

impl<BackendApi: BackendApiTrait, Transport: TransportTrait, GithubApi>
    SyncEngine<BackendApi, Transport, GithubApi>
{
    pub async fn new<F, Fut>(
        backend_api: Rc<BackendApi>,
        make_transport: F,
        github_api: Rc<GithubApi>,
    ) -> SyncResult<Self, Transport>
    where
        F: Fn(Url) -> Fut + 'static,
        Fut: Future<Output = Result<Transport, Transport::TransportError>> + 'static,
    {
        // Convert the nice generic function into the boxed version we need internally
        let make_transport =
            Rc::new(move |url| Box::pin(make_transport(url)) as Pin<Box<dyn Future<Output = _>>>);

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
                let matching_trackers = orig_trackers
                    .iter()
                    .filter(|sub| {
                        tracing::trace!(
                            "checking if {:?} is affected by {:?}",
                            sub.original_reactivity_trackers,
                            reactivity_trackers
                        );
                        sub.original_reactivity_trackers
                            .is_affected_by_writes_in(reactivity_trackers)
                    })
                    .collect::<Vec<_>>();

                tracing::trace!(
                    "matching_trackers: {:?}",
                    matching_trackers
                        .iter()
                        .map(|sub| sub.original_reactivity_trackers.clone())
                        .collect::<Vec<_>>()
                );

                matching_trackers.into_iter().for_each(|sub| (sub.func)());
            }),
        )
        .await?;

        Ok(Self {
            db: Rc::new(db),
            db_subscriptions: SendWrapper::new(db_subscriptions),
            backend_api,
            github_api,
            make_transport,
        })
    }
}
