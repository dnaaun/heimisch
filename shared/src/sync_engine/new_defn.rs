use std::future::Future;
use std::pin::Pin;
/// Without this isolation, our `impl` definition for the `DbTableMarkers` type will not have one
/// "defining use."
use std::sync::Arc;

use crate::backend_api_trait::BackendApiTrait;
use crate::sync_engine::error::RawDbErrorToSyncError;
use crate::types::github_app::GithubAppTableMarker;
use crate::types::installation_access_token_row::InstallationAccessTokenRowTableMarker;
use crate::types::installation_initial_sync_status::InstallationInitialSyncStatusTableMarker;
use crate::types::issue::IssueTableMarker;
use crate::types::issue_comment::IssueCommentTableMarker;
use crate::types::issue_comment_initial_sync_status::IssueCommentsInitialSyncStatusTableMarker;
use crate::types::issues_initial_sync_status::IssuesInitialSyncStatusTableMarker;
use crate::types::label::{Label, LabelTableMarker};
use crate::types::last_webhook_update_at::{LastWebhookUpdateAt, LastWebhookUpdateAtTableMarker};
use crate::types::license::LicenseTableMarker;
use crate::types::milestone::MilestoneTableMarker;
use crate::types::repository::RepositoryTableMarker;
use crate::types::repository_initial_sync_status::RepositoryInitialSyncStatusTableMarker;
use crate::types::user::UserTableMarker;
use crate::types::{
    github_app::GithubApp, installation_access_token_row::InstallationAccessTokenRow,
    installation_initial_sync_status::InstallationInitialSyncStatus, issue::Issue,
    issue_comment::IssueComment, issue_comment_initial_sync_status::IssueCommentsInitialSyncStatus,
    issues_initial_sync_status::IssuesInitialSyncStatus, license::License, milestone::Milestone,
    repository::Repository, repository_initial_sync_status::RepositoryInitialSyncStatus,
    user::User,
};
use futures::future::BoxFuture;
use typed_db::{Db, RawDbTrait};
use url::Url;

use super::optimistic::db::DbWithOptimisticChanges;
use super::registry::Registry;
use super::websocket_updates::transport::TransportTrait;
use super::DbSubscription;
use super::{error::SyncResult, SyncEngine};

pub type DbTableMarkers = (
    InstallationInitialSyncStatusTableMarker,
    (
        RepositoryInitialSyncStatusTableMarker,
        (
            IssueCommentsInitialSyncStatusTableMarker,
            (
                LastWebhookUpdateAtTableMarker,
                (
                    IssueCommentTableMarker,
                    (
                        InstallationAccessTokenRowTableMarker,
                        (
                            IssuesInitialSyncStatusTableMarker,
                            (
                                LabelTableMarker,
                                (
                                    LicenseTableMarker,
                                    (
                                        MilestoneTableMarker,
                                        (
                                            RepositoryTableMarker,
                                            (
                                                GithubAppTableMarker,
                                                (UserTableMarker, (IssueTableMarker, ())),
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

impl<RawDb: RawDbTrait, BackendApi: BackendApiTrait, Transport: TransportTrait, GithubApi>
    SyncEngine<RawDb, BackendApi, Transport, GithubApi>
{
    pub async fn new<F, Fut>(
        backend_api: Arc<BackendApi>,
        make_transport: F,
        github_api: Arc<GithubApi>,
        db_name: String,
    ) -> SyncResult<Self, Transport, RawDb>
    where
        F: Fn(Url) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Transport, Transport::TransportError>> + Send + Sync + 'static,
    {
        let db = Db::<RawDb, ()>::builder(db_name)
            .with_table::<Issue>()
            .with_table::<User>()
            .with_table::<GithubApp>()
            .with_table::<Repository>()
            .with_table::<Milestone>()
            .with_table::<License>()
            .with_table::<Label>()
            .with_table::<IssuesInitialSyncStatus>()
            .with_table::<InstallationAccessTokenRow>()
            .with_table::<IssueComment>()
            .with_table::<LastWebhookUpdateAt>()
            .with_table::<IssueCommentsInitialSyncStatus>()
            .with_table::<RepositoryInitialSyncStatus>()
            .with_table::<InstallationInitialSyncStatus>();

        let db_subscriptions: Registry<DbSubscription> = Default::default();
        let db_subscriptions2 = db_subscriptions.clone();
        let db = DbWithOptimisticChanges::new(
            db,
            Arc::new(move |reactivity_trackers| {
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
        .await
        .tse()?;

        // Convert the nice generic function into the boxed version we need internally
        let make_transport = Arc::new(move |arg| {
            Box::pin(make_transport(arg))
                as BoxFuture<'static, Result<Transport, Transport::TransportError>>
        });

        Ok(Self {
            db: Arc::new(db),
            db_subscriptions,
            backend_api,
            github_api,
            make_transport,
        })
    }
}
