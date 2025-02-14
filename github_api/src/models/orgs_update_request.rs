/*
 * GitHub v3 REST API
 *
 * GitHub's v3 REST API.
 *
 * The version of the OpenAPI document: 1.1.4
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrgsUpdateRequest {
    /// Billing email address. This address is not publicized.
    #[serde(rename = "billing_email", skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,
    /// The company name.
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// The publicly visible email address.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The Twitter username of the company.
    #[serde(rename = "twitter_username", skip_serializing_if = "Option::is_none")]
    pub twitter_username: Option<String>,
    /// The location.
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// The shorthand name of the company.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the company. The maximum size is 160 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether an organization can use organization projects.
    #[serde(
        rename = "has_organization_projects",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_organization_projects: Option<bool>,
    /// Whether repositories that belong to the organization can use repository projects.
    #[serde(
        rename = "has_repository_projects",
        skip_serializing_if = "Option::is_none"
    )]
    pub has_repository_projects: Option<bool>,
    /// Default permission level members have for organization repositories.
    #[serde(
        rename = "default_repository_permission",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_repository_permission: Option<DefaultRepositoryPermission>,
    /// Whether of non-admin organization members can create repositories. **Note:** A parameter can override this parameter. See `members_allowed_repository_creation_type` in this table for details.
    #[serde(
        rename = "members_can_create_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub members_can_create_repositories: Option<bool>,
    /// Whether organization members can create internal repositories, which are visible to all enterprise members. You can only allow members to create internal repositories if your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. For more information, see \"[Restricting repository creation in your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)\" in the GitHub Help documentation.
    #[serde(
        rename = "members_can_create_internal_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub members_can_create_internal_repositories: Option<bool>,
    /// Whether organization members can create private repositories, which are visible to organization members with permission. For more information, see \"[Restricting repository creation in your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)\" in the GitHub Help documentation.
    #[serde(
        rename = "members_can_create_private_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub members_can_create_private_repositories: Option<bool>,
    /// Whether organization members can create public repositories, which are visible to anyone. For more information, see \"[Restricting repository creation in your organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/restricting-repository-creation-in-your-organization)\" in the GitHub Help documentation.
    #[serde(
        rename = "members_can_create_public_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub members_can_create_public_repositories: Option<bool>,
    /// Specifies which types of repositories non-admin organization members can create. `private` is only available to repositories that are part of an organization on GitHub Enterprise Cloud.  **Note:** This parameter is closing down and will be removed in the future. Its return value ignores internal repositories. Using this parameter overrides values set in `members_can_create_repositories`. See the parameter deprecation notice in the operation description for details.
    #[serde(
        rename = "members_allowed_repository_creation_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub members_allowed_repository_creation_type: Option<MembersAllowedRepositoryCreationType>,
    /// Whether organization members can create GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(
        rename = "members_can_create_pages",
        skip_serializing_if = "Option::is_none"
    )]
    pub members_can_create_pages: Option<bool>,
    /// Whether organization members can create public GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(
        rename = "members_can_create_public_pages",
        skip_serializing_if = "Option::is_none"
    )]
    pub members_can_create_public_pages: Option<bool>,
    /// Whether organization members can create private GitHub Pages sites. Existing published sites will not be impacted.
    #[serde(
        rename = "members_can_create_private_pages",
        skip_serializing_if = "Option::is_none"
    )]
    pub members_can_create_private_pages: Option<bool>,
    /// Whether organization members can fork private organization repositories.
    #[serde(
        rename = "members_can_fork_private_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub members_can_fork_private_repositories: Option<bool>,
    /// Whether contributors to organization repositories are required to sign off on commits they make through GitHub's web interface.
    #[serde(
        rename = "web_commit_signoff_required",
        skip_serializing_if = "Option::is_none"
    )]
    pub web_commit_signoff_required: Option<bool>,
    #[serde(rename = "blog", skip_serializing_if = "Option::is_none")]
    pub blog: Option<String>,
    /// **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether GitHub Advanced Security is automatically enabled for new repositories and repositories transferred to this organization.  To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(
        rename = "advanced_security_enabled_for_new_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub advanced_security_enabled_for_new_repositories: Option<bool>,
    /// **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether Dependabot alerts are automatically enabled for new repositories and repositories transferred to this organization.  To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(
        rename = "dependabot_alerts_enabled_for_new_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub dependabot_alerts_enabled_for_new_repositories: Option<bool>,
    /// **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether Dependabot security updates are automatically enabled for new repositories and repositories transferred to this organization.  To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(
        rename = "dependabot_security_updates_enabled_for_new_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub dependabot_security_updates_enabled_for_new_repositories: Option<bool>,
    /// **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether dependency graph is automatically enabled for new repositories and repositories transferred to this organization.  To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(
        rename = "dependency_graph_enabled_for_new_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub dependency_graph_enabled_for_new_repositories: Option<bool>,
    /// **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether secret scanning is automatically enabled for new repositories and repositories transferred to this organization.  To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(
        rename = "secret_scanning_enabled_for_new_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_scanning_enabled_for_new_repositories: Option<bool>,
    /// **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether secret scanning push protection is automatically enabled for new repositories and repositories transferred to this organization.  To use this parameter, you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  You can check which security and analysis features are currently enabled by using a `GET /orgs/{org}` request.
    #[serde(
        rename = "secret_scanning_push_protection_enabled_for_new_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_scanning_push_protection_enabled_for_new_repositories: Option<bool>,
    /// Whether a custom link is shown to contributors who are blocked from pushing a secret by push protection.
    #[serde(
        rename = "secret_scanning_push_protection_custom_link_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_scanning_push_protection_custom_link_enabled: Option<bool>,
    /// If `secret_scanning_push_protection_custom_link_enabled` is true, the URL that will be displayed to contributors who are blocked from pushing a secret.
    #[serde(
        rename = "secret_scanning_push_protection_custom_link",
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_scanning_push_protection_custom_link: Option<String>,
    /// Controls whether or not deploy keys may be added and used for repositories in the organization.
    #[serde(
        rename = "deploy_keys_enabled_for_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub deploy_keys_enabled_for_repositories: Option<bool>,
}

impl OrgsUpdateRequest {
    pub fn new() -> OrgsUpdateRequest {
        OrgsUpdateRequest {
            billing_email: None,
            company: None,
            email: None,
            twitter_username: None,
            location: None,
            name: None,
            description: None,
            has_organization_projects: None,
            has_repository_projects: None,
            default_repository_permission: None,
            members_can_create_repositories: None,
            members_can_create_internal_repositories: None,
            members_can_create_private_repositories: None,
            members_can_create_public_repositories: None,
            members_allowed_repository_creation_type: None,
            members_can_create_pages: None,
            members_can_create_public_pages: None,
            members_can_create_private_pages: None,
            members_can_fork_private_repositories: None,
            web_commit_signoff_required: None,
            blog: None,
            advanced_security_enabled_for_new_repositories: None,
            dependabot_alerts_enabled_for_new_repositories: None,
            dependabot_security_updates_enabled_for_new_repositories: None,
            dependency_graph_enabled_for_new_repositories: None,
            secret_scanning_enabled_for_new_repositories: None,
            secret_scanning_push_protection_enabled_for_new_repositories: None,
            secret_scanning_push_protection_custom_link_enabled: None,
            secret_scanning_push_protection_custom_link: None,
            deploy_keys_enabled_for_repositories: None,
        }
    }
}
/// Default permission level members have for organization repositories.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DefaultRepositoryPermission {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "none")]
    None,
}

impl Default for DefaultRepositoryPermission {
    fn default() -> DefaultRepositoryPermission {
        Self::Read
    }
}
/// Specifies which types of repositories non-admin organization members can create. `private` is only available to repositories that are part of an organization on GitHub Enterprise Cloud.  **Note:** This parameter is closing down and will be removed in the future. Its return value ignores internal repositories. Using this parameter overrides values set in `members_can_create_repositories`. See the parameter deprecation notice in the operation description for details.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MembersAllowedRepositoryCreationType {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "none")]
    None,
}

impl Default for MembersAllowedRepositoryCreationType {
    fn default() -> MembersAllowedRepositoryCreationType {
        Self::All
    }
}
