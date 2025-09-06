use std::future::Future;

use github_api::{
    apis::{
        apps_api::{
            AppsSlashGetInstallationError, AppsSlashListReposAccessibleToInstallationError,
        },
        configuration,
        issues_api::{
            IssuesSlashCreateError, IssuesSlashListCommentsForRepoError,
            IssuesSlashListForRepoError,
        },
        users_api::UsersSlashGetAuthenticatedError,
        Error,
    },
    models,
};
use utils::JustSend;

use super::trait_defn::GithubApiTrait;

pub struct GithubApi;

impl GithubApiTrait for GithubApi {
    fn users_slash_get_authenticated(
        &self,
        configuration: &configuration::Configuration,
    ) -> impl Future<
        Output = Result<
            models::UsersGetAuthenticated200Response,
            Error<UsersSlashGetAuthenticatedError>,
        >,
    > + Send
           + Sync {
        JustSend::new(github_api::apis::users_api::users_slash_get_authenticated(
            configuration,
        ))
    }

    fn issues_slash_create(
        &self,
        configuration: &configuration::Configuration,
        owner: &str,
        repo: &str,
        issues_create_request: models::IssuesCreateRequest,
    ) -> impl std::future::Future<Output = Result<models::Issue, Error<IssuesSlashCreateError>>>
           + Send
           + Sync {
        JustSend::new(github_api::apis::issues_api::issues_slash_create(
            configuration,
            owner,
            repo,
            issues_create_request,
        ))
    }

    fn apps_slash_list_repos_accessible_to_installation(
        &self,
        configuration: &configuration::Configuration,
        per_page: Option<i32>,
        page: Option<i32>,
    ) -> impl Future<
        Output = Result<
            models::AppsListReposAccessibleToInstallation200Response,
            Error<AppsSlashListReposAccessibleToInstallationError>,
        >,
    > + Send
           + Sync {
        JustSend::new(
            github_api::apis::apps_api::apps_slash_list_repos_accessible_to_installation(
                configuration,
                per_page,
                page,
            ),
        )
    }

    fn issues_slash_list_comments_for_repo(
        &self,
        configuration: &configuration::Configuration,
        owner: &str,
        repo: &str,
        sort: Option<&str>,
        direction: Option<&str>,
        since: Option<String>,
        per_page: Option<i32>,
        page: Option<i32>,
    ) -> impl Future<
        Output = Result<Vec<models::IssueComment>, Error<IssuesSlashListCommentsForRepoError>>,
    > + Send
           + Sync {
        JustSend::new(
            github_api::apis::issues_api::issues_slash_list_comments_for_repo(
                configuration,
                owner,
                repo,
                sort,
                direction,
                since,
                per_page,
                page,
            ),
        )
    }

    fn apps_slash_get_installation(
        &self,
        configuration: &configuration::Configuration,
        installation_id: i32,
    ) -> impl Future<Output = Result<models::Installation, Error<AppsSlashGetInstallationError>>>
           + Send
           + Sync {
        JustSend::new(github_api::apis::apps_api::apps_slash_get_installation(
            configuration,
            installation_id,
        ))
    }

    fn issues_slash_list_for_repo(
        &self,
        configuration: &configuration::Configuration,
        owner: &str,
        repo: &str,
        milestone: Option<&str>,
        state: Option<&str>,
        assignee: Option<&str>,
        creator: Option<&str>,
        mentioned: Option<&str>,
        labels: Option<&str>,
        sort: Option<&str>,
        direction: Option<&str>,
        since: Option<String>,
        per_page: Option<i32>,
        page: Option<i32>,
    ) -> impl Future<Output = Result<Vec<models::Issue>, Error<IssuesSlashListForRepoError>>> + Send + Sync
    {
        JustSend::new(github_api::apis::issues_api::issues_slash_list_for_repo(
            configuration,
            owner,
            repo,
            milestone,
            state,
            assignee,
            creator,
            mentioned,
            labels,
            sort,
            direction,
            since,
            per_page,
            page,
        ))
    }
}
