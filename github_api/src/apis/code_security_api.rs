/*
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * The version of the OpenAPI document: 1.1.4
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`code_security_slash_attach_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeSecuritySlashAttachConfigurationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`code_security_slash_create_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeSecuritySlashCreateConfigurationError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`code_security_slash_delete_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeSecuritySlashDeleteConfigurationError {
    Status400(models::BasicError),
    Status403(models::BasicError),
    Status404(models::BasicError),
    Status409(models::BasicError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`code_security_slash_detach_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeSecuritySlashDetachConfigurationError {
    Status400(models::BasicError),
    Status403(models::BasicError),
    Status404(models::BasicError),
    Status409(models::BasicError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`code_security_slash_get_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeSecuritySlashGetConfigurationError {
    Status403(models::BasicError),
    Status404(models::BasicError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`code_security_slash_get_configuration_for_repository`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeSecuritySlashGetConfigurationForRepositoryError {
    Status403(models::BasicError),
    Status404(models::BasicError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`code_security_slash_get_configurations_for_org`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeSecuritySlashGetConfigurationsForOrgError {
    Status403(models::BasicError),
    Status404(models::BasicError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`code_security_slash_get_default_configurations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeSecuritySlashGetDefaultConfigurationsError {
    Status403(models::BasicError),
    Status404(models::BasicError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`code_security_slash_get_repositories_for_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeSecuritySlashGetRepositoriesForConfigurationError {
    Status403(models::BasicError),
    Status404(models::BasicError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`code_security_slash_set_configuration_as_default`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeSecuritySlashSetConfigurationAsDefaultError {
    Status403(models::BasicError),
    Status404(models::BasicError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`code_security_slash_update_configuration`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeSecuritySlashUpdateConfigurationError {
    UnknownValue(serde_json::Value),
}

/// Attach a code security configuration to a set of repositories. If the repositories specified are already attached to a configuration, they will be re-attached to the provided configuration.  If insufficient GHAS licenses are available to attach the configuration to a repository, only free features will be enabled.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
pub async fn code_security_slash_attach_configuration(
    configuration: &configuration::Configuration,
    org: &str,
    configuration_id: i32,
    code_security_attach_configuration_request: models::CodeSecurityAttachConfigurationRequest,
) -> Result<serde_json::Value, Error<CodeSecuritySlashAttachConfigurationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/orgs/{org}/code-security/configurations/{configuration_id}/attach",
        local_var_configuration.base_path,
        org = crate::apis::urlencode(org),
        configuration_id = configuration_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&code_security_attach_configuration_request);

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {bearer_access_token}"));
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CodeSecuritySlashAttachConfigurationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a code security configuration in an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
pub async fn code_security_slash_create_configuration(
    configuration: &configuration::Configuration,
    org: &str,
    code_security_create_configuration_request: models::CodeSecurityCreateConfigurationRequest,
) -> Result<models::CodeSecurityConfiguration, Error<CodeSecuritySlashCreateConfigurationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/orgs/{org}/code-security/configurations",
        local_var_configuration.base_path,
        org = crate::apis::urlencode(org)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&code_security_create_configuration_request);

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {bearer_access_token}"));
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CodeSecuritySlashCreateConfigurationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes the desired code security configuration from an organization. Repositories attached to the configuration will retain their settings but will no longer be associated with the configuration.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
pub async fn code_security_slash_delete_configuration(
    configuration: &configuration::Configuration,
    org: &str,
    configuration_id: i32,
) -> Result<(), Error<CodeSecuritySlashDeleteConfigurationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/orgs/{org}/code-security/configurations/{configuration_id}",
        local_var_configuration.base_path,
        org = crate::apis::urlencode(org),
        configuration_id = configuration_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {bearer_access_token}"));
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CodeSecuritySlashDeleteConfigurationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Detach code security configuration(s) from a set of repositories. Repositories will retain their settings but will no longer be associated with the configuration.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
pub async fn code_security_slash_detach_configuration(
    configuration: &configuration::Configuration,
    org: &str,
    code_security_detach_configuration_request: models::CodeSecurityDetachConfigurationRequest,
) -> Result<(), Error<CodeSecuritySlashDetachConfigurationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/orgs/{org}/code-security/configurations/detach",
        local_var_configuration.base_path,
        org = crate::apis::urlencode(org)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&code_security_detach_configuration_request);

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {bearer_access_token}"));
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CodeSecuritySlashDetachConfigurationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Gets a code security configuration available in an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
pub async fn code_security_slash_get_configuration(
    configuration: &configuration::Configuration,
    org: &str,
    configuration_id: i32,
) -> Result<models::CodeSecurityConfiguration, Error<CodeSecuritySlashGetConfigurationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/orgs/{org}/code-security/configurations/{configuration_id}",
        local_var_configuration.base_path,
        org = crate::apis::urlencode(org),
        configuration_id = configuration_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {bearer_access_token}"));
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CodeSecuritySlashGetConfigurationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the code security configuration that manages a repository's code security settings.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
pub async fn code_security_slash_get_configuration_for_repository(
    configuration: &configuration::Configuration,
    owner: &str,
    repo: &str,
) -> Result<
    models::CodeSecurityConfigurationForRepository,
    Error<CodeSecuritySlashGetConfigurationForRepositoryError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/repos/{owner}/{repo}/code-security-configuration",
        local_var_configuration.base_path,
        owner = crate::apis::urlencode(owner),
        repo = crate::apis::urlencode(repo)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {bearer_access_token}"));
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CodeSecuritySlashGetConfigurationForRepositoryError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists all code security configurations available in an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
pub async fn code_security_slash_get_configurations_for_org(
    configuration: &configuration::Configuration,
    org: &str,
    target_type: Option<&str>,
    per_page: Option<i32>,
    before: Option<&str>,
    after: Option<&str>,
) -> Result<
    Vec<models::CodeSecurityConfiguration>,
    Error<CodeSecuritySlashGetConfigurationsForOrgError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/orgs/{org}/code-security/configurations",
        local_var_configuration.base_path,
        org = crate::apis::urlencode(org)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = target_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("target_type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder =
            local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = before {
        local_var_req_builder =
            local_var_req_builder.query(&[("before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = after {
        local_var_req_builder =
            local_var_req_builder.query(&[("after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {bearer_access_token}"));
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CodeSecuritySlashGetConfigurationsForOrgError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists the default code security configurations for an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
pub async fn code_security_slash_get_default_configurations(
    configuration: &configuration::Configuration,
    org: &str,
) -> Result<
    Vec<models::CodeSecurityDefaultConfigurationsInner>,
    Error<CodeSecuritySlashGetDefaultConfigurationsError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/orgs/{org}/code-security/configurations/defaults",
        local_var_configuration.base_path,
        org = crate::apis::urlencode(org)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {bearer_access_token}"));
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CodeSecuritySlashGetDefaultConfigurationsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists the repositories associated with a code security configuration in an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
pub async fn code_security_slash_get_repositories_for_configuration(
    configuration: &configuration::Configuration,
    org: &str,
    configuration_id: i32,
    per_page: Option<i32>,
    before: Option<&str>,
    after: Option<&str>,
    status: Option<&str>,
) -> Result<
    Vec<models::CodeSecurityConfigurationRepositories>,
    Error<CodeSecuritySlashGetRepositoriesForConfigurationError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/orgs/{org}/code-security/configurations/{configuration_id}/repositories",
        local_var_configuration.base_path,
        org = crate::apis::urlencode(org),
        configuration_id = configuration_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = per_page {
        local_var_req_builder =
            local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = before {
        local_var_req_builder =
            local_var_req_builder.query(&[("before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = after {
        local_var_req_builder =
            local_var_req_builder.query(&[("after", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = status {
        local_var_req_builder =
            local_var_req_builder.query(&[("status", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {bearer_access_token}"));
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CodeSecuritySlashGetRepositoriesForConfigurationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets a code security configuration as a default to be applied to new repositories in your organization.  This configuration will be applied to the matching repository type (all, none, public, private and internal) by default when they are created.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
pub async fn code_security_slash_set_configuration_as_default(
    configuration: &configuration::Configuration,
    org: &str,
    configuration_id: i32,
    code_security_set_configuration_as_default_request: models::CodeSecuritySetConfigurationAsDefaultRequest,
) -> Result<
    models::CodeSecuritySetConfigurationAsDefault200Response,
    Error<CodeSecuritySlashSetConfigurationAsDefaultError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/orgs/{org}/code-security/configurations/{configuration_id}/defaults",
        local_var_configuration.base_path,
        org = crate::apis::urlencode(org),
        configuration_id = configuration_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.json(&code_security_set_configuration_as_default_request);

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {bearer_access_token}"));
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CodeSecuritySlashSetConfigurationAsDefaultError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a code security configuration in an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
pub async fn code_security_slash_update_configuration(
    configuration: &configuration::Configuration,
    org: &str,
    configuration_id: i32,
    code_security_update_configuration_request: models::CodeSecurityUpdateConfigurationRequest,
) -> Result<models::CodeSecurityConfiguration, Error<CodeSecuritySlashUpdateConfigurationError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/orgs/{org}/code-security/configurations/{configuration_id}",
        local_var_configuration.base_path,
        org = crate::apis::urlencode(org),
        configuration_id = configuration_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&code_security_update_configuration_request);

    if let Some(ref bearer_access_token) = configuration.bearer_access_token {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::AUTHORIZATION, format!("Bearer {bearer_access_token}"));
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CodeSecuritySlashUpdateConfigurationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
