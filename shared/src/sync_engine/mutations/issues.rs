use github_api::{apis::issues_api::issues_slash_create, models::IssuesCreateRequest};

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

impl<T: TypedTransportTrait> SyncEngine<T> {
    pub async fn create_issue(
        &self,
        installation_id: &InstallationId,
        owner_id: &UserId,
        repo_id: &RepositoryId,
        issues_create_request: IssuesCreateRequest,
    ) -> Result<(), SyncError<T>> {
        let owner = self
            .db
            .object_store::<User>()?
            .no_optimism_get(owner_id)
            .await?
            .ok_or_else(|| SyncErrorSrc::<T>::DataModel("no user".into()))?
            .login;
        let repo = self
            .db
            .object_store::<Repository>()?
            .no_optimism_get(repo_id)
            .await?
            .ok_or_else(|| SyncErrorSrc::<T>::DataModel("repo not found".into()))?
            .name;

        let conf = self.get_api_conf(installation_id).await?;

        let mut optimistic_issue = Issue::default();
        optimistic_issue.repository_id = repo_id.clone();
        optimistic_issue.user_id = Some(owner_id.clone()).into();
        optimistic_issue.body = issues_create_request.body.clone().into();
        optimistic_issue.body_text = issues_create_request.body.clone().into();
        optimistic_issue.body_html = issues_create_request.body.clone().into();
        optimistic_issue.title = (match &issues_create_request.title {
            github_api::models::IssuesCreateRequestTitle::String(t) => t.clone(),
            github_api::models::IssuesCreateRequestTitle::Integer(i) => i.to_string(),
        })
        .into();

        self.db
            .object_store_rw::<Issue>()?
            .create(optimistic_issue, async move {
                issues_slash_create(&conf, &owner, &repo, issues_create_request)
                    .await
                    .map(|i| IssueId::from(i.id))
                    .map_err(|_| ())
            });

        Ok(())
    }
}
