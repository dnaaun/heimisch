use super::GithubApiTrait;

#[derive(bon::Builder)]
pub struct MockGithubApi {
    #[builder(default = Box::new(|_| panic!("mock not implemented")))]
    pub users_slash_get_authenticated_mock: Box<
        dyn Fn(
                &github_api::apis::configuration::Configuration,
            ) -> Result<
                github_api::models::UsersGetAuthenticated200Response,
                github_api::apis::Error<
                    github_api::apis::users_api::UsersSlashGetAuthenticatedError,
                >,
            > + Send
            + Sync
            + 'static,
    >,

    #[builder(default = Box::new(|_, _, _, _| panic!("mock not implemented")))]
    pub issues_slash_create_mock: Box<
        dyn Fn(
                &github_api::apis::configuration::Configuration,
                &str,
                &str,
                github_api::models::IssuesCreateRequest,
            ) -> Result<
                github_api::models::Issue,
                github_api::apis::Error<github_api::apis::issues_api::IssuesSlashCreateError>,
            > + Send
            + Sync
            + 'static,
    >,

    #[builder(default = Box::new(|_, _, _| panic!("mock not implemented")))]
    pub apps_slash_list_repos_accessible_to_installation_mock: Box<
        dyn Fn(
                &github_api::apis::configuration::Configuration,
                Option<i32>,
                Option<i32>,
            ) -> Result<
                github_api::models::AppsListReposAccessibleToInstallation200Response,
                github_api::apis::Error<
                    github_api::apis::apps_api::AppsSlashListReposAccessibleToInstallationError,
                >,
            > + Send
            + Sync
            + 'static,
    >,

    #[builder(default = Box::new(|_, _, _, _, _, _, _, _| panic!("mock not implemented")))]
    pub issues_slash_list_comments_for_repo_mock: Box<
        dyn Fn(
                &github_api::apis::configuration::Configuration,
                &str,
                &str,
                Option<&str>,
                Option<&str>,
                Option<String>,
                Option<i32>,
                Option<i32>,
            ) -> Result<
                Vec<github_api::models::IssueComment>,
                github_api::apis::Error<
                    github_api::apis::issues_api::IssuesSlashListCommentsForRepoError,
                >,
            > + Send
            + Sync
            + 'static,
    >,

    #[builder(default = Box::new(|_, _| panic!("mock not implemented")))]
    pub apps_slash_get_installation_mock: Box<
        dyn Fn(
                &github_api::apis::configuration::Configuration,
                i32,
            ) -> Result<
                github_api::models::Installation,
                github_api::apis::Error<github_api::apis::apps_api::AppsSlashGetInstallationError>,
            > + Send
            + Sync
            + 'static,
    >,

    #[builder(default = Box::new(|_, _, _, _, _, _, _, _, _, _, _, _, _, _| panic!("mock not implemented")))]
    pub issues_slash_list_for_repo_mock: Box<
        dyn Fn(
                &github_api::apis::configuration::Configuration,
                &str,
                &str,
                Option<&str>,
                Option<&str>,
                Option<&str>,
                Option<&str>,
                Option<&str>,
                Option<&str>,
                Option<&str>,
                Option<&str>,
                Option<String>,
                Option<i32>,
                Option<i32>,
            ) -> Result<
                Vec<github_api::models::Issue>,
                github_api::apis::Error<github_api::apis::issues_api::IssuesSlashListForRepoError>,
            > + Send
            + Sync
            + 'static,
    >,
}

impl GithubApiTrait for MockGithubApi {
    async fn users_slash_get_authenticated(
        &self,
        configuration: &github_api::apis::configuration::Configuration,
    ) -> Result<
        github_api::models::UsersGetAuthenticated200Response,
        github_api::apis::Error<github_api::apis::users_api::UsersSlashGetAuthenticatedError>,
    > {
        (self.users_slash_get_authenticated_mock)(configuration)
    }

    async fn issues_slash_create(
        &self,
        configuration: &github_api::apis::configuration::Configuration,
        owner: &str,
        repo: &str,
        issues_create_request: github_api::models::IssuesCreateRequest,
    ) -> Result<
        github_api::models::Issue,
        github_api::apis::Error<github_api::apis::issues_api::IssuesSlashCreateError>,
    > {
        (self.issues_slash_create_mock)(configuration, owner, repo, issues_create_request)
    }

    async fn apps_slash_list_repos_accessible_to_installation(
        &self,
        configuration: &github_api::apis::configuration::Configuration,
        per_page: Option<i32>,
        page: Option<i32>,
    ) -> Result<
        github_api::models::AppsListReposAccessibleToInstallation200Response,
        github_api::apis::Error<
            github_api::apis::apps_api::AppsSlashListReposAccessibleToInstallationError,
        >,
    > {
        (self.apps_slash_list_repos_accessible_to_installation_mock)(configuration, per_page, page)
    }

    async fn issues_slash_list_comments_for_repo(
        &self,
        configuration: &github_api::apis::configuration::Configuration,
        owner: &str,
        repo: &str,
        sort: Option<&str>,
        direction: Option<&str>,
        since: Option<String>,
        per_page: Option<i32>,
        page: Option<i32>,
    ) -> Result<
        Vec<github_api::models::IssueComment>,
        github_api::apis::Error<github_api::apis::issues_api::IssuesSlashListCommentsForRepoError>,
    > {
        (self.issues_slash_list_comments_for_repo_mock)(
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

    async fn apps_slash_get_installation(
        &self,
        configuration: &github_api::apis::configuration::Configuration,
        installation_id: i32,
    ) -> Result<
        github_api::models::Installation,
        github_api::apis::Error<github_api::apis::apps_api::AppsSlashGetInstallationError>,
    > {
        (self.apps_slash_get_installation_mock)(configuration, installation_id)
    }

    async fn issues_slash_list_for_repo(
        &self,
        configuration: &github_api::apis::configuration::Configuration,
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
    ) -> Result<
        Vec<github_api::models::Issue>,
        github_api::apis::Error<github_api::apis::issues_api::IssuesSlashListForRepoError>,
    > {
        (self.issues_slash_list_for_repo_mock)(
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
