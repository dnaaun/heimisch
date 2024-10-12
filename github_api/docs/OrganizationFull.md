# OrganizationFull

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**login** | **String** |  | 
**id** | **i32** |  | 
**node_id** | **String** |  | 
**url** | **String** |  | 
**repos_url** | **String** |  | 
**events_url** | **String** |  | 
**hooks_url** | **String** |  | 
**issues_url** | **String** |  | 
**members_url** | **String** |  | 
**public_members_url** | **String** |  | 
**avatar_url** | **String** |  | 
**description** | Option<**String**> |  | 
**name** | Option<**String**> |  | [optional]
**company** | Option<**String**> |  | [optional]
**blog** | Option<**String**> |  | [optional]
**location** | Option<**String**> |  | [optional]
**email** | Option<**String**> |  | [optional]
**twitter_username** | Option<**String**> |  | [optional]
**is_verified** | Option<**bool**> |  | [optional]
**has_organization_projects** | **bool** |  | 
**has_repository_projects** | **bool** |  | 
**public_repos** | **i32** |  | 
**public_gists** | **i32** |  | 
**followers** | **i32** |  | 
**following** | **i32** |  | 
**html_url** | **String** |  | 
**r#type** | **String** |  | 
**total_private_repos** | Option<**i32**> |  | [optional]
**owned_private_repos** | Option<**i32**> |  | [optional]
**private_gists** | Option<**i32**> |  | [optional]
**disk_usage** | Option<**i32**> |  | [optional]
**collaborators** | Option<**i32**> | The number of collaborators on private repositories.  This field may be null if the number of private repositories is over 50,000. | [optional]
**billing_email** | Option<**String**> |  | [optional]
**plan** | Option<[**models::OrganizationFullPlan**](organization_full_plan.md)> |  | [optional]
**default_repository_permission** | Option<**String**> |  | [optional]
**members_can_create_repositories** | Option<**bool**> |  | [optional]
**two_factor_requirement_enabled** | Option<**bool**> |  | [optional]
**members_allowed_repository_creation_type** | Option<**String**> |  | [optional]
**members_can_create_public_repositories** | Option<**bool**> |  | [optional]
**members_can_create_private_repositories** | Option<**bool**> |  | [optional]
**members_can_create_internal_repositories** | Option<**bool**> |  | [optional]
**members_can_create_pages** | Option<**bool**> |  | [optional]
**members_can_create_public_pages** | Option<**bool**> |  | [optional]
**members_can_create_private_pages** | Option<**bool**> |  | [optional]
**members_can_fork_private_repositories** | Option<**bool**> |  | [optional]
**web_commit_signoff_required** | Option<**bool**> |  | [optional]
**advanced_security_enabled_for_new_repositories** | Option<**bool**> | **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether GitHub Advanced Security is enabled for new repositories and repositories transferred to this organization.  This field is only visible to organization owners or members of a team with the security manager role. | [optional]
**dependabot_alerts_enabled_for_new_repositories** | Option<**bool**> | **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether Dependabot alerts are automatically enabled for new repositories and repositories transferred to this organization.  This field is only visible to organization owners or members of a team with the security manager role. | [optional]
**dependabot_security_updates_enabled_for_new_repositories** | Option<**bool**> | **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether Dependabot security updates are automatically enabled for new repositories and repositories transferred to this organization.  This field is only visible to organization owners or members of a team with the security manager role. | [optional]
**dependency_graph_enabled_for_new_repositories** | Option<**bool**> | **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether dependency graph is automatically enabled for new repositories and repositories transferred to this organization.  This field is only visible to organization owners or members of a team with the security manager role. | [optional]
**secret_scanning_enabled_for_new_repositories** | Option<**bool**> | **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether secret scanning is automatically enabled for new repositories and repositories transferred to this organization.  This field is only visible to organization owners or members of a team with the security manager role. | [optional]
**secret_scanning_push_protection_enabled_for_new_repositories** | Option<**bool**> | **Endpoint closing down notice.** Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead.  Whether secret scanning push protection is automatically enabled for new repositories and repositories transferred to this organization.  This field is only visible to organization owners or members of a team with the security manager role. | [optional]
**secret_scanning_push_protection_custom_link_enabled** | Option<**bool**> | Whether a custom link is shown to contributors who are blocked from pushing a secret by push protection. | [optional]
**secret_scanning_push_protection_custom_link** | Option<**String**> | An optional URL string to display to contributors who are blocked from pushing a secret. | [optional]
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**archived_at** | Option<**String**> |  | 
**deploy_keys_enabled_for_repositories** | Option<**bool**> | Controls whether or not deploy keys may be added and used for repositories in the organization. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


