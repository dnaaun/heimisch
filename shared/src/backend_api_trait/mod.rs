use url::Url;

use crate::{
    endpoints::{
        defns::api::{
            app_installs::create::{
                CreateAppInstallEndpoint, CreateAppInstallPayload, CreateAppInstallResponse,
            },
            auth::finish::{AuthFinishEndpoint, AuthFinishPayload, AuthFinishResponse},
            installations::{
                GetInstallationAccessTokenEndpoint, GetInstallationAccessTokenQueryParams,
                GetInstallationsEndpoint, GetInstallationsResponse,
            },
        },
        endpoint_client::{EndpointClient, OwnApiError},
    },
    types::installation_access_token_row::InstallationAccessToken,
};

#[derive(Clone)]
pub struct BackendApi {
    client: EndpointClient,
}

impl BackendApi {
    pub fn new(client: EndpointClient) -> Self {
        Self { client }
    }
}

#[mockall::automock]
pub trait BackendApiTrait: 'static {
    fn get_domain(&self) -> Url;
    fn get_installations(
        &self,
    ) -> impl std::future::Future<Output = Result<GetInstallationsResponse, OwnApiError>>;

    fn get_installation_access_token(
        &self,
        params: GetInstallationAccessTokenQueryParams,
    ) -> impl std::future::Future<Output = Result<InstallationAccessToken, OwnApiError>>;

    fn create_app_install(
        &self,
        payload: CreateAppInstallPayload,
    ) -> impl std::future::Future<Output = Result<CreateAppInstallResponse, OwnApiError>>;

    fn auth_finish(
        &self,
        payload: AuthFinishPayload,
    ) -> impl std::future::Future<Output = Result<AuthFinishResponse, OwnApiError>>;
}

impl BackendApiTrait for BackendApi {
    fn get_domain(&self) -> Url {
        self.client.domain.clone()
    }

    async fn get_installations(&self) -> Result<GetInstallationsResponse, OwnApiError> {
        self.client
            .make_get_request(GetInstallationsEndpoint, ())
            .await
    }

    async fn get_installation_access_token(
        &self,
        params: GetInstallationAccessTokenQueryParams,
    ) -> Result<InstallationAccessToken, OwnApiError> {
        self.client
            .make_get_request(GetInstallationAccessTokenEndpoint, params)
            .await
    }

    async fn create_app_install(
        &self,
        payload: CreateAppInstallPayload,
    ) -> Result<CreateAppInstallResponse, OwnApiError> {
        self.client
            .make_post_request(CreateAppInstallEndpoint, payload, ())
            .await
    }

    async fn auth_finish(
        &self,
        payload: AuthFinishPayload,
    ) -> Result<AuthFinishResponse, OwnApiError> {
        self.client
            .make_post_request(AuthFinishEndpoint, payload, ())
            .await
    }
}
