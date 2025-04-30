use std::future::Future;
use std::pin::Pin;
/// Without this isolation, our `impl` definition for the `DbStoreMarkers` type will not have one
/// "defining use."
use std::rc::Rc;

use crate::backend_api_trait::BackendApiTrait;
use crate::types::github_app::GithubAppStoreMarker;
use crate::types::installation_access_token_row::InstallationAccessTokenRowStoreMarker;
use crate::types::installation_initial_sync_status::InstallationInitialSyncStatusStoreMarker;
use crate::types::issue::IssueStoreMarker;
use crate::types::issue_comment::IssueCommentStoreMarker;
use crate::types::issue_comment_initial_sync_status::IssueCommentsInitialSyncStatusStoreMarker;
use crate::types::issues_initial_sync_status::IssuesInitialSyncStatusStoreMarker;
use crate::types::label::{Label, LabelStoreMarker};
use crate::types::last_webhook_update_at::{LastWebhookUpdateAt, LastWebhookUpdateAtStoreMarker};
use crate::types::license::LicenseStoreMarker;
use crate::types::milestone::MilestoneStoreMarker;
use crate::types::repository::RepositoryStoreMarker;
use crate::types::repository_initial_sync_status::RepositoryInitialSyncStatusStoreMarker;
use crate::types::user::UserStoreMarker;
use crate::types::{
    github_app::GithubApp, installation_access_token_row::InstallationAccessTokenRow,
    installation_initial_sync_status::InstallationInitialSyncStatus, issue::Issue,
    issue_comment::IssueComment, issue_comment_initial_sync_status::IssueCommentsInitialSyncStatus,
    issues_initial_sync_status::IssuesInitialSyncStatus, license::License, milestone::Milestone,
    repository::Repository, repository_initial_sync_status::RepositoryInitialSyncStatus,
    user::User,
};
use send_wrapper::SendWrapper;
use typesafe_idb::TypesafeDb;
use url::Url;

use super::optimistic::db::DbWithOptimisticChanges;
use super::registry::Registry;
use super::websocket_updates::transport::TransportTrait;
use super::DbSubscription;
use super::{error::SyncResult, SyncEngine};

pub type DbStoreMarkers = (
    InstallationInitialSyncStatusStoreMarker,
    (
        RepositoryInitialSyncStatusStoreMarker,
        (
            IssueCommentsInitialSyncStatusStoreMarker,
            (
                LastWebhookUpdateAtStoreMarker,
                (
                    IssueCommentStoreMarker,
                    (
                        InstallationAccessTokenRowStoreMarker,
                        (
                            IssuesInitialSyncStatusStoreMarker,
                            (
                                LabelStoreMarker,
                                (
                                    LicenseStoreMarker,
                                    (
                                        MilestoneStoreMarker,
                                        (
                                            RepositoryStoreMarker,
                                            (
                                                GithubAppStoreMarker,
                                                (UserStoreMarker, (IssueStoreMarker, ())),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    ),
);

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
            .with_store::<RepositoryInitialSyncStatus>()
            .with_store::<InstallationInitialSyncStatus>();

        let db_subscriptions: Rc<Registry<DbSubscription>> = Default::default();
        let db_subscriptions2 = db_subscriptions.clone();
        let db = DbWithOptimisticChanges::new(
            db,
            Rc::new(move |reactivity_trackers| {
                let orig_trackers = db_subscriptions2.get();
                let matching_trackers = orig_trackers
                    .iter()
                    .filter(|sub| {
                        sub.original_reactivity_trackers
                            .is_affected_by_writes_in(reactivity_trackers)
                    })
                    .collect::<Vec<_>>();

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
