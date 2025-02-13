use github_api::{github_api_trait::GithubApiTrait, models::IssuesCreateRequest};
use jiff::Timestamp;

use crate::{
    random::random, sync_engine::{
        error::SyncError,
        SyncEngine, 
    }, types::{
        installation::InstallationId,
        issue::{Issue, IssueId},
        repository::Repository,
        user::User,
    }, sync_engine::websocket_updates::transport::TransportTrait,
};

impl<T: TransportTrait, GithubApi: GithubApiTrait> SyncEngine<T, GithubApi> {
    /// Returns the optimistic id of the issue.
    /// 
    /// Invariant upheld: The issue number and id will be a negative number for the optimistic issue.
    pub async fn create_issue(
        &self,
        installation_id: &InstallationId,
        owner: &User,
         repo: &Repository,
        issues_create_request: IssuesCreateRequest,
    ) -> Result<IssueId, SyncError<T>> {
        let conf = self.get_api_conf(installation_id).await?;

        let now = Timestamp::now();
        let issue_id = IssueId::default();
        let optimistic_issue = Issue {
            id: issue_id,
            repository_id: repo.id,
            user_id: Some(owner.id).into(),
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
            number: -i64::from(random()), // Negative because the number will never be negative in the real/non-optimistic world.
            ..Default::default()
        };

        let owner_login = owner.login.clone();
        let repo_name = repo.name.clone();
        
        self.db
            .object_store_rw::<Issue>()?
            .create(optimistic_issue, async move {
                GithubApi::issues_slash_create(&conf, &owner_login, &repo_name, issues_create_request)
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