use github_api::{github_api_trait::GithubApiTrait, models::IssuesCreateRequest};
use jiff::Timestamp;

use crate::{
    sync_engine::{
        error::{SyncError, SyncErrorSrc},
        SyncEngine, TypedTransportTrait,
    },
    types::{
        installation::InstallationId,
        issue::{Issue, IssueId},
        repository::{Repository, RepositoryId},
        user::{User, UserId},
    },
};

impl<T: TypedTransportTrait, GithubApi: GithubApiTrait> SyncEngine<T, GithubApi> {
    /// Returns the optimistic id of the issue.
    pub async fn create_issue(
        &self,
        installation_id: &InstallationId,
        owner_id: &UserId,
        repo_id: &RepositoryId,
        issues_create_request: IssuesCreateRequest,
    ) -> Result<IssueId, SyncError<T>> {
        // Just leaving a note for myself that when I did
        // self.db.object_store::<User/Repository>(), I got the "transaction is already closed"
        // issue of indexeddb (probgably caused by the fact that indexeddb transactions commit as
        // soon as one returns to the event loop if a callback (or the chain of callbacks, look at
        // MDN/etc for more details) isn't open).
        let txn = self
            .db
            .txn()
            .with_store::<User>()
            .with_store::<Repository>()
            .build();
        let owner = txn
            .object_store::<User>()
            .unwrap()
            .no_optimism_get(owner_id)
            .await?
            .ok_or_else(|| SyncErrorSrc::<T>::DataModel("no user".into()))?
            .login;
        let repo = txn
            .object_store::<Repository>()?
            .no_optimism_get(repo_id)
            .await?
            .ok_or_else(|| SyncErrorSrc::<T>::DataModel("repo not found".into()))?
            .name;

        let conf = self.get_api_conf(installation_id).await?;

        let now = Timestamp::now();
        let issue_id = IssueId::default();
        let optimistic_issue = Issue {
            id: issue_id,
            repository_id: *repo_id,
            user_id: Some(*owner_id).into(),
            body: issues_create_request.body.clone().into(),
            body_text: issues_create_request.body.clone().into(),
            body_html: issues_create_request.body.clone().into(),
            title: (match &issues_create_request.title {
                github_api::models::IssuesCreateRequestTitle::String(t) => t.clone(),
                github_api::models::IssuesCreateRequestTitle::Integer(i) => i.to_string(),
            })
            .into(),
            created_at: now.into(),
            updated_at: now.into(),
            ..Default::default()
        };

        self.db
            .object_store_rw::<Issue>()?
            .create(optimistic_issue, async move {
                GithubApi::issues_slash_create(&conf, &owner, &repo, issues_create_request)
                    .await
                    .map(|i| IssueId::from(i.id))
                    .map_err(|_| ())
            },
            |id| {
                tracing::info!("issue created: {:?}", id);
            },
        );

        Ok(issue_id)
    }
}
