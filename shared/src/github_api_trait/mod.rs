use std::{future::Future, sync::Arc};

use send_wrapper::SendWrapper;

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

/// This is how we do dependency injection / mocking.
#[cfg_attr(test, mockall::automock)]
pub trait GithubApiTrait: Send + Sync + 'static {
    fn users_slash_get_authenticated(
        &self,
        configuration: &configuration::Configuration,
    ) -> impl Future<
        Output = Result<
            models::UsersGetAuthenticated200Response,
            Error<UsersSlashGetAuthenticatedError>,
        >,
    > + Send
           + Sync;
    fn issues_slash_create(
        &self,
        configuration: &configuration::Configuration,
        owner: &str,
        repo: &str,
        issues_create_request: models::IssuesCreateRequest,
    ) -> impl std::future::Future<Output = Result<models::Issue, Error<IssuesSlashCreateError>>>
           + Send
           + Sync;
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
           + Sync;
    fn issues_slash_list_comments_for_repo<'a>(
        &self,
        configuration: &configuration::Configuration,
        owner: &str,
        repo: &str,
        sort: Option<&'a str>,
        direction: Option<&'a str>,
        since: Option<String>,
        per_page: Option<i32>,
        page: Option<i32>,
    ) -> impl Future<
        Output = Result<Vec<models::IssueComment>, Error<IssuesSlashListCommentsForRepoError>>,
    > + Send
           + Sync;
    fn apps_slash_get_installation(
        &self,
        configuration: &configuration::Configuration,
        installation_id: i32,
    ) -> impl Future<Output = Result<models::Installation, Error<AppsSlashGetInstallationError>>>
           + Send
           + Sync;
    fn issues_slash_list_for_repo<'a>(
        &self,
        configuration: &configuration::Configuration,
        owner: &str,
        repo: &str,
        milestone: Option<&'a str>,
        state: Option<&'a str>,
        assignee: Option<&'a str>,
        creator: Option<&'a str>,
        mentioned: Option<&'a str>,
        labels: Option<&'a str>,
        sort: Option<&'a str>,
        direction: Option<&'a str>,
        since: Option<String>,
        per_page: Option<i32>,
        page: Option<i32>,
    ) -> impl Future<Output = Result<Vec<models::Issue>, Error<IssuesSlashListForRepoError>>> + Send + Sync;
}

pub struct GithubApi;

impl<T: 'static> GithubApiTrait for Arc<T>
where
    T: GithubApiTrait,
{
    fn users_slash_get_authenticated(
        &self,
        configuration: &configuration::Configuration,
    ) -> impl Future<
        Output = Result<
            models::UsersGetAuthenticated200Response,
            Error<UsersSlashGetAuthenticatedError>,
        >,
    > {
        T::users_slash_get_authenticated(self, configuration)
    }

    fn issues_slash_create(
        &self,
        configuration: &configuration::Configuration,
        owner: &str,
        repo: &str,
        issues_create_request: models::IssuesCreateRequest,
    ) -> impl Future<Output = Result<models::Issue, Error<IssuesSlashCreateError>>> + Send + Sync
    {
        T::issues_slash_create(self, configuration, owner, repo, issues_create_request)
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
    > {
        T::apps_slash_list_repos_accessible_to_installation(self, configuration, per_page, page)
    }

    fn issues_slash_list_comments_for_repo<'a>(
        &self,
        configuration: &configuration::Configuration,
        owner: &str,
        repo: &str,
        sort: Option<&'a str>,
        direction: Option<&'a str>,
        since: Option<String>,
        per_page: Option<i32>,
        page: Option<i32>,
    ) -> impl Future<
        Output = Result<Vec<models::IssueComment>, Error<IssuesSlashListCommentsForRepoError>>,
    > {
        T::issues_slash_list_comments_for_repo(
            self,
            configuration,
            owner,
            repo,
            sort,
            direction,
            since,
            per_page,
            page,
        )
    }

    fn apps_slash_get_installation(
        &self,
        configuration: &configuration::Configuration,
        installation_id: i32,
    ) -> impl Future<Output = Result<models::Installation, Error<AppsSlashGetInstallationError>>>
    {
        T::apps_slash_get_installation(self, configuration, installation_id)
    }

    fn issues_slash_list_for_repo<'a>(
        &self,
        configuration: &configuration::Configuration,
        owner: &str,
        repo: &str,
        milestone: Option<&'a str>,
        state: Option<&'a str>,
        assignee: Option<&'a str>,
        creator: Option<&'a str>,
        mentioned: Option<&'a str>,
        labels: Option<&'a str>,
        sort: Option<&'a str>,
        direction: Option<&'a str>,
        since: Option<String>,
        per_page: Option<i32>,
        page: Option<i32>,
    ) -> impl Future<Output = Result<Vec<models::Issue>, Error<IssuesSlashListForRepoError>>> + Send + Sync
    {
        T::issues_slash_list_for_repo(
            self,
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
        )
    }
}

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
        SendWrapper::new(github_api::apis::users_api::users_slash_get_authenticated(
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
        SendWrapper::new(github_api::apis::issues_api::issues_slash_create(
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
        SendWrapper::new(
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
        SendWrapper::new(
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
        SendWrapper::new(github_api::apis::apps_api::apps_slash_get_installation(
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
        SendWrapper::new(github_api::apis::issues_api::issues_slash_list_for_repo(
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
