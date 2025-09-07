use std::sync::Arc;

use crate::backend_api_trait::BackendApiTrait;
use crate::sync_engine::error::RawDbErrorToSyncError;
use crate::types::label::Label;
use crate::types::last_webhook_update_at::LastWebhookUpdateAt;
use crate::types::{
    github_app::GithubApp, installation_access_token_row::InstallationAccessTokenRow,
    installation_initial_sync_status::InstallationInitialSyncStatus, issue::Issue,
    issue_comment::IssueComment, issue_comment_initial_sync_status::IssueCommentsInitialSyncStatus,
    issues_initial_sync_status::IssuesInitialSyncStatus, license::License, milestone::Milestone,
    repository::Repository, repository_initial_sync_status::RepositoryInitialSyncStatus,
    user::User,
};
use bon::bon;
use futures::future::BoxFuture;
use typed_db::{Db, RawDbTrait};
use url::Url;

use super::optimistic::db::DbWithOptimisticChanges;
use super::registry::Registry;
use super::websocket_updates::transport::TransportTrait;
use super::DbSubscription;
use super::{error::SyncResult, SyncEngine};

#[bon]
impl<RawDb, BackendApi, Transport, GithubApi> SyncEngine<RawDb, BackendApi, Transport, GithubApi>
where
    RawDb: RawDbTrait,
    BackendApi: BackendApiTrait,
    Transport: TransportTrait,
{
    #[builder]
    pub async fn new(
        backend_api: Arc<BackendApi>,
        github_api: GithubApi,
        db_name: String,
        make_transport: Arc<
            dyn Fn(Url) -> BoxFuture<'static, Result<Transport, Transport::TransportError>>
                + Send
                + Sync,
        >,
    ) -> SyncResult<Self, Transport, RawDb> {
        let db = Db::<RawDb>::builder(db_name)
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

        Ok(Self {
            db: Arc::new(db),
            db_subscriptions,
            backend_api,
            github_api: Arc::new(github_api),
            make_transport,
        })
    }
}
