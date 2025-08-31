use github_api::models::IssuesCreateRequest;
use jiff::Timestamp;
use typed_db::RawDbTrait;

use crate::{
    backend_api_trait::BackendApiTrait,
    github_api_trait::GithubApiTrait,
    random::random,
    sync_engine::{error::SyncError, websocket_updates::transport::TransportTrait, SyncEngine},
    types::{
        installation::InstallationId,
        issue::{Issue, IssueId},
        repository::Repository,
        user::User,
    },
};

impl<
        RawDb: RawDbTrait + 'static,
        BackendApi: BackendApiTrait,
        Transport: TransportTrait,
        GithubApi: GithubApiTrait,
    > SyncEngine<RawDb, BackendApi, Transport, GithubApi>
{
    /// Returns the optimistic id of the issue.
    ///
    /// Invariant upheld: The issue number and id will be a negative number for the optimistic issue.
    pub fn create_issue(
        &self,
        installation_id: &InstallationId,
        owner: &User,
        repo: &Repository,
        issues_create_request: IssuesCreateRequest,
    ) -> Result<IssueId, SyncError<Transport, RawDb>> {
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

        let this = SyncEngine::clone(self);
        let installation_id = *installation_id;

        self.db
            .table_rw::<Issue>()
            .create_optimistically(optimistic_issue, async move {
                let conf = this.get_api_conf(&installation_id).await.map_err(|_| ())?;
                let id = this
                    .github_api
                    .issues_slash_create(&conf, &owner_login, &repo_name, issues_create_request)
                    .await
                    .map(|i| IssueId::from(i.id))
                    .map_err(|_| ());
                id
            });

        Ok(issue_id)
    }
}
