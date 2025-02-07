use std::future::Future;

use crate::{
    apis::{
        configuration, issues_api::IssuesSlashCreateError,
        users_api::UsersSlashGetAuthenticatedError, Error,
    },
    models,
};

/// This is how we do dependency injection / mocking.
pub trait GithubApiTrait {
    fn users_slash_get_authenticated(
        configuration: &configuration::Configuration,
    ) -> impl Future<
        Output = Result<
            models::UsersGetAuthenticated200Response,
            Error<UsersSlashGetAuthenticatedError>,
        >,
    >;
    fn issues_slash_create(
        configuration: &configuration::Configuration,
        owner: &str,
        repo: &str,
        issues_create_request: models::IssuesCreateRequest,
    ) -> impl std::future::Future<Output = Result<models::Issue, Error<IssuesSlashCreateError>>> + Send;
}

pub struct GithubApi;

impl GithubApiTrait for GithubApi {
    fn users_slash_get_authenticated(
        configuration: &configuration::Configuration,
    ) -> impl Future<
        Output = Result<
            models::UsersGetAuthenticated200Response,
            Error<UsersSlashGetAuthenticatedError>,
        >,
    > {
        crate::apis::users_api::users_slash_get_authenticated(configuration)
    }
    async fn issues_slash_create(
        configuration: &configuration::Configuration,
        owner: &str,
        repo: &str,
        issues_create_request: models::IssuesCreateRequest,
    ) -> Result<models::Issue, Error<IssuesSlashCreateError>> {
        crate::apis::issues_api::issues_slash_create(
            configuration,
            owner,
            repo,
            issues_create_request,
        )
        .await
    }
}
