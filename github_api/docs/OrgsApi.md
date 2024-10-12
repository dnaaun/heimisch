# \OrgsApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_insights_slash_get_route_stats_by_actor**](OrgsApi.md#api_insights_slash_get_route_stats_by_actor) | **GET** /orgs/{org}/insights/api/route-stats/{actor_type}/{actor_id} | Get route stats by actor
[**api_insights_slash_get_subject_stats**](OrgsApi.md#api_insights_slash_get_subject_stats) | **GET** /orgs/{org}/insights/api/subject-stats | Get subject stats
[**api_insights_slash_get_summary_stats**](OrgsApi.md#api_insights_slash_get_summary_stats) | **GET** /orgs/{org}/insights/api/summary-stats | Get summary stats
[**api_insights_slash_get_summary_stats_by_actor**](OrgsApi.md#api_insights_slash_get_summary_stats_by_actor) | **GET** /orgs/{org}/insights/api/summary-stats/{actor_type}/{actor_id} | Get summary stats by actor
[**api_insights_slash_get_summary_stats_by_user**](OrgsApi.md#api_insights_slash_get_summary_stats_by_user) | **GET** /orgs/{org}/insights/api/summary-stats/users/{user_id} | Get summary stats by user
[**api_insights_slash_get_time_stats**](OrgsApi.md#api_insights_slash_get_time_stats) | **GET** /orgs/{org}/insights/api/time-stats | Get time stats
[**api_insights_slash_get_time_stats_by_actor**](OrgsApi.md#api_insights_slash_get_time_stats_by_actor) | **GET** /orgs/{org}/insights/api/time-stats/{actor_type}/{actor_id} | Get time stats by actor
[**api_insights_slash_get_time_stats_by_user**](OrgsApi.md#api_insights_slash_get_time_stats_by_user) | **GET** /orgs/{org}/insights/api/time-stats/users/{user_id} | Get time stats by user
[**api_insights_slash_get_user_stats**](OrgsApi.md#api_insights_slash_get_user_stats) | **GET** /orgs/{org}/insights/api/user-stats/{user_id} | Get user stats
[**orgs_slash_add_security_manager_team**](OrgsApi.md#orgs_slash_add_security_manager_team) | **PUT** /orgs/{org}/security-managers/teams/{team_slug} | Add a security manager team
[**orgs_slash_assign_team_to_org_role**](OrgsApi.md#orgs_slash_assign_team_to_org_role) | **PUT** /orgs/{org}/organization-roles/teams/{team_slug}/{role_id} | Assign an organization role to a team
[**orgs_slash_assign_user_to_org_role**](OrgsApi.md#orgs_slash_assign_user_to_org_role) | **PUT** /orgs/{org}/organization-roles/users/{username}/{role_id} | Assign an organization role to a user
[**orgs_slash_block_user**](OrgsApi.md#orgs_slash_block_user) | **PUT** /orgs/{org}/blocks/{username} | Block a user from an organization
[**orgs_slash_cancel_invitation**](OrgsApi.md#orgs_slash_cancel_invitation) | **DELETE** /orgs/{org}/invitations/{invitation_id} | Cancel an organization invitation
[**orgs_slash_check_blocked_user**](OrgsApi.md#orgs_slash_check_blocked_user) | **GET** /orgs/{org}/blocks/{username} | Check if a user is blocked by an organization
[**orgs_slash_check_membership_for_user**](OrgsApi.md#orgs_slash_check_membership_for_user) | **GET** /orgs/{org}/members/{username} | Check organization membership for a user
[**orgs_slash_check_public_membership_for_user**](OrgsApi.md#orgs_slash_check_public_membership_for_user) | **GET** /orgs/{org}/public_members/{username} | Check public organization membership for a user
[**orgs_slash_convert_member_to_outside_collaborator**](OrgsApi.md#orgs_slash_convert_member_to_outside_collaborator) | **PUT** /orgs/{org}/outside_collaborators/{username} | Convert an organization member to outside collaborator
[**orgs_slash_create_invitation**](OrgsApi.md#orgs_slash_create_invitation) | **POST** /orgs/{org}/invitations | Create an organization invitation
[**orgs_slash_create_or_update_custom_properties**](OrgsApi.md#orgs_slash_create_or_update_custom_properties) | **PATCH** /orgs/{org}/properties/schema | Create or update custom properties for an organization
[**orgs_slash_create_or_update_custom_properties_values_for_repos**](OrgsApi.md#orgs_slash_create_or_update_custom_properties_values_for_repos) | **PATCH** /orgs/{org}/properties/values | Create or update custom property values for organization repositories
[**orgs_slash_create_or_update_custom_property**](OrgsApi.md#orgs_slash_create_or_update_custom_property) | **PUT** /orgs/{org}/properties/schema/{custom_property_name} | Create or update a custom property for an organization
[**orgs_slash_create_webhook**](OrgsApi.md#orgs_slash_create_webhook) | **POST** /orgs/{org}/hooks | Create an organization webhook
[**orgs_slash_delete**](OrgsApi.md#orgs_slash_delete) | **DELETE** /orgs/{org} | Delete an organization
[**orgs_slash_delete_webhook**](OrgsApi.md#orgs_slash_delete_webhook) | **DELETE** /orgs/{org}/hooks/{hook_id} | Delete an organization webhook
[**orgs_slash_enable_or_disable_security_product_on_all_org_repos**](OrgsApi.md#orgs_slash_enable_or_disable_security_product_on_all_org_repos) | **POST** /orgs/{org}/{security_product}/{enablement} | Enable or disable a security feature for an organization
[**orgs_slash_get**](OrgsApi.md#orgs_slash_get) | **GET** /orgs/{org} | Get an organization
[**orgs_slash_get_all_custom_properties**](OrgsApi.md#orgs_slash_get_all_custom_properties) | **GET** /orgs/{org}/properties/schema | Get all custom properties for an organization
[**orgs_slash_get_custom_property**](OrgsApi.md#orgs_slash_get_custom_property) | **GET** /orgs/{org}/properties/schema/{custom_property_name} | Get a custom property for an organization
[**orgs_slash_get_membership_for_authenticated_user**](OrgsApi.md#orgs_slash_get_membership_for_authenticated_user) | **GET** /user/memberships/orgs/{org} | Get an organization membership for the authenticated user
[**orgs_slash_get_membership_for_user**](OrgsApi.md#orgs_slash_get_membership_for_user) | **GET** /orgs/{org}/memberships/{username} | Get organization membership for a user
[**orgs_slash_get_org_role**](OrgsApi.md#orgs_slash_get_org_role) | **GET** /orgs/{org}/organization-roles/{role_id} | Get an organization role
[**orgs_slash_get_webhook**](OrgsApi.md#orgs_slash_get_webhook) | **GET** /orgs/{org}/hooks/{hook_id} | Get an organization webhook
[**orgs_slash_get_webhook_config_for_org**](OrgsApi.md#orgs_slash_get_webhook_config_for_org) | **GET** /orgs/{org}/hooks/{hook_id}/config | Get a webhook configuration for an organization
[**orgs_slash_get_webhook_delivery**](OrgsApi.md#orgs_slash_get_webhook_delivery) | **GET** /orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id} | Get a webhook delivery for an organization webhook
[**orgs_slash_list**](OrgsApi.md#orgs_slash_list) | **GET** /organizations | List organizations
[**orgs_slash_list_app_installations**](OrgsApi.md#orgs_slash_list_app_installations) | **GET** /orgs/{org}/installations | List app installations for an organization
[**orgs_slash_list_attestations**](OrgsApi.md#orgs_slash_list_attestations) | **GET** /orgs/{org}/attestations/{subject_digest} | List attestations
[**orgs_slash_list_blocked_users**](OrgsApi.md#orgs_slash_list_blocked_users) | **GET** /orgs/{org}/blocks | List users blocked by an organization
[**orgs_slash_list_custom_properties_values_for_repos**](OrgsApi.md#orgs_slash_list_custom_properties_values_for_repos) | **GET** /orgs/{org}/properties/values | List custom property values for organization repositories
[**orgs_slash_list_failed_invitations**](OrgsApi.md#orgs_slash_list_failed_invitations) | **GET** /orgs/{org}/failed_invitations | List failed organization invitations
[**orgs_slash_list_for_authenticated_user**](OrgsApi.md#orgs_slash_list_for_authenticated_user) | **GET** /user/orgs | List organizations for the authenticated user
[**orgs_slash_list_for_user**](OrgsApi.md#orgs_slash_list_for_user) | **GET** /users/{username}/orgs | List organizations for a user
[**orgs_slash_list_invitation_teams**](OrgsApi.md#orgs_slash_list_invitation_teams) | **GET** /orgs/{org}/invitations/{invitation_id}/teams | List organization invitation teams
[**orgs_slash_list_members**](OrgsApi.md#orgs_slash_list_members) | **GET** /orgs/{org}/members | List organization members
[**orgs_slash_list_memberships_for_authenticated_user**](OrgsApi.md#orgs_slash_list_memberships_for_authenticated_user) | **GET** /user/memberships/orgs | List organization memberships for the authenticated user
[**orgs_slash_list_org_role_teams**](OrgsApi.md#orgs_slash_list_org_role_teams) | **GET** /orgs/{org}/organization-roles/{role_id}/teams | List teams that are assigned to an organization role
[**orgs_slash_list_org_role_users**](OrgsApi.md#orgs_slash_list_org_role_users) | **GET** /orgs/{org}/organization-roles/{role_id}/users | List users that are assigned to an organization role
[**orgs_slash_list_org_roles**](OrgsApi.md#orgs_slash_list_org_roles) | **GET** /orgs/{org}/organization-roles | Get all organization roles for an organization
[**orgs_slash_list_outside_collaborators**](OrgsApi.md#orgs_slash_list_outside_collaborators) | **GET** /orgs/{org}/outside_collaborators | List outside collaborators for an organization
[**orgs_slash_list_pat_grant_repositories**](OrgsApi.md#orgs_slash_list_pat_grant_repositories) | **GET** /orgs/{org}/personal-access-tokens/{pat_id}/repositories | List repositories a fine-grained personal access token has access to
[**orgs_slash_list_pat_grant_request_repositories**](OrgsApi.md#orgs_slash_list_pat_grant_request_repositories) | **GET** /orgs/{org}/personal-access-token-requests/{pat_request_id}/repositories | List repositories requested to be accessed by a fine-grained personal access token
[**orgs_slash_list_pat_grant_requests**](OrgsApi.md#orgs_slash_list_pat_grant_requests) | **GET** /orgs/{org}/personal-access-token-requests | List requests to access organization resources with fine-grained personal access tokens
[**orgs_slash_list_pat_grants**](OrgsApi.md#orgs_slash_list_pat_grants) | **GET** /orgs/{org}/personal-access-tokens | List fine-grained personal access tokens with access to organization resources
[**orgs_slash_list_pending_invitations**](OrgsApi.md#orgs_slash_list_pending_invitations) | **GET** /orgs/{org}/invitations | List pending organization invitations
[**orgs_slash_list_public_members**](OrgsApi.md#orgs_slash_list_public_members) | **GET** /orgs/{org}/public_members | List public organization members
[**orgs_slash_list_security_manager_teams**](OrgsApi.md#orgs_slash_list_security_manager_teams) | **GET** /orgs/{org}/security-managers | List security manager teams
[**orgs_slash_list_webhook_deliveries**](OrgsApi.md#orgs_slash_list_webhook_deliveries) | **GET** /orgs/{org}/hooks/{hook_id}/deliveries | List deliveries for an organization webhook
[**orgs_slash_list_webhooks**](OrgsApi.md#orgs_slash_list_webhooks) | **GET** /orgs/{org}/hooks | List organization webhooks
[**orgs_slash_ping_webhook**](OrgsApi.md#orgs_slash_ping_webhook) | **POST** /orgs/{org}/hooks/{hook_id}/pings | Ping an organization webhook
[**orgs_slash_redeliver_webhook_delivery**](OrgsApi.md#orgs_slash_redeliver_webhook_delivery) | **POST** /orgs/{org}/hooks/{hook_id}/deliveries/{delivery_id}/attempts | Redeliver a delivery for an organization webhook
[**orgs_slash_remove_custom_property**](OrgsApi.md#orgs_slash_remove_custom_property) | **DELETE** /orgs/{org}/properties/schema/{custom_property_name} | Remove a custom property for an organization
[**orgs_slash_remove_member**](OrgsApi.md#orgs_slash_remove_member) | **DELETE** /orgs/{org}/members/{username} | Remove an organization member
[**orgs_slash_remove_membership_for_user**](OrgsApi.md#orgs_slash_remove_membership_for_user) | **DELETE** /orgs/{org}/memberships/{username} | Remove organization membership for a user
[**orgs_slash_remove_outside_collaborator**](OrgsApi.md#orgs_slash_remove_outside_collaborator) | **DELETE** /orgs/{org}/outside_collaborators/{username} | Remove outside collaborator from an organization
[**orgs_slash_remove_public_membership_for_authenticated_user**](OrgsApi.md#orgs_slash_remove_public_membership_for_authenticated_user) | **DELETE** /orgs/{org}/public_members/{username} | Remove public organization membership for the authenticated user
[**orgs_slash_remove_security_manager_team**](OrgsApi.md#orgs_slash_remove_security_manager_team) | **DELETE** /orgs/{org}/security-managers/teams/{team_slug} | Remove a security manager team
[**orgs_slash_review_pat_grant_request**](OrgsApi.md#orgs_slash_review_pat_grant_request) | **POST** /orgs/{org}/personal-access-token-requests/{pat_request_id} | Review a request to access organization resources with a fine-grained personal access token
[**orgs_slash_review_pat_grant_requests_in_bulk**](OrgsApi.md#orgs_slash_review_pat_grant_requests_in_bulk) | **POST** /orgs/{org}/personal-access-token-requests | Review requests to access organization resources with fine-grained personal access tokens
[**orgs_slash_revoke_all_org_roles_team**](OrgsApi.md#orgs_slash_revoke_all_org_roles_team) | **DELETE** /orgs/{org}/organization-roles/teams/{team_slug} | Remove all organization roles for a team
[**orgs_slash_revoke_all_org_roles_user**](OrgsApi.md#orgs_slash_revoke_all_org_roles_user) | **DELETE** /orgs/{org}/organization-roles/users/{username} | Remove all organization roles for a user
[**orgs_slash_revoke_org_role_team**](OrgsApi.md#orgs_slash_revoke_org_role_team) | **DELETE** /orgs/{org}/organization-roles/teams/{team_slug}/{role_id} | Remove an organization role from a team
[**orgs_slash_revoke_org_role_user**](OrgsApi.md#orgs_slash_revoke_org_role_user) | **DELETE** /orgs/{org}/organization-roles/users/{username}/{role_id} | Remove an organization role from a user
[**orgs_slash_set_membership_for_user**](OrgsApi.md#orgs_slash_set_membership_for_user) | **PUT** /orgs/{org}/memberships/{username} | Set organization membership for a user
[**orgs_slash_set_public_membership_for_authenticated_user**](OrgsApi.md#orgs_slash_set_public_membership_for_authenticated_user) | **PUT** /orgs/{org}/public_members/{username} | Set public organization membership for the authenticated user
[**orgs_slash_unblock_user**](OrgsApi.md#orgs_slash_unblock_user) | **DELETE** /orgs/{org}/blocks/{username} | Unblock a user from an organization
[**orgs_slash_update**](OrgsApi.md#orgs_slash_update) | **PATCH** /orgs/{org} | Update an organization
[**orgs_slash_update_membership_for_authenticated_user**](OrgsApi.md#orgs_slash_update_membership_for_authenticated_user) | **PATCH** /user/memberships/orgs/{org} | Update an organization membership for the authenticated user
[**orgs_slash_update_pat_access**](OrgsApi.md#orgs_slash_update_pat_access) | **POST** /orgs/{org}/personal-access-tokens/{pat_id} | Update the access a fine-grained personal access token has to organization resources
[**orgs_slash_update_pat_accesses**](OrgsApi.md#orgs_slash_update_pat_accesses) | **POST** /orgs/{org}/personal-access-tokens | Update the access to organization resources via fine-grained personal access tokens
[**orgs_slash_update_webhook**](OrgsApi.md#orgs_slash_update_webhook) | **PATCH** /orgs/{org}/hooks/{hook_id} | Update an organization webhook
[**orgs_slash_update_webhook_config_for_org**](OrgsApi.md#orgs_slash_update_webhook_config_for_org) | **PATCH** /orgs/{org}/hooks/{hook_id}/config | Update a webhook configuration for an organization



## api_insights_slash_get_route_stats_by_actor

> Vec<models::ApiInsightsRouteStatsInner> api_insights_slash_get_route_stats_by_actor(org, actor_type, actor_id, min_timestamp, max_timestamp, page, per_page, direction, sort)
Get route stats by actor

Get API request count statistics for an actor broken down by route within a specified time frame.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**actor_type** | **String** | The type of the actor | [required] |
**actor_id** | **i32** | The ID of the actor | [required] |
**min_timestamp** | **String** | The minimum timestamp to query for stats | [required] |
**max_timestamp** | **String** | The maximum timestamp to query for stats | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**sort** | Option<[**Vec<String>**](String.md)> | The property to sort the results by. |  |

### Return type

[**Vec<models::ApiInsightsRouteStatsInner>**](api_insights_route_stats_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_insights_slash_get_subject_stats

> Vec<models::ApiInsightsSubjectStatsInner> api_insights_slash_get_subject_stats(org, min_timestamp, max_timestamp, page, per_page, direction, sort)
Get subject stats

Get API request statistics for all subjects within an organization within a specified time frame. Subjects can be users or GitHub Apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**min_timestamp** | **String** | The minimum timestamp to query for stats | [required] |
**max_timestamp** | **String** | The maximum timestamp to query for stats | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**sort** | Option<[**Vec<String>**](String.md)> | The property to sort the results by. |  |

### Return type

[**Vec<models::ApiInsightsSubjectStatsInner>**](api_insights_subject_stats_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_insights_slash_get_summary_stats

> models::ApiInsightsSummaryStats api_insights_slash_get_summary_stats(org, min_timestamp, max_timestamp)
Get summary stats

Get overall statistics of API requests made within an organization by all users and apps within a specified time frame.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**min_timestamp** | **String** | The minimum timestamp to query for stats | [required] |
**max_timestamp** | **String** | The maximum timestamp to query for stats | [required] |

### Return type

[**models::ApiInsightsSummaryStats**](api-insights-summary-stats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_insights_slash_get_summary_stats_by_actor

> models::ApiInsightsSummaryStats api_insights_slash_get_summary_stats_by_actor(org, min_timestamp, max_timestamp, actor_type, actor_id)
Get summary stats by actor

Get overall statistics of API requests within the organization made by a specific actor. Actors can be GitHub App installations, OAuth apps or other tokens on behalf of a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**min_timestamp** | **String** | The minimum timestamp to query for stats | [required] |
**max_timestamp** | **String** | The maximum timestamp to query for stats | [required] |
**actor_type** | **String** | The type of the actor | [required] |
**actor_id** | **i32** | The ID of the actor | [required] |

### Return type

[**models::ApiInsightsSummaryStats**](api-insights-summary-stats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_insights_slash_get_summary_stats_by_user

> models::ApiInsightsSummaryStats api_insights_slash_get_summary_stats_by_user(org, user_id, min_timestamp, max_timestamp)
Get summary stats by user

Get overall statistics of API requests within the organization for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**user_id** | **String** | The ID of the user to query for stats | [required] |
**min_timestamp** | **String** | The minimum timestamp to query for stats | [required] |
**max_timestamp** | **String** | The maximum timestamp to query for stats | [required] |

### Return type

[**models::ApiInsightsSummaryStats**](api-insights-summary-stats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_insights_slash_get_time_stats

> Vec<models::ApiInsightsTimeStatsInner> api_insights_slash_get_time_stats(org, min_timestamp, max_timestamp, timestamp_increment)
Get time stats

Get the number of API requests and rate-limited requests made within an organization over a specified time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**min_timestamp** | **String** | The minimum timestamp to query for stats | [required] |
**max_timestamp** | **String** | The maximum timestamp to query for stats | [required] |
**timestamp_increment** | **String** | The increment of time used to breakdown the query results (5m, 10m, 1h, etc.) | [required] |

### Return type

[**Vec<models::ApiInsightsTimeStatsInner>**](api_insights_time_stats_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_insights_slash_get_time_stats_by_actor

> Vec<models::ApiInsightsTimeStatsInner> api_insights_slash_get_time_stats_by_actor(org, actor_type, actor_id, min_timestamp, max_timestamp, timestamp_increment)
Get time stats by actor

Get the number of API requests and rate-limited requests made within an organization by a specific actor within a specified time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**actor_type** | **String** | The type of the actor | [required] |
**actor_id** | **i32** | The ID of the actor | [required] |
**min_timestamp** | **String** | The minimum timestamp to query for stats | [required] |
**max_timestamp** | **String** | The maximum timestamp to query for stats | [required] |
**timestamp_increment** | **String** | The increment of time used to breakdown the query results (5m, 10m, 1h, etc.) | [required] |

### Return type

[**Vec<models::ApiInsightsTimeStatsInner>**](api_insights_time_stats_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_insights_slash_get_time_stats_by_user

> Vec<models::ApiInsightsTimeStatsInner> api_insights_slash_get_time_stats_by_user(org, user_id, min_timestamp, max_timestamp, timestamp_increment)
Get time stats by user

Get the number of API requests and rate-limited requests made within an organization by a specific user over a specified time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**user_id** | **String** | The ID of the user to query for stats | [required] |
**min_timestamp** | **String** | The minimum timestamp to query for stats | [required] |
**max_timestamp** | **String** | The maximum timestamp to query for stats | [required] |
**timestamp_increment** | **String** | The increment of time used to breakdown the query results (5m, 10m, 1h, etc.) | [required] |

### Return type

[**Vec<models::ApiInsightsTimeStatsInner>**](api_insights_time_stats_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_insights_slash_get_user_stats

> Vec<models::ApiInsightsUserStatsInner> api_insights_slash_get_user_stats(org, user_id, min_timestamp, max_timestamp, page, per_page, direction, sort)
Get user stats

Get API usage statistics within an organization for a user broken down by the type of access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**user_id** | **String** | The ID of the user to query for stats | [required] |
**min_timestamp** | **String** | The minimum timestamp to query for stats | [required] |
**max_timestamp** | **String** | The maximum timestamp to query for stats | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**sort** | Option<[**Vec<String>**](String.md)> | The property to sort the results by. |  |

### Return type

[**Vec<models::ApiInsightsUserStatsInner>**](api_insights_user_stats_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_add_security_manager_team

> orgs_slash_add_security_manager_team(org, team_slug)
Add a security manager team

Adds a team as a security manager for an organization. For more information, see \"[Managing security for an organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization) for an organization.\"  The authenticated user must be an administrator for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

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


## orgs_slash_assign_team_to_org_role

> orgs_slash_assign_team_to_org_role(org, team_slug, role_id)
Assign an organization role to a team

Assigns an organization role to a team in an organization. For more information on organization roles, see \"[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles).\"  The authenticated user must be an administrator for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**role_id** | **i32** | The unique identifier of the role. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_assign_user_to_org_role

> orgs_slash_assign_user_to_org_role(org, username, role_id)
Assign an organization role to a user

Assigns an organization role to a member of an organization. For more information on organization roles, see \"[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles).\"  The authenticated user must be an administrator for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**role_id** | **i32** | The unique identifier of the role. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_block_user

> orgs_slash_block_user(org, username)
Block a user from an organization

Blocks the given user on behalf of the specified organization and returns a 204. If the organization cannot block the given user a 422 is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_cancel_invitation

> orgs_slash_cancel_invitation(org, invitation_id)
Cancel an organization invitation

Cancel an organization invitation. In order to cancel an organization invitation, the authenticated user must be an organization owner.  This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**invitation_id** | **i32** | The unique identifier of the invitation. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_check_blocked_user

> orgs_slash_check_blocked_user(org, username)
Check if a user is blocked by an organization

Returns a 204 if the given user is blocked by the given organization. Returns a 404 if the organization is not blocking the user, or if the user account has been identified as spam by GitHub.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_check_membership_for_user

> orgs_slash_check_membership_for_user(org, username)
Check organization membership for a user

Check if a user is, publicly or privately, a member of the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_check_public_membership_for_user

> orgs_slash_check_public_membership_for_user(org, username)
Check public organization membership for a user

Check if the provided user is a public member of the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_convert_member_to_outside_collaborator

> serde_json::Value orgs_slash_convert_member_to_outside_collaborator(org, username, orgs_convert_member_to_outside_collaborator_request)
Convert an organization member to outside collaborator

When an organization member is converted to an outside collaborator, they'll only have access to the repositories that their current team membership allows. The user will no longer be a member of the organization. For more information, see \"[Converting an organization member to an outside collaborator](https://docs.github.com/articles/converting-an-organization-member-to-an-outside-collaborator/)\". Converting an organization member to an outside collaborator may be restricted by enterprise administrators. For more information, see \"[Enforcing repository management policies in your enterprise](https://docs.github.com/admin/policies/enforcing-policies-for-your-enterprise/enforcing-repository-management-policies-in-your-enterprise#enforcing-a-policy-for-inviting-outside-collaborators-to-repositories).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**orgs_convert_member_to_outside_collaborator_request** | Option<[**OrgsConvertMemberToOutsideCollaboratorRequest**](OrgsConvertMemberToOutsideCollaboratorRequest.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_create_invitation

> models::OrganizationInvitation orgs_slash_create_invitation(org, orgs_create_invitation_request)
Create an organization invitation

Invite people to an organization by using their GitHub user ID or their email address. In order to create invitations in an organization, the authenticated user must be an organization owner.  This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see \"[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)\" and \"[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**orgs_create_invitation_request** | Option<[**OrgsCreateInvitationRequest**](OrgsCreateInvitationRequest.md)> |  |  |

### Return type

[**models::OrganizationInvitation**](organization-invitation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_create_or_update_custom_properties

> Vec<models::CustomProperty> orgs_slash_create_or_update_custom_properties(org, orgs_create_or_update_custom_properties_request)
Create or update custom properties for an organization

Creates new or updates existing custom properties defined for an organization in a batch.  To use this endpoint, the authenticated user must be one of:   - An administrator for the organization.   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**orgs_create_or_update_custom_properties_request** | [**OrgsCreateOrUpdateCustomPropertiesRequest**](OrgsCreateOrUpdateCustomPropertiesRequest.md) |  | [required] |

### Return type

[**Vec<models::CustomProperty>**](custom-property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_create_or_update_custom_properties_values_for_repos

> orgs_slash_create_or_update_custom_properties_values_for_repos(org, orgs_create_or_update_custom_properties_values_for_repos_request)
Create or update custom property values for organization repositories

Create new or update existing custom property values for repositories in a batch that belong to an organization. Each target repository will have its custom property values updated to match the values provided in the request.  A maximum of 30 repositories can be updated in a single request.  Using a value of `null` for a custom property will remove or 'unset' the property value from the repository.  To use this endpoint, the authenticated user must be one of:   - An administrator for the organization.   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_values_editor` in the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**orgs_create_or_update_custom_properties_values_for_repos_request** | [**OrgsCreateOrUpdateCustomPropertiesValuesForReposRequest**](OrgsCreateOrUpdateCustomPropertiesValuesForReposRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_create_or_update_custom_property

> models::CustomProperty orgs_slash_create_or_update_custom_property(org, custom_property_name, orgs_create_or_update_custom_property_request)
Create or update a custom property for an organization

Creates a new or updates an existing custom property that is defined for an organization.  To use this endpoint, the authenticated user must be one of: - An administrator for the organization. - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**custom_property_name** | **String** | The custom property name | [required] |
**orgs_create_or_update_custom_property_request** | [**OrgsCreateOrUpdateCustomPropertyRequest**](OrgsCreateOrUpdateCustomPropertyRequest.md) |  | [required] |

### Return type

[**models::CustomProperty**](custom-property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_create_webhook

> models::OrgHook orgs_slash_create_webhook(org, orgs_create_webhook_request)
Create an organization webhook

Create a hook that posts payloads in JSON format.  You must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**orgs_create_webhook_request** | [**OrgsCreateWebhookRequest**](OrgsCreateWebhookRequest.md) |  | [required] |

### Return type

[**models::OrgHook**](org-hook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_delete

> serde_json::Value orgs_slash_delete(org)
Delete an organization

Deletes an organization and all its repositories.  The organization login will be unavailable for 90 days after deletion.  Please review the Terms of Service regarding account deletion before using this endpoint:  https://docs.github.com/site-policy/github-terms/github-terms-of-service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_delete_webhook

> orgs_slash_delete_webhook(org, hook_id)
Delete an organization webhook

You must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_enable_or_disable_security_product_on_all_org_repos

> orgs_slash_enable_or_disable_security_product_on_all_org_repos(org, security_product, enablement, orgs_enable_or_disable_security_product_on_all_org_repos_request)
Enable or disable a security feature for an organization

> [!WARNING] > **Closing down notice:** The ability to enable or disable a security feature for all eligible repositories in an organization is closing down. Please use [code security configurations](https://docs.github.com/rest/code-security/configurations) instead. For more information, see the [changelog](https://github.blog/changelog/2024-07-22-deprecation-of-api-endpoint-to-enable-or-disable-a-security-feature-for-an-organization/).  Enables or disables the specified security feature for all eligible repositories in an organization. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  The authenticated user must be an organization owner or be member of a team with the security manager role to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org`, `write:org`, or `repo` scopes to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**security_product** | **String** | The security feature to enable or disable. | [required] |
**enablement** | **String** | The action to take.  `enable_all` means to enable the specified security feature for all repositories in the organization. `disable_all` means to disable the specified security feature for all repositories in the organization. | [required] |
**orgs_enable_or_disable_security_product_on_all_org_repos_request** | Option<[**OrgsEnableOrDisableSecurityProductOnAllOrgReposRequest**](OrgsEnableOrDisableSecurityProductOnAllOrgReposRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_get

> models::OrganizationFull orgs_slash_get(org)
Get an organization

Gets information about an organization.  When the value of `two_factor_requirement_enabled` is `true`, the organization requires all members, billing managers, and outside collaborators to enable [two-factor authentication](https://docs.github.com/articles/securing-your-account-with-two-factor-authentication-2fa/).  To see the full details about an organization, the authenticated user must be an organization owner.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to see the full details about an organization.  To see information about an organization's GitHub plan, GitHub Apps need the `Organization plan` permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::OrganizationFull**](organization-full.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_get_all_custom_properties

> Vec<models::CustomProperty> orgs_slash_get_all_custom_properties(org)
Get all custom properties for an organization

Gets all custom properties defined for an organization. Organization members can read these properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::CustomProperty>**](custom-property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_get_custom_property

> models::CustomProperty orgs_slash_get_custom_property(org, custom_property_name)
Get a custom property for an organization

Gets a custom property that is defined for an organization. Organization members can read these properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**custom_property_name** | **String** | The custom property name | [required] |

### Return type

[**models::CustomProperty**](custom-property.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_get_membership_for_authenticated_user

> models::OrgMembership orgs_slash_get_membership_for_authenticated_user(org)
Get an organization membership for the authenticated user

If the authenticated user is an active or pending member of the organization, this endpoint will return the user's membership. If the authenticated user is not affiliated with the organization, a `404` is returned. This endpoint will return a `403` if the request is made by a GitHub App that is blocked by the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::OrgMembership**](org-membership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_get_membership_for_user

> models::OrgMembership orgs_slash_get_membership_for_user(org, username)
Get organization membership for a user

In order to get a user's membership with an organization, the authenticated user must be an organization member. The `state` parameter in the response can be used to identify the user's membership status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::OrgMembership**](org-membership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_get_org_role

> models::OrganizationRole orgs_slash_get_org_role(org, role_id)
Get an organization role

Gets an organization role that is available to this organization. For more information on organization roles, see \"[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles).\"  To use this endpoint, the authenticated user must be one of:  - An administrator for the organization. - A user, or a user on a team, with the fine-grained permissions of `read_organization_custom_org_role` in the organization.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**role_id** | **i32** | The unique identifier of the role. | [required] |

### Return type

[**models::OrganizationRole**](organization-role.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_get_webhook

> models::OrgHook orgs_slash_get_webhook(org, hook_id)
Get an organization webhook

Returns a webhook configured in an organization. To get only the webhook `config` properties, see \"[Get a webhook configuration for an organization](/rest/orgs/webhooks#get-a-webhook-configuration-for-an-organization).  You must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |

### Return type

[**models::OrgHook**](org-hook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_get_webhook_config_for_org

> models::WebhookConfig orgs_slash_get_webhook_config_for_org(org, hook_id)
Get a webhook configuration for an organization

Returns the webhook configuration for an organization. To get more information about the webhook, including the `active` state and `events`, use \"[Get an organization webhook ](/rest/orgs/webhooks#get-an-organization-webhook).\"  You must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |

### Return type

[**models::WebhookConfig**](webhook-config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_get_webhook_delivery

> models::HookDelivery orgs_slash_get_webhook_delivery(org, hook_id, delivery_id)
Get a webhook delivery for an organization webhook

Returns a delivery for a webhook configured in an organization.  You must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |
**delivery_id** | **i32** |  | [required] |

### Return type

[**models::HookDelivery**](hook-delivery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list

> Vec<models::OrganizationSimple> orgs_slash_list(since, per_page)
List organizations

Lists all organizations, in the order that they were created.  > [!NOTE] > Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of organizations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**i32**> | An organization ID. Only return organizations with an ID greater than this ID. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**Vec<models::OrganizationSimple>**](organization-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_app_installations

> models::OrgsListAppInstallations200Response orgs_slash_list_app_installations(org, per_page, page)
List app installations for an organization

Lists all GitHub Apps in an organization. The installation count includes all GitHub Apps installed on repositories in the organization.  The authenticated user must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:read` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::OrgsListAppInstallations200Response**](orgs_list_app_installations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_attestations

> models::OrgsListAttestations200Response orgs_slash_list_attestations(org, subject_digest, per_page, before, after)
List attestations

List a collection of artifact attestations with a given subject digest that are associated with repositories owned by an organization.  The collection of attestations returned by this endpoint is filtered according to the authenticated user's permissions; if the authenticated user cannot read a repository, the attestations associated with that repository will not be included in the response. In addition, when using a fine-grained access token the `attestations:read` permission is required.  **Please note:** in order to offer meaningful security benefits, an attestation's signature and timestamps **must** be cryptographically verified, and the identity of the attestation signer **must** be validated. Attestations can be verified using the [GitHub CLI `attestation verify` command](https://cli.github.com/manual/gh_attestation_verify). For more information, see [our guide on how to use artifact attestations to establish a build's provenance](https://docs.github.com/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**subject_digest** | **String** | The parameter should be set to the attestation's subject's SHA256 digest, in the form `sha256:HEX_DIGEST`. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |

### Return type

[**models::OrgsListAttestations200Response**](orgs_list_attestations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_blocked_users

> Vec<models::SimpleUser> orgs_slash_list_blocked_users(org, per_page, page)
List users blocked by an organization

List the users blocked by an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
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


## orgs_slash_list_custom_properties_values_for_repos

> Vec<models::OrgRepoCustomPropertyValues> orgs_slash_list_custom_properties_values_for_repos(org, per_page, page, repository_query)
List custom property values for organization repositories

Lists organization repositories with all of their custom property values. Organization members can read these properties.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**repository_query** | Option<**String**> | Finds repositories in the organization with a query containing one or more search keywords and qualifiers. Qualifiers allow you to limit your search to specific areas of GitHub. The REST API supports the same qualifiers as the web interface for GitHub. To learn more about the format of the query, see [Constructing a search query](https://docs.github.com/rest/search/search#constructing-a-search-query). See \"[Searching for repositories](https://docs.github.com/articles/searching-for-repositories/)\" for a detailed list of qualifiers. |  |

### Return type

[**Vec<models::OrgRepoCustomPropertyValues>**](org-repo-custom-property-values.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_failed_invitations

> Vec<models::OrganizationInvitation> orgs_slash_list_failed_invitations(org, per_page, page)
List failed organization invitations

The return hash contains `failed_at` and `failed_reason` fields which represent the time at which the invitation failed and the reason for the failure.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
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


## orgs_slash_list_for_authenticated_user

> Vec<models::OrganizationSimple> orgs_slash_list_for_authenticated_user(per_page, page)
List organizations for the authenticated user

List organizations for the authenticated user.  For OAuth app tokens and personal access tokens (classic), this endpoint only lists organizations that your authorization allows you to operate on in some way (e.g., you can list teams with `read:org` scope, you can publicize your organization membership with `user` scope, etc.). Therefore, this API requires at least `user` or `read:org` scope for OAuth app tokens and personal access tokens (classic). Requests with insufficient scope will receive a `403 Forbidden` response.  > [!NOTE] > Requests using a fine-grained access token will receive a `200 Success` response with an empty list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::OrganizationSimple>**](organization-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_for_user

> Vec<models::OrganizationSimple> orgs_slash_list_for_user(username, per_page, page)
List organizations for a user

List [public organization memberships](https://docs.github.com/articles/publicizing-or-concealing-organization-membership) for the specified user.  This method only lists _public_ memberships, regardless of authentication. If you need to fetch all of the organization memberships (public and private) for the authenticated user, use the [List organizations for the authenticated user](https://docs.github.com/rest/orgs/orgs#list-organizations-for-the-authenticated-user) API instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::OrganizationSimple>**](organization-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_invitation_teams

> Vec<models::Team> orgs_slash_list_invitation_teams(org, invitation_id, per_page, page)
List organization invitation teams

List all teams associated with an invitation. In order to see invitations in an organization, the authenticated user must be an organization owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**invitation_id** | **i32** | The unique identifier of the invitation. | [required] |
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


## orgs_slash_list_members

> Vec<models::SimpleUser> orgs_slash_list_members(org, filter, role, per_page, page)
List organization members

List all users who are members of an organization. If the authenticated user is also a member of this organization then both concealed and public members will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**filter** | Option<**String**> | Filter members returned in the list. `2fa_disabled` means that only members without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled will be returned. This options is only available for organization owners. |  |[default to all]
**role** | Option<**String**> | Filter members returned by their role. |  |[default to all]
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


## orgs_slash_list_memberships_for_authenticated_user

> Vec<models::OrgMembership> orgs_slash_list_memberships_for_authenticated_user(state, per_page, page)
List organization memberships for the authenticated user

Lists all of the authenticated user's organization memberships.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state** | Option<**String**> | Indicates the state of the memberships to return. If not specified, the API returns both active and pending memberships. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::OrgMembership>**](org-membership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_org_role_teams

> Vec<models::TeamRoleAssignment> orgs_slash_list_org_role_teams(org, role_id, per_page, page)
List teams that are assigned to an organization role

Lists the teams that are assigned to an organization role. For more information on organization roles, see \"[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles).\"  To use this endpoint, you must be an administrator for the organization.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**role_id** | **i32** | The unique identifier of the role. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::TeamRoleAssignment>**](team-role-assignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_org_role_users

> Vec<models::UserRoleAssignment> orgs_slash_list_org_role_users(org, role_id, per_page, page)
List users that are assigned to an organization role

Lists organization members that are assigned to an organization role. For more information on organization roles, see \"[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles).\"  To use this endpoint, you must be an administrator for the organization.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**role_id** | **i32** | The unique identifier of the role. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::UserRoleAssignment>**](user-role-assignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_org_roles

> models::OrgsListOrgRoles200Response orgs_slash_list_org_roles(org)
Get all organization roles for an organization

Lists the organization roles available in this organization. For more information on organization roles, see \"[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles).\"  To use this endpoint, the authenticated user must be one of:  - An administrator for the organization. - A user, or a user on a team, with the fine-grained permissions of `read_organization_custom_org_role` in the organization.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::OrgsListOrgRoles200Response**](orgs_list_org_roles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_outside_collaborators

> Vec<models::SimpleUser> orgs_slash_list_outside_collaborators(org, filter, per_page, page)
List outside collaborators for an organization

List all users who are outside collaborators of an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**filter** | Option<**String**> | Filter the list of outside collaborators. `2fa_disabled` means that only outside collaborators without [two-factor authentication](https://github.com/blog/1614-two-factor-authentication) enabled will be returned. |  |[default to all]
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


## orgs_slash_list_pat_grant_repositories

> Vec<models::MinimalRepository> orgs_slash_list_pat_grant_repositories(org, pat_id, per_page, page)
List repositories a fine-grained personal access token has access to

Lists the repositories a fine-grained personal access token has access to.  Only GitHub Apps can use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**pat_id** | **i32** | Unique identifier of the fine-grained personal access token. | [required] |
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


## orgs_slash_list_pat_grant_request_repositories

> Vec<models::MinimalRepository> orgs_slash_list_pat_grant_request_repositories(org, pat_request_id, per_page, page)
List repositories requested to be accessed by a fine-grained personal access token

Lists the repositories a fine-grained personal access token request is requesting access to.  Only GitHub Apps can use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**pat_request_id** | **i32** | Unique identifier of the request for access via fine-grained personal access token. | [required] |
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


## orgs_slash_list_pat_grant_requests

> Vec<models::OrganizationProgrammaticAccessGrantRequest> orgs_slash_list_pat_grant_requests(org, per_page, page, sort, direction, owner, repository, permission, last_used_before, last_used_after)
List requests to access organization resources with fine-grained personal access tokens

Lists requests from organization members to access organization resources with a fine-grained personal access token.  Only GitHub Apps can use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**sort** | Option<**String**> | The property by which to sort the results. |  |[default to created_at]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**owner** | Option<[**Vec<String>**](String.md)> | A list of owner usernames to use to filter the results. |  |
**repository** | Option<**String**> | The name of the repository to use to filter the results. |  |
**permission** | Option<**String**> | The permission to use to filter the results. |  |
**last_used_before** | Option<**String**> | Only show fine-grained personal access tokens used before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**last_used_after** | Option<**String**> | Only show fine-grained personal access tokens used after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |

### Return type

[**Vec<models::OrganizationProgrammaticAccessGrantRequest>**](organization-programmatic-access-grant-request.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_pat_grants

> Vec<models::OrganizationProgrammaticAccessGrant> orgs_slash_list_pat_grants(org, per_page, page, sort, direction, owner, repository, permission, last_used_before, last_used_after)
List fine-grained personal access tokens with access to organization resources

Lists approved fine-grained personal access tokens owned by organization members that can access organization resources.  Only GitHub Apps can use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**sort** | Option<**String**> | The property by which to sort the results. |  |[default to created_at]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**owner** | Option<[**Vec<String>**](String.md)> | A list of owner usernames to use to filter the results. |  |
**repository** | Option<**String**> | The name of the repository to use to filter the results. |  |
**permission** | Option<**String**> | The permission to use to filter the results. |  |
**last_used_before** | Option<**String**> | Only show fine-grained personal access tokens used before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**last_used_after** | Option<**String**> | Only show fine-grained personal access tokens used after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |

### Return type

[**Vec<models::OrganizationProgrammaticAccessGrant>**](organization-programmatic-access-grant.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_pending_invitations

> Vec<models::OrganizationInvitation> orgs_slash_list_pending_invitations(org, per_page, page, role, invitation_source)
List pending organization invitations

The return hash contains a `role` field which refers to the Organization Invitation role and will be one of the following values: `direct_member`, `admin`, `billing_manager`, or `hiring_manager`. If the invitee is not a GitHub member, the `login` field in the return hash will be `null`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**role** | Option<**String**> | Filter invitations by their member role. |  |[default to all]
**invitation_source** | Option<**String**> | Filter invitations by their invitation source. |  |[default to all]

### Return type

[**Vec<models::OrganizationInvitation>**](organization-invitation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_public_members

> Vec<models::SimpleUser> orgs_slash_list_public_members(org, per_page, page)
List public organization members

Members of an organization can choose to have their membership publicized or not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
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


## orgs_slash_list_security_manager_teams

> Vec<models::TeamSimple> orgs_slash_list_security_manager_teams(org)
List security manager teams

Lists teams that are security managers for an organization. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::TeamSimple>**](team-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_webhook_deliveries

> Vec<models::HookDeliveryItem> orgs_slash_list_webhook_deliveries(org, hook_id, per_page, cursor)
List deliveries for an organization webhook

Returns a list of webhook deliveries for a webhook configured in an organization.  You must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**cursor** | Option<**String**> | Used for pagination: the starting delivery from which the page of deliveries is fetched. Refer to the `link` header for the next and previous page cursors. |  |

### Return type

[**Vec<models::HookDeliveryItem>**](hook-delivery-item.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_list_webhooks

> Vec<models::OrgHook> orgs_slash_list_webhooks(org, per_page, page)
List organization webhooks

You must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::OrgHook>**](org-hook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_ping_webhook

> orgs_slash_ping_webhook(org, hook_id)
Ping an organization webhook

This will trigger a [ping event](https://docs.github.com/webhooks/#ping-event) to be sent to the hook.  You must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_redeliver_webhook_delivery

> serde_json::Value orgs_slash_redeliver_webhook_delivery(org, hook_id, delivery_id)
Redeliver a delivery for an organization webhook

Redeliver a delivery for a webhook configured in an organization.  You must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |
**delivery_id** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_remove_custom_property

> orgs_slash_remove_custom_property(org, custom_property_name)
Remove a custom property for an organization

Removes a custom property that is defined for an organization.  To use this endpoint, the authenticated user must be one of:   - An administrator for the organization.   - A user, or a user on a team, with the fine-grained permission of `custom_properties_org_definitions_manager` in the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**custom_property_name** | **String** | The custom property name | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_remove_member

> orgs_slash_remove_member(org, username)
Remove an organization member

Removing a user from this list will remove them from all teams and they will no longer have any access to the organization's repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_remove_membership_for_user

> orgs_slash_remove_membership_for_user(org, username)
Remove organization membership for a user

In order to remove a user's membership with an organization, the authenticated user must be an organization owner.  If the specified user is an active member of the organization, this will remove them from the organization. If the specified user has been invited to the organization, this will cancel their invitation. The specified user will receive an email notification in both cases.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_remove_outside_collaborator

> orgs_slash_remove_outside_collaborator(org, username)
Remove outside collaborator from an organization

Removing a user from this list will remove them from all the organization's repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_remove_public_membership_for_authenticated_user

> orgs_slash_remove_public_membership_for_authenticated_user(org, username)
Remove public organization membership for the authenticated user

Removes the public membership for the authenticated user from the specified organization, unless public visibility is enforced by default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_remove_security_manager_team

> orgs_slash_remove_security_manager_team(org, team_slug)
Remove a security manager team

Removes the security manager role from a team for an organization. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization) team from an organization.\"  The authenticated user must be an administrator for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

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


## orgs_slash_review_pat_grant_request

> orgs_slash_review_pat_grant_request(org, pat_request_id, orgs_review_pat_grant_request_request)
Review a request to access organization resources with a fine-grained personal access token

Approves or denies a pending request to access organization resources via a fine-grained personal access token.  Only GitHub Apps can use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**pat_request_id** | **i32** | Unique identifier of the request for access via fine-grained personal access token. | [required] |
**orgs_review_pat_grant_request_request** | [**OrgsReviewPatGrantRequestRequest**](OrgsReviewPatGrantRequestRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_review_pat_grant_requests_in_bulk

> serde_json::Value orgs_slash_review_pat_grant_requests_in_bulk(org, orgs_review_pat_grant_requests_in_bulk_request)
Review requests to access organization resources with fine-grained personal access tokens

Approves or denies multiple pending requests to access organization resources via a fine-grained personal access token.  Only GitHub Apps can use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**orgs_review_pat_grant_requests_in_bulk_request** | [**OrgsReviewPatGrantRequestsInBulkRequest**](OrgsReviewPatGrantRequestsInBulkRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_revoke_all_org_roles_team

> orgs_slash_revoke_all_org_roles_team(org, team_slug)
Remove all organization roles for a team

Removes all assigned organization roles from a team. For more information on organization roles, see \"[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles).\"  The authenticated user must be an administrator for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

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


## orgs_slash_revoke_all_org_roles_user

> orgs_slash_revoke_all_org_roles_user(org, username)
Remove all organization roles for a user

Revokes all assigned organization roles from a user. For more information on organization roles, see \"[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles).\"  The authenticated user must be an administrator for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_revoke_org_role_team

> orgs_slash_revoke_org_role_team(org, team_slug, role_id)
Remove an organization role from a team

Removes an organization role from a team. For more information on organization roles, see \"[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles).\"  The authenticated user must be an administrator for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**role_id** | **i32** | The unique identifier of the role. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_revoke_org_role_user

> orgs_slash_revoke_org_role_user(org, username, role_id)
Remove an organization role from a user

Remove an organization role from a user. For more information on organization roles, see \"[Using organization roles](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/using-organization-roles).\"  The authenticated user must be an administrator for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**role_id** | **i32** | The unique identifier of the role. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_set_membership_for_user

> models::OrgMembership orgs_slash_set_membership_for_user(org, username, orgs_set_membership_for_user_request)
Set organization membership for a user

Only authenticated organization owners can add a member to the organization or update the member's role.  *   If the authenticated user is _adding_ a member to the organization, the invited user will receive an email inviting them to the organization. The user's [membership status](https://docs.github.com/rest/orgs/members#get-organization-membership-for-a-user) will be `pending` until they accept the invitation.      *   Authenticated users can _update_ a user's membership by passing the `role` parameter. If the authenticated user changes a member's role to `admin`, the affected user will receive an email notifying them that they've been made an organization owner. If the authenticated user changes an owner's role to `member`, no email will be sent.  **Rate limits**  To prevent abuse, organization owners are limited to creating 50 organization invitations for an organization within a 24 hour period. If the organization is more than one month old or on a paid plan, the limit is 500 invitations per 24 hour period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**orgs_set_membership_for_user_request** | Option<[**OrgsSetMembershipForUserRequest**](OrgsSetMembershipForUserRequest.md)> |  |  |

### Return type

[**models::OrgMembership**](org-membership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_set_public_membership_for_authenticated_user

> orgs_slash_set_public_membership_for_authenticated_user(org, username)
Set public organization membership for the authenticated user

The user can publicize their own membership. (A user cannot publicize the membership for another user.)  Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_unblock_user

> orgs_slash_unblock_user(org, username)
Unblock a user from an organization

Unblocks the given user on behalf of the specified organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_update

> models::OrganizationFull orgs_slash_update(org, orgs_update_request)
Update an organization

> [!WARNING] > **Closing down notice:** GitHub will replace and discontinue `members_allowed_repository_creation_type` in favor of more granular permissions. The new input parameters are `members_can_create_public_repositories`, `members_can_create_private_repositories` for all organizations and `members_can_create_internal_repositories` for organizations associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+. For more information, see the [blog post](https://developer.github.com/changes/2019-12-03-internal-visibility-changes).  > [!WARNING] > **Closing down notice:** Code security product enablement for new repositories through the organization API is closing down. Please use [code security configurations](https://docs.github.com/rest/code-security/configurations#set-a-code-security-configuration-as-a-default-for-an-organization) to set defaults instead. For more information on setting a default security configuration, see the [changelog](https://github.blog/changelog/2024-07-09-sunsetting-security-settings-defaults-parameters-in-the-organizations-rest-api/).  Updates the organization's profile and member privileges.  The authenticated user must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `admin:org` or `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**orgs_update_request** | Option<[**OrgsUpdateRequest**](OrgsUpdateRequest.md)> |  |  |

### Return type

[**models::OrganizationFull**](organization-full.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_update_membership_for_authenticated_user

> models::OrgMembership orgs_slash_update_membership_for_authenticated_user(org, orgs_update_membership_for_authenticated_user_request)
Update an organization membership for the authenticated user

Converts the authenticated user to an active member of the organization, if that user has a pending invitation from the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**orgs_update_membership_for_authenticated_user_request** | [**OrgsUpdateMembershipForAuthenticatedUserRequest**](OrgsUpdateMembershipForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**models::OrgMembership**](org-membership.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_update_pat_access

> orgs_slash_update_pat_access(org, pat_id, orgs_update_pat_access_request)
Update the access a fine-grained personal access token has to organization resources

Updates the access an organization member has to organization resources via a fine-grained personal access token. Limited to revoking the token's existing access. Limited to revoking a token's existing access.  Only GitHub Apps can use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**pat_id** | **i32** | The unique identifier of the fine-grained personal access token. | [required] |
**orgs_update_pat_access_request** | [**OrgsUpdatePatAccessRequest**](OrgsUpdatePatAccessRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_update_pat_accesses

> serde_json::Value orgs_slash_update_pat_accesses(org, orgs_update_pat_accesses_request)
Update the access to organization resources via fine-grained personal access tokens

Updates the access organization members have to organization resources via fine-grained personal access tokens. Limited to revoking a token's existing access.  Only GitHub Apps can use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**orgs_update_pat_accesses_request** | [**OrgsUpdatePatAccessesRequest**](OrgsUpdatePatAccessesRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_update_webhook

> models::OrgHook orgs_slash_update_webhook(org, hook_id, orgs_update_webhook_request)
Update an organization webhook

Updates a webhook configured in an organization. When you update a webhook, the `secret` will be overwritten. If you previously had a `secret` set, you must provide the same `secret` or set a new `secret` or the secret will be removed. If you are only updating individual webhook `config` properties, use \"[Update a webhook configuration for an organization](/rest/orgs/webhooks#update-a-webhook-configuration-for-an-organization)\".  You must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |
**orgs_update_webhook_request** | Option<[**OrgsUpdateWebhookRequest**](OrgsUpdateWebhookRequest.md)> |  |  |

### Return type

[**models::OrgHook**](org-hook.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## orgs_slash_update_webhook_config_for_org

> models::WebhookConfig orgs_slash_update_webhook_config_for_org(org, hook_id, apps_update_webhook_config_for_app_request)
Update a webhook configuration for an organization

Updates the webhook configuration for an organization. To update more information about the webhook, including the `active` state and `events`, use \"[Update an organization webhook ](/rest/orgs/webhooks#update-an-organization-webhook).\"  You must be an organization owner to use this endpoint.  OAuth app tokens and personal access tokens (classic) need `admin:org_hook` scope. OAuth apps cannot list, view, or edit webhooks that they did not create and users cannot list, view, or edit webhooks that were created by OAuth apps.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**hook_id** | **i32** | The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery. | [required] |
**apps_update_webhook_config_for_app_request** | Option<[**AppsUpdateWebhookConfigForAppRequest**](AppsUpdateWebhookConfigForAppRequest.md)> |  |  |

### Return type

[**models::WebhookConfig**](webhook-config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

