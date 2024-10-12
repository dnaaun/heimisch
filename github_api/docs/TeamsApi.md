# \TeamsApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**teams_slash_add_member_legacy**](TeamsApi.md#teams_slash_add_member_legacy) | **PUT** /teams/{team_id}/members/{username} | Add team member (Legacy)
[**teams_slash_add_or_update_membership_for_user_in_org**](TeamsApi.md#teams_slash_add_or_update_membership_for_user_in_org) | **PUT** /orgs/{org}/teams/{team_slug}/memberships/{username} | Add or update team membership for a user
[**teams_slash_add_or_update_membership_for_user_legacy**](TeamsApi.md#teams_slash_add_or_update_membership_for_user_legacy) | **PUT** /teams/{team_id}/memberships/{username} | Add or update team membership for a user (Legacy)
[**teams_slash_add_or_update_project_permissions_in_org**](TeamsApi.md#teams_slash_add_or_update_project_permissions_in_org) | **PUT** /orgs/{org}/teams/{team_slug}/projects/{project_id} | Add or update team project permissions
[**teams_slash_add_or_update_project_permissions_legacy**](TeamsApi.md#teams_slash_add_or_update_project_permissions_legacy) | **PUT** /teams/{team_id}/projects/{project_id} | Add or update team project permissions (Legacy)
[**teams_slash_add_or_update_repo_permissions_in_org**](TeamsApi.md#teams_slash_add_or_update_repo_permissions_in_org) | **PUT** /orgs/{org}/teams/{team_slug}/repos/{owner}/{repo} | Add or update team repository permissions
[**teams_slash_add_or_update_repo_permissions_legacy**](TeamsApi.md#teams_slash_add_or_update_repo_permissions_legacy) | **PUT** /teams/{team_id}/repos/{owner}/{repo} | Add or update team repository permissions (Legacy)
[**teams_slash_check_permissions_for_project_in_org**](TeamsApi.md#teams_slash_check_permissions_for_project_in_org) | **GET** /orgs/{org}/teams/{team_slug}/projects/{project_id} | Check team permissions for a project
[**teams_slash_check_permissions_for_project_legacy**](TeamsApi.md#teams_slash_check_permissions_for_project_legacy) | **GET** /teams/{team_id}/projects/{project_id} | Check team permissions for a project (Legacy)
[**teams_slash_check_permissions_for_repo_in_org**](TeamsApi.md#teams_slash_check_permissions_for_repo_in_org) | **GET** /orgs/{org}/teams/{team_slug}/repos/{owner}/{repo} | Check team permissions for a repository
[**teams_slash_check_permissions_for_repo_legacy**](TeamsApi.md#teams_slash_check_permissions_for_repo_legacy) | **GET** /teams/{team_id}/repos/{owner}/{repo} | Check team permissions for a repository (Legacy)
[**teams_slash_create**](TeamsApi.md#teams_slash_create) | **POST** /orgs/{org}/teams | Create a team
[**teams_slash_create_discussion_comment_in_org**](TeamsApi.md#teams_slash_create_discussion_comment_in_org) | **POST** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments | Create a discussion comment
[**teams_slash_create_discussion_comment_legacy**](TeamsApi.md#teams_slash_create_discussion_comment_legacy) | **POST** /teams/{team_id}/discussions/{discussion_number}/comments | Create a discussion comment (Legacy)
[**teams_slash_create_discussion_in_org**](TeamsApi.md#teams_slash_create_discussion_in_org) | **POST** /orgs/{org}/teams/{team_slug}/discussions | Create a discussion
[**teams_slash_create_discussion_legacy**](TeamsApi.md#teams_slash_create_discussion_legacy) | **POST** /teams/{team_id}/discussions | Create a discussion (Legacy)
[**teams_slash_delete_discussion_comment_in_org**](TeamsApi.md#teams_slash_delete_discussion_comment_in_org) | **DELETE** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number} | Delete a discussion comment
[**teams_slash_delete_discussion_comment_legacy**](TeamsApi.md#teams_slash_delete_discussion_comment_legacy) | **DELETE** /teams/{team_id}/discussions/{discussion_number}/comments/{comment_number} | Delete a discussion comment (Legacy)
[**teams_slash_delete_discussion_in_org**](TeamsApi.md#teams_slash_delete_discussion_in_org) | **DELETE** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number} | Delete a discussion
[**teams_slash_delete_discussion_legacy**](TeamsApi.md#teams_slash_delete_discussion_legacy) | **DELETE** /teams/{team_id}/discussions/{discussion_number} | Delete a discussion (Legacy)
[**teams_slash_delete_in_org**](TeamsApi.md#teams_slash_delete_in_org) | **DELETE** /orgs/{org}/teams/{team_slug} | Delete a team
[**teams_slash_delete_legacy**](TeamsApi.md#teams_slash_delete_legacy) | **DELETE** /teams/{team_id} | Delete a team (Legacy)
[**teams_slash_get_by_name**](TeamsApi.md#teams_slash_get_by_name) | **GET** /orgs/{org}/teams/{team_slug} | Get a team by name
[**teams_slash_get_discussion_comment_in_org**](TeamsApi.md#teams_slash_get_discussion_comment_in_org) | **GET** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number} | Get a discussion comment
[**teams_slash_get_discussion_comment_legacy**](TeamsApi.md#teams_slash_get_discussion_comment_legacy) | **GET** /teams/{team_id}/discussions/{discussion_number}/comments/{comment_number} | Get a discussion comment (Legacy)
[**teams_slash_get_discussion_in_org**](TeamsApi.md#teams_slash_get_discussion_in_org) | **GET** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number} | Get a discussion
[**teams_slash_get_discussion_legacy**](TeamsApi.md#teams_slash_get_discussion_legacy) | **GET** /teams/{team_id}/discussions/{discussion_number} | Get a discussion (Legacy)
[**teams_slash_get_legacy**](TeamsApi.md#teams_slash_get_legacy) | **GET** /teams/{team_id} | Get a team (Legacy)
[**teams_slash_get_member_legacy**](TeamsApi.md#teams_slash_get_member_legacy) | **GET** /teams/{team_id}/members/{username} | Get team member (Legacy)
[**teams_slash_get_membership_for_user_in_org**](TeamsApi.md#teams_slash_get_membership_for_user_in_org) | **GET** /orgs/{org}/teams/{team_slug}/memberships/{username} | Get team membership for a user
[**teams_slash_get_membership_for_user_legacy**](TeamsApi.md#teams_slash_get_membership_for_user_legacy) | **GET** /teams/{team_id}/memberships/{username} | Get team membership for a user (Legacy)
[**teams_slash_list**](TeamsApi.md#teams_slash_list) | **GET** /orgs/{org}/teams | List teams
[**teams_slash_list_child_in_org**](TeamsApi.md#teams_slash_list_child_in_org) | **GET** /orgs/{org}/teams/{team_slug}/teams | List child teams
[**teams_slash_list_child_legacy**](TeamsApi.md#teams_slash_list_child_legacy) | **GET** /teams/{team_id}/teams | List child teams (Legacy)
[**teams_slash_list_discussion_comments_in_org**](TeamsApi.md#teams_slash_list_discussion_comments_in_org) | **GET** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments | List discussion comments
[**teams_slash_list_discussion_comments_legacy**](TeamsApi.md#teams_slash_list_discussion_comments_legacy) | **GET** /teams/{team_id}/discussions/{discussion_number}/comments | List discussion comments (Legacy)
[**teams_slash_list_discussions_in_org**](TeamsApi.md#teams_slash_list_discussions_in_org) | **GET** /orgs/{org}/teams/{team_slug}/discussions | List discussions
[**teams_slash_list_discussions_legacy**](TeamsApi.md#teams_slash_list_discussions_legacy) | **GET** /teams/{team_id}/discussions | List discussions (Legacy)
[**teams_slash_list_for_authenticated_user**](TeamsApi.md#teams_slash_list_for_authenticated_user) | **GET** /user/teams | List teams for the authenticated user
[**teams_slash_list_members_in_org**](TeamsApi.md#teams_slash_list_members_in_org) | **GET** /orgs/{org}/teams/{team_slug}/members | List team members
[**teams_slash_list_members_legacy**](TeamsApi.md#teams_slash_list_members_legacy) | **GET** /teams/{team_id}/members | List team members (Legacy)
[**teams_slash_list_pending_invitations_in_org**](TeamsApi.md#teams_slash_list_pending_invitations_in_org) | **GET** /orgs/{org}/teams/{team_slug}/invitations | List pending team invitations
[**teams_slash_list_pending_invitations_legacy**](TeamsApi.md#teams_slash_list_pending_invitations_legacy) | **GET** /teams/{team_id}/invitations | List pending team invitations (Legacy)
[**teams_slash_list_projects_in_org**](TeamsApi.md#teams_slash_list_projects_in_org) | **GET** /orgs/{org}/teams/{team_slug}/projects | List team projects
[**teams_slash_list_projects_legacy**](TeamsApi.md#teams_slash_list_projects_legacy) | **GET** /teams/{team_id}/projects | List team projects (Legacy)
[**teams_slash_list_repos_in_org**](TeamsApi.md#teams_slash_list_repos_in_org) | **GET** /orgs/{org}/teams/{team_slug}/repos | List team repositories
[**teams_slash_list_repos_legacy**](TeamsApi.md#teams_slash_list_repos_legacy) | **GET** /teams/{team_id}/repos | List team repositories (Legacy)
[**teams_slash_remove_member_legacy**](TeamsApi.md#teams_slash_remove_member_legacy) | **DELETE** /teams/{team_id}/members/{username} | Remove team member (Legacy)
[**teams_slash_remove_membership_for_user_in_org**](TeamsApi.md#teams_slash_remove_membership_for_user_in_org) | **DELETE** /orgs/{org}/teams/{team_slug}/memberships/{username} | Remove team membership for a user
[**teams_slash_remove_membership_for_user_legacy**](TeamsApi.md#teams_slash_remove_membership_for_user_legacy) | **DELETE** /teams/{team_id}/memberships/{username} | Remove team membership for a user (Legacy)
[**teams_slash_remove_project_in_org**](TeamsApi.md#teams_slash_remove_project_in_org) | **DELETE** /orgs/{org}/teams/{team_slug}/projects/{project_id} | Remove a project from a team
[**teams_slash_remove_project_legacy**](TeamsApi.md#teams_slash_remove_project_legacy) | **DELETE** /teams/{team_id}/projects/{project_id} | Remove a project from a team (Legacy)
[**teams_slash_remove_repo_in_org**](TeamsApi.md#teams_slash_remove_repo_in_org) | **DELETE** /orgs/{org}/teams/{team_slug}/repos/{owner}/{repo} | Remove a repository from a team
[**teams_slash_remove_repo_legacy**](TeamsApi.md#teams_slash_remove_repo_legacy) | **DELETE** /teams/{team_id}/repos/{owner}/{repo} | Remove a repository from a team (Legacy)
[**teams_slash_update_discussion_comment_in_org**](TeamsApi.md#teams_slash_update_discussion_comment_in_org) | **PATCH** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number} | Update a discussion comment
[**teams_slash_update_discussion_comment_legacy**](TeamsApi.md#teams_slash_update_discussion_comment_legacy) | **PATCH** /teams/{team_id}/discussions/{discussion_number}/comments/{comment_number} | Update a discussion comment (Legacy)
[**teams_slash_update_discussion_in_org**](TeamsApi.md#teams_slash_update_discussion_in_org) | **PATCH** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number} | Update a discussion
[**teams_slash_update_discussion_legacy**](TeamsApi.md#teams_slash_update_discussion_legacy) | **PATCH** /teams/{team_id}/discussions/{discussion_number} | Update a discussion (Legacy)
[**teams_slash_update_in_org**](TeamsApi.md#teams_slash_update_in_org) | **PATCH** /orgs/{org}/teams/{team_slug} | Update a team
[**teams_slash_update_legacy**](TeamsApi.md#teams_slash_update_legacy) | **PATCH** /teams/{team_id} | Update a team (Legacy)



## teams_slash_add_member_legacy

> teams_slash_add_member_legacy(team_id, username)
Add team member (Legacy)

The \"Add team member\" endpoint (described below) is closing down.  We recommend using the [Add or update team membership for a user](https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user) endpoint instead. It allows you to invite new organization members to your teams.  Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  To add someone to a team, the authenticated user must be an organization owner or a team maintainer in the team they're changing. The person being added to the team must be a member of the team's organization.  > [!NOTE] > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see \"[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/).\"  Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_add_or_update_membership_for_user_in_org

> models::TeamMembership teams_slash_add_or_update_membership_for_user_in_org(org, team_slug, username, teams_add_or_update_membership_for_user_in_org_request)
Add or update team membership for a user

Adds an organization member to a team. An authenticated organization owner or team maintainer can add organization members to a team.  Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  > [!NOTE] > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see \"[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/).\"  An organization owner can add someone who is not part of the team's organization to a team. When an organization owner adds someone to a team who is not an organization member, this endpoint will send an invitation to the person via email. This newly-created membership will be in the \"pending\" state until the person accepts the invitation, at which point the membership will transition to the \"active\" state and the user will be added as a member of the team.  If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/memberships/{username}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**teams_add_or_update_membership_for_user_in_org_request** | Option<[**TeamsAddOrUpdateMembershipForUserInOrgRequest**](TeamsAddOrUpdateMembershipForUserInOrgRequest.md)> |  |  |

### Return type

[**models::TeamMembership**](team-membership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_add_or_update_membership_for_user_legacy

> models::TeamMembership teams_slash_add_or_update_membership_for_user_legacy(team_id, username, teams_add_or_update_membership_for_user_in_org_request)
Add or update team membership for a user (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Add or update team membership for a user](https://docs.github.com/rest/teams/members#add-or-update-team-membership-for-a-user) endpoint.  Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  If the user is already a member of the team's organization, this endpoint will add the user to the team. To add a membership between an organization member and a team, the authenticated user must be an organization owner or a team maintainer.  > [!NOTE] > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see \"[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/).\"  If the user is unaffiliated with the team's organization, this endpoint will send an invitation to the user via email. This newly-created membership will be in the \"pending\" state until the user accepts the invitation, at which point the membership will transition to the \"active\" state and the user will be added as a member of the team. To add a membership between an unaffiliated user and a team, the authenticated user must be an organization owner.  If the user is already a member of the team, this endpoint will update the role of the team member's role. To update the membership of a team member, the authenticated user must be an organization owner or a team maintainer.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**teams_add_or_update_membership_for_user_in_org_request** | Option<[**TeamsAddOrUpdateMembershipForUserInOrgRequest**](TeamsAddOrUpdateMembershipForUserInOrgRequest.md)> |  |  |

### Return type

[**models::TeamMembership**](team-membership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_add_or_update_project_permissions_in_org

> teams_slash_add_or_update_project_permissions_in_org(org, team_slug, project_id, teams_add_or_update_project_permissions_in_org_request)
Add or update team project permissions

Adds an organization project to a team. To add a project to a team or update the team's permission on a project, the authenticated user must have `admin` permissions for the project. The project and team must be part of the same organization.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/projects/{project_id}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**project_id** | **i32** | The unique identifier of the project. | [required] |
**teams_add_or_update_project_permissions_in_org_request** | Option<[**TeamsAddOrUpdateProjectPermissionsInOrgRequest**](TeamsAddOrUpdateProjectPermissionsInOrgRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_add_or_update_project_permissions_legacy

> teams_slash_add_or_update_project_permissions_legacy(team_id, project_id, teams_add_or_update_project_permissions_legacy_request)
Add or update team project permissions (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Add or update team project permissions](https://docs.github.com/rest/teams/teams#add-or-update-team-project-permissions) endpoint.  Adds an organization project to a team. To add a project to a team or update the team's permission on a project, the authenticated user must have `admin` permissions for the project. The project and team must be part of the same organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**project_id** | **i32** | The unique identifier of the project. | [required] |
**teams_add_or_update_project_permissions_legacy_request** | Option<[**TeamsAddOrUpdateProjectPermissionsLegacyRequest**](TeamsAddOrUpdateProjectPermissionsLegacyRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_add_or_update_repo_permissions_in_org

> teams_slash_add_or_update_repo_permissions_in_org(org, team_slug, owner, repo, teams_add_or_update_repo_permissions_in_org_request)
Add or update team repository permissions

To add a repository to a team or update the team's permission on a repository, the authenticated user must have admin access to the repository, and must be able to see the team. The repository must be owned by the organization, or a direct fork of a repository owned by the organization. You will get a `422 Unprocessable Entity` status if you attempt to add a repository to a team that is not owned by the organization. Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method).\"  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `PUT /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.  For more information about the permission levels, see \"[Repository permission levels for an organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/repository-permission-levels-for-an-organization#permission-levels-for-repositories-owned-by-an-organization)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**teams_add_or_update_repo_permissions_in_org_request** | Option<[**TeamsAddOrUpdateRepoPermissionsInOrgRequest**](TeamsAddOrUpdateRepoPermissionsInOrgRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_add_or_update_repo_permissions_legacy

> teams_slash_add_or_update_repo_permissions_legacy(team_id, owner, repo, teams_add_or_update_repo_permissions_legacy_request)
Add or update team repository permissions (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new \"[Add or update team repository permissions](https://docs.github.com/rest/teams/teams#add-or-update-team-repository-permissions)\" endpoint.  To add a repository to a team or update the team's permission on a repository, the authenticated user must have admin access to the repository, and must be able to see the team. The repository must be owned by the organization, or a direct fork of a repository owned by the organization. You will get a `422 Unprocessable Entity` status if you attempt to add a repository to a team that is not owned by the organization.  Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**teams_add_or_update_repo_permissions_legacy_request** | Option<[**TeamsAddOrUpdateRepoPermissionsLegacyRequest**](TeamsAddOrUpdateRepoPermissionsLegacyRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_check_permissions_for_project_in_org

> models::TeamProject teams_slash_check_permissions_for_project_in_org(org, team_slug, project_id)
Check team permissions for a project

Checks whether a team has `read`, `write`, or `admin` permissions for an organization project. The response includes projects inherited from a parent team.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/projects/{project_id}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**project_id** | **i32** | The unique identifier of the project. | [required] |

### Return type

[**models::TeamProject**](team-project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_check_permissions_for_project_legacy

> models::TeamProject teams_slash_check_permissions_for_project_legacy(team_id, project_id)
Check team permissions for a project (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Check team permissions for a project](https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-project) endpoint.  Checks whether a team has `read`, `write`, or `admin` permissions for an organization project. The response includes projects inherited from a parent team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**project_id** | **i32** | The unique identifier of the project. | [required] |

### Return type

[**models::TeamProject**](team-project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_check_permissions_for_repo_in_org

> models::TeamRepository teams_slash_check_permissions_for_repo_in_org(org, team_slug, owner, repo)
Check team permissions for a repository

Checks whether a team has `admin`, `push`, `maintain`, `triage`, or `pull` permission for a repository. Repositories inherited through a parent team will also be checked.  You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types/) via the `application/vnd.github.v3.repository+json` accept header.  If a team doesn't have permission for the repository, you will receive a `404 Not Found` response status.  If the repository is private, you must have at least `read` permission for that repository, and your token must have the `repo` or `admin:org` scope. Otherwise, you will receive a `404 Not Found` response status.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::TeamRepository**](team-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_check_permissions_for_repo_legacy

> models::TeamRepository teams_slash_check_permissions_for_repo_legacy(team_id, owner, repo)
Check team permissions for a repository (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Check team permissions for a repository](https://docs.github.com/rest/teams/teams#check-team-permissions-for-a-repository) endpoint.  > [!NOTE] > Repositories inherited through a parent team will also be checked.  You can also get information about the specified repository, including what permissions the team grants on it, by passing the following custom [media type](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types/) via the `Accept` header:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::TeamRepository**](team-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_create

> models::TeamFull teams_slash_create(org, teams_create_request)
Create a team

To create a team, the authenticated user must be a member or owner of `{org}`. By default, organization members can create teams. Organization owners can limit team creation to organization owners. For more information, see \"[Setting team creation permissions](https://docs.github.com/articles/setting-team-creation-permissions-in-your-organization).\"  When you create a new team, you automatically become a team maintainer without explicitly adding yourself to the optional array of `maintainers`. For more information, see \"[About teams](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/about-teams)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**teams_create_request** | [**TeamsCreateRequest**](TeamsCreateRequest.md) |  | [required] |

### Return type

[**models::TeamFull**](team-full.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_create_discussion_comment_in_org

> models::TeamDiscussionComment teams_slash_create_discussion_comment_in_org(org, team_slug, discussion_number, teams_create_discussion_comment_in_org_request)
Create a discussion comment

Creates a new comment on a team discussion.  This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see \"[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)\" and \"[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api).\"  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments`.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**teams_create_discussion_comment_in_org_request** | [**TeamsCreateDiscussionCommentInOrgRequest**](TeamsCreateDiscussionCommentInOrgRequest.md) |  | [required] |

### Return type

[**models::TeamDiscussionComment**](team-discussion-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_create_discussion_comment_legacy

> models::TeamDiscussionComment teams_slash_create_discussion_comment_legacy(team_id, discussion_number, teams_create_discussion_comment_in_org_request)
Create a discussion comment (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Create a discussion comment](https://docs.github.com/rest/teams/discussion-comments#create-a-discussion-comment) endpoint.  Creates a new comment on a team discussion.  This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see \"[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)\" and \"[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api).\"  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**teams_create_discussion_comment_in_org_request** | [**TeamsCreateDiscussionCommentInOrgRequest**](TeamsCreateDiscussionCommentInOrgRequest.md) |  | [required] |

### Return type

[**models::TeamDiscussionComment**](team-discussion-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_create_discussion_in_org

> models::TeamDiscussion teams_slash_create_discussion_in_org(org, team_slug, teams_create_discussion_in_org_request)
Create a discussion

Creates a new discussion post on a team's page.  This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see \"[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)\" and \"[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api).\"  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/{org_id}/team/{team_id}/discussions`.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**teams_create_discussion_in_org_request** | [**TeamsCreateDiscussionInOrgRequest**](TeamsCreateDiscussionInOrgRequest.md) |  | [required] |

### Return type

[**models::TeamDiscussion**](team-discussion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_create_discussion_legacy

> models::TeamDiscussion teams_slash_create_discussion_legacy(team_id, teams_create_discussion_in_org_request)
Create a discussion (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create a discussion`](https://docs.github.com/rest/teams/discussions#create-a-discussion) endpoint.  Creates a new discussion post on a team's page.  This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see \"[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)\" and \"[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api).\"  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**teams_create_discussion_in_org_request** | [**TeamsCreateDiscussionInOrgRequest**](TeamsCreateDiscussionInOrgRequest.md) |  | [required] |

### Return type

[**models::TeamDiscussion**](team-discussion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_delete_discussion_comment_in_org

> teams_slash_delete_discussion_comment_in_org(org, team_slug, discussion_number, comment_number)
Delete a discussion comment

Deletes a comment on a team discussion.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**comment_number** | **i32** | The number that identifies the comment. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_delete_discussion_comment_legacy

> teams_slash_delete_discussion_comment_legacy(team_id, discussion_number, comment_number)
Delete a discussion comment (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Delete a discussion comment](https://docs.github.com/rest/teams/discussion-comments#delete-a-discussion-comment) endpoint.  Deletes a comment on a team discussion.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**comment_number** | **i32** | The number that identifies the comment. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_delete_discussion_in_org

> teams_slash_delete_discussion_in_org(org, team_slug, discussion_number)
Delete a discussion

Delete a discussion from a team's page.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_delete_discussion_legacy

> teams_slash_delete_discussion_legacy(team_id, discussion_number)
Delete a discussion (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Delete a discussion`](https://docs.github.com/rest/teams/discussions#delete-a-discussion) endpoint.  Delete a discussion from a team's page.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_delete_in_org

> teams_slash_delete_in_org(org, team_slug)
Delete a team

To delete a team, the authenticated user must be an organization owner or team maintainer.  If you are an organization owner, deleting a parent team will delete all of its child teams as well.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_delete_legacy

> teams_slash_delete_legacy(team_id)
Delete a team (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Delete a team](https://docs.github.com/rest/teams/teams#delete-a-team) endpoint.  To delete a team, the authenticated user must be an organization owner or team maintainer.  If you are an organization owner, deleting a parent team will delete all of its child teams as well.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_get_by_name

> models::TeamFull teams_slash_get_by_name(org, team_slug)
Get a team by name

Gets a team using the team's `slug`. To create the `slug`, GitHub replaces special characters in the `name` string, changes all words to lowercase, and replaces spaces with a `-` separator. For example, `\"My TEam Näme\"` would become `my-team-name`.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |

### Return type

[**models::TeamFull**](team-full.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_get_discussion_comment_in_org

> models::TeamDiscussionComment teams_slash_get_discussion_comment_in_org(org, team_slug, discussion_number, comment_number)
Get a discussion comment

Get a specific comment on a team discussion.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**comment_number** | **i32** | The number that identifies the comment. | [required] |

### Return type

[**models::TeamDiscussionComment**](team-discussion-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_get_discussion_comment_legacy

> models::TeamDiscussionComment teams_slash_get_discussion_comment_legacy(team_id, discussion_number, comment_number)
Get a discussion comment (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get a discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment) endpoint.  Get a specific comment on a team discussion.  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**comment_number** | **i32** | The number that identifies the comment. | [required] |

### Return type

[**models::TeamDiscussionComment**](team-discussion-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_get_discussion_in_org

> models::TeamDiscussion teams_slash_get_discussion_in_org(org, team_slug, discussion_number)
Get a discussion

Get a specific discussion on a team's page.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |

### Return type

[**models::TeamDiscussion**](team-discussion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_get_discussion_legacy

> models::TeamDiscussion teams_slash_get_discussion_legacy(team_id, discussion_number)
Get a discussion (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get a discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion) endpoint.  Get a specific discussion on a team's page.  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |

### Return type

[**models::TeamDiscussion**](team-discussion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_get_legacy

> models::TeamFull teams_slash_get_legacy(team_id)
Get a team (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the [Get a team by name](https://docs.github.com/rest/teams/teams#get-a-team-by-name) endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |

### Return type

[**models::TeamFull**](team-full.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_get_member_legacy

> teams_slash_get_member_legacy(team_id, username)
Get team member (Legacy)

The \"Get team member\" endpoint (described below) is closing down.  We recommend using the [Get team membership for a user](https://docs.github.com/rest/teams/members#get-team-membership-for-a-user) endpoint instead. It allows you to get both active and pending memberships.  To list members in a team, the team must be visible to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_get_membership_for_user_in_org

> models::TeamMembership teams_slash_get_membership_for_user_in_org(org, team_slug, username)
Get team membership for a user

Team members will include the members of child teams.  To get a user's membership with a team, the team must be visible to the authenticated user.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/memberships/{username}`.  > [!NOTE] > The response contains the `state` of the membership and the member's `role`.  The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see [Create a team](https://docs.github.com/rest/teams/teams#create-a-team).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::TeamMembership**](team-membership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_get_membership_for_user_legacy

> models::TeamMembership teams_slash_get_membership_for_user_legacy(team_id, username)
Get team membership for a user (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Get team membership for a user](https://docs.github.com/rest/teams/members#get-team-membership-for-a-user) endpoint.  Team members will include the members of child teams.  To get a user's membership with a team, the team must be visible to the authenticated user.  **Note:** The response contains the `state` of the membership and the member's `role`.  The `role` for organization owners is set to `maintainer`. For more information about `maintainer` roles, see [Create a team](https://docs.github.com/rest/teams/teams#create-a-team).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::TeamMembership**](team-membership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list

> Vec<models::Team> teams_slash_list(org, per_page, page)
List teams

Lists all teams in an organization that are visible to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Team>**](team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_child_in_org

> Vec<models::Team> teams_slash_list_child_in_org(org, team_slug, per_page, page)
List child teams

Lists the child teams of the team specified by `{team_slug}`.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/teams`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Team>**](team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_child_legacy

> Vec<models::Team> teams_slash_list_child_legacy(team_id, per_page, page)
List child teams (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List child teams`](https://docs.github.com/rest/teams/teams#list-child-teams) endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Team>**](team.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_discussion_comments_in_org

> Vec<models::TeamDiscussionComment> teams_slash_list_discussion_comments_in_org(org, team_slug, discussion_number, direction, per_page, page)
List discussion comments

List all comments on a team discussion.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments`.  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::TeamDiscussionComment>**](team-discussion-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_discussion_comments_legacy

> Vec<models::TeamDiscussionComment> teams_slash_list_discussion_comments_legacy(team_id, discussion_number, direction, per_page, page)
List discussion comments (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [List discussion comments](https://docs.github.com/rest/teams/discussion-comments#list-discussion-comments) endpoint.  List all comments on a team discussion.  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::TeamDiscussionComment>**](team-discussion-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_discussions_in_org

> Vec<models::TeamDiscussion> teams_slash_list_discussions_in_org(org, team_slug, direction, per_page, page, pinned)
List discussions

List all discussions on a team's page.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/discussions`.  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**pinned** | Option<**String**> | Pinned discussions only filter |  |

### Return type

[**Vec<models::TeamDiscussion>**](team-discussion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_discussions_legacy

> Vec<models::TeamDiscussion> teams_slash_list_discussions_legacy(team_id, direction, per_page, page)
List discussions (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List discussions`](https://docs.github.com/rest/teams/discussions#list-discussions) endpoint.  List all discussions on a team's page.  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::TeamDiscussion>**](team-discussion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_for_authenticated_user

> Vec<models::TeamFull> teams_slash_list_for_authenticated_user(per_page, page)
List teams for the authenticated user

List all of the teams across all of the organizations to which the authenticated user belongs.  OAuth app tokens and personal access tokens (classic) need the `user`, `repo`, or `read:org` scope to use this endpoint.  When using a fine-grained personal access token, the resource owner of the token must be a single organization, and the response will only include the teams from that organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::TeamFull>**](team-full.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_members_in_org

> Vec<models::SimpleUser> teams_slash_list_members_in_org(org, team_slug, role, per_page, page)
List team members

Team members will include the members of child teams.  To list members in a team, the team must be visible to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**role** | Option<**String**> | Filters members returned by their role in the team. |  |[default to all]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::SimpleUser>**](simple-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_members_legacy

> Vec<models::SimpleUser> teams_slash_list_members_legacy(team_id, role, per_page, page)
List team members (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team members`](https://docs.github.com/rest/teams/members#list-team-members) endpoint.  Team members will include the members of child teams.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**role** | Option<**String**> | Filters members returned by their role in the team. |  |[default to all]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::SimpleUser>**](simple-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_pending_invitations_in_org

> Vec<models::OrganizationInvitation> teams_slash_list_pending_invitations_in_org(org, team_slug, per_page, page)
List pending team invitations

The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/invitations`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::OrganizationInvitation>**](organization-invitation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_pending_invitations_legacy

> Vec<models::OrganizationInvitation> teams_slash_list_pending_invitations_legacy(team_id, per_page, page)
List pending team invitations (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List pending team invitations`](https://docs.github.com/rest/teams/members#list-pending-team-invitations) endpoint.  The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, `hiring_manager`, or `reinstate`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::OrganizationInvitation>**](organization-invitation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_projects_in_org

> Vec<models::TeamProject> teams_slash_list_projects_in_org(org, team_slug, per_page, page)
List team projects

Lists the organization projects for a team.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/projects`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::TeamProject>**](team-project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_projects_legacy

> Vec<models::TeamProject> teams_slash_list_projects_legacy(team_id, per_page, page)
List team projects (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List team projects`](https://docs.github.com/rest/teams/teams#list-team-projects) endpoint.  Lists the organization projects for a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::TeamProject>**](team-project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_repos_in_org

> Vec<models::MinimalRepository> teams_slash_list_repos_in_org(org, team_slug, per_page, page)
List team repositories

Lists a team's repositories visible to the authenticated user.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/{org_id}/team/{team_id}/repos`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::MinimalRepository>**](minimal-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_list_repos_legacy

> Vec<models::MinimalRepository> teams_slash_list_repos_legacy(team_id, per_page, page)
List team repositories (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [List team repositories](https://docs.github.com/rest/teams/teams#list-team-repositories) endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::MinimalRepository>**](minimal-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_remove_member_legacy

> teams_slash_remove_member_legacy(team_id, username)
Remove team member (Legacy)

The \"Remove team member\" endpoint (described below) is closing down.  We recommend using the [Remove team membership for a user](https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user) endpoint instead. It allows you to remove both active and pending memberships.  Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  To remove a team member, the authenticated user must have 'admin' permissions to the team or be an owner of the org that the team is associated with. Removing a team member does not delete the user, it just removes them from the team.  > [!NOTE] > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see \"[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_remove_membership_for_user_in_org

> teams_slash_remove_membership_for_user_in_org(org, team_slug, username)
Remove team membership for a user

To remove a membership between a user and a team, the authenticated user must have 'admin' permissions to the team or be an owner of the organization that the team is associated with. Removing team membership does not delete the user, it just removes their membership from the team.  Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  > [!NOTE] > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see \"[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/).\"  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/memberships/{username}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_remove_membership_for_user_legacy

> teams_slash_remove_membership_for_user_legacy(team_id, username)
Remove team membership for a user (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove team membership for a user](https://docs.github.com/rest/teams/members#remove-team-membership-for-a-user) endpoint.  Team synchronization is available for organizations using GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.  To remove a membership between a user and a team, the authenticated user must have 'admin' permissions to the team or be an owner of the organization that the team is associated with. Removing team membership does not delete the user, it just removes their membership from the team.  > [!NOTE] > When you have team synchronization set up for a team with your organization's identity provider (IdP), you will see an error if you attempt to use the API for making changes to the team's membership. If you have access to manage group membership in your IdP, you can manage GitHub team membership through your identity provider, which automatically adds and removes team members in an organization. For more information, see \"[Synchronizing teams between your identity provider and GitHub](https://docs.github.com/articles/synchronizing-teams-between-your-identity-provider-and-github/).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_remove_project_in_org

> teams_slash_remove_project_in_org(org, team_slug, project_id)
Remove a project from a team

Removes an organization project from a team. An organization owner or a team maintainer can remove any project from the team. To remove a project from a team as an organization member, the authenticated user must have `read` access to both the team and project, or `admin` access to the team or project. This endpoint removes the project from the team, but does not delete the project.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/projects/{project_id}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**project_id** | **i32** | The unique identifier of the project. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_remove_project_legacy

> teams_slash_remove_project_legacy(team_id, project_id)
Remove a project from a team (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove a project from a team](https://docs.github.com/rest/teams/teams#remove-a-project-from-a-team) endpoint.  Removes an organization project from a team. An organization owner or a team maintainer can remove any project from the team. To remove a project from a team as an organization member, the authenticated user must have `read` access to both the team and project, or `admin` access to the team or project. **Note:** This endpoint removes the project from the team, but does not delete it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**project_id** | **i32** | The unique identifier of the project. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_remove_repo_in_org

> teams_slash_remove_repo_in_org(org, team_slug, owner, repo)
Remove a repository from a team

If the authenticated user is an organization owner or a team maintainer, they can remove any repositories from the team. To remove a repository from a team as an organization member, the authenticated user must have admin access to the repository and must be able to see the team. This does not delete the repository, it just removes it from the team.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `DELETE /organizations/{org_id}/team/{team_id}/repos/{owner}/{repo}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_remove_repo_legacy

> teams_slash_remove_repo_legacy(team_id, owner, repo)
Remove a repository from a team (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Remove a repository from a team](https://docs.github.com/rest/teams/teams#remove-a-repository-from-a-team) endpoint.  If the authenticated user is an organization owner or a team maintainer, they can remove any repositories from the team. To remove a repository from a team as an organization member, the authenticated user must have admin access to the repository and must be able to see the team. NOTE: This does not delete the repository, it just removes it from the team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_update_discussion_comment_in_org

> models::TeamDiscussionComment teams_slash_update_discussion_comment_in_org(org, team_slug, discussion_number, comment_number, teams_create_discussion_comment_in_org_request)
Update a discussion comment

Edits the body text of a discussion comment.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}/comments/{comment_number}`.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**comment_number** | **i32** | The number that identifies the comment. | [required] |
**teams_create_discussion_comment_in_org_request** | [**TeamsCreateDiscussionCommentInOrgRequest**](TeamsCreateDiscussionCommentInOrgRequest.md) |  | [required] |

### Return type

[**models::TeamDiscussionComment**](team-discussion-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_update_discussion_comment_legacy

> models::TeamDiscussionComment teams_slash_update_discussion_comment_legacy(team_id, discussion_number, comment_number, teams_create_discussion_comment_in_org_request)
Update a discussion comment (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a discussion comment](https://docs.github.com/rest/teams/discussion-comments#update-a-discussion-comment) endpoint.  Edits the body text of a discussion comment.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**comment_number** | **i32** | The number that identifies the comment. | [required] |
**teams_create_discussion_comment_in_org_request** | [**TeamsCreateDiscussionCommentInOrgRequest**](TeamsCreateDiscussionCommentInOrgRequest.md) |  | [required] |

### Return type

[**models::TeamDiscussionComment**](team-discussion-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_update_discussion_in_org

> models::TeamDiscussion teams_slash_update_discussion_in_org(org, team_slug, discussion_number, teams_update_discussion_in_org_request)
Update a discussion

Edits the title and body text of a discussion post. Only the parameters you provide are updated.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}/discussions/{discussion_number}`.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**teams_update_discussion_in_org_request** | Option<[**TeamsUpdateDiscussionInOrgRequest**](TeamsUpdateDiscussionInOrgRequest.md)> |  |  |

### Return type

[**models::TeamDiscussion**](team-discussion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_update_discussion_legacy

> models::TeamDiscussion teams_slash_update_discussion_legacy(team_id, discussion_number, teams_update_discussion_in_org_request)
Update a discussion (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a discussion](https://docs.github.com/rest/teams/discussions#update-a-discussion) endpoint.  Edits the title and body text of a discussion post. Only the parameters you provide are updated.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**teams_update_discussion_in_org_request** | Option<[**TeamsUpdateDiscussionInOrgRequest**](TeamsUpdateDiscussionInOrgRequest.md)> |  |  |

### Return type

[**models::TeamDiscussion**](team-discussion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_update_in_org

> models::TeamFull teams_slash_update_in_org(org, team_slug, teams_update_in_org_request)
Update a team

To edit a team, the authenticated user must either be an organization owner or a team maintainer.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `PATCH /organizations/{org_id}/team/{team_id}`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**teams_update_in_org_request** | Option<[**TeamsUpdateInOrgRequest**](TeamsUpdateInOrgRequest.md)> |  |  |

### Return type

[**models::TeamFull**](team-full.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_slash_update_legacy

> models::TeamFull teams_slash_update_legacy(team_id, teams_update_legacy_request)
Update a team (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [Update a team](https://docs.github.com/rest/teams/teams#update-a-team) endpoint.  To edit a team, the authenticated user must either be an organization owner or a team maintainer.  > [!NOTE] > With nested teams, the `privacy` for parent teams cannot be `secret`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**teams_update_legacy_request** | [**TeamsUpdateLegacyRequest**](TeamsUpdateLegacyRequest.md) |  | [required] |

### Return type

[**models::TeamFull**](team-full.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

