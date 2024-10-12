# \InteractionsApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**interactions_slash_get_restrictions_for_authenticated_user**](InteractionsApi.md#interactions_slash_get_restrictions_for_authenticated_user) | **GET** /user/interaction-limits | Get interaction restrictions for your public repositories
[**interactions_slash_get_restrictions_for_org**](InteractionsApi.md#interactions_slash_get_restrictions_for_org) | **GET** /orgs/{org}/interaction-limits | Get interaction restrictions for an organization
[**interactions_slash_get_restrictions_for_repo**](InteractionsApi.md#interactions_slash_get_restrictions_for_repo) | **GET** /repos/{owner}/{repo}/interaction-limits | Get interaction restrictions for a repository
[**interactions_slash_remove_restrictions_for_authenticated_user**](InteractionsApi.md#interactions_slash_remove_restrictions_for_authenticated_user) | **DELETE** /user/interaction-limits | Remove interaction restrictions from your public repositories
[**interactions_slash_remove_restrictions_for_org**](InteractionsApi.md#interactions_slash_remove_restrictions_for_org) | **DELETE** /orgs/{org}/interaction-limits | Remove interaction restrictions for an organization
[**interactions_slash_remove_restrictions_for_repo**](InteractionsApi.md#interactions_slash_remove_restrictions_for_repo) | **DELETE** /repos/{owner}/{repo}/interaction-limits | Remove interaction restrictions for a repository
[**interactions_slash_set_restrictions_for_authenticated_user**](InteractionsApi.md#interactions_slash_set_restrictions_for_authenticated_user) | **PUT** /user/interaction-limits | Set interaction restrictions for your public repositories
[**interactions_slash_set_restrictions_for_org**](InteractionsApi.md#interactions_slash_set_restrictions_for_org) | **PUT** /orgs/{org}/interaction-limits | Set interaction restrictions for an organization
[**interactions_slash_set_restrictions_for_repo**](InteractionsApi.md#interactions_slash_set_restrictions_for_repo) | **PUT** /repos/{owner}/{repo}/interaction-limits | Set interaction restrictions for a repository



## interactions_slash_get_restrictions_for_authenticated_user

> models::InteractionsGetRestrictionsForOrg200Response interactions_slash_get_restrictions_for_authenticated_user()
Get interaction restrictions for your public repositories

Shows which type of GitHub user can interact with your public repositories and when the restriction expires.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::InteractionsGetRestrictionsForOrg200Response**](interactions_get_restrictions_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## interactions_slash_get_restrictions_for_org

> models::InteractionsGetRestrictionsForOrg200Response interactions_slash_get_restrictions_for_org(org)
Get interaction restrictions for an organization

Shows which type of GitHub user can interact with this organization and when the restriction expires. If there is no restrictions, you will see an empty response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::InteractionsGetRestrictionsForOrg200Response**](interactions_get_restrictions_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## interactions_slash_get_restrictions_for_repo

> models::InteractionsGetRestrictionsForOrg200Response interactions_slash_get_restrictions_for_repo(owner, repo)
Get interaction restrictions for a repository

Shows which type of GitHub user can interact with this repository and when the restriction expires. If there are no restrictions, you will see an empty response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::InteractionsGetRestrictionsForOrg200Response**](interactions_get_restrictions_for_org_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## interactions_slash_remove_restrictions_for_authenticated_user

> interactions_slash_remove_restrictions_for_authenticated_user()
Remove interaction restrictions from your public repositories

Removes any interaction restrictions from your public repositories.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## interactions_slash_remove_restrictions_for_org

> interactions_slash_remove_restrictions_for_org(org)
Remove interaction restrictions for an organization

Removes all interaction restrictions from public repositories in the given organization. You must be an organization owner to remove restrictions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## interactions_slash_remove_restrictions_for_repo

> interactions_slash_remove_restrictions_for_repo(owner, repo)
Remove interaction restrictions for a repository

Removes all interaction restrictions from the given repository. You must have owner or admin access to remove restrictions. If the interaction limit is set for the user or organization that owns this repository, you will receive a `409 Conflict` response and will not be able to use this endpoint to change the interaction limit for a single repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## interactions_slash_set_restrictions_for_authenticated_user

> models::InteractionLimitResponse interactions_slash_set_restrictions_for_authenticated_user(interaction_limit)
Set interaction restrictions for your public repositories

Temporarily restricts which type of GitHub user can interact with your public repositories. Setting the interaction limit at the user level will overwrite any interaction limits that are set for individual repositories owned by the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**interaction_limit** | [**InteractionLimit**](InteractionLimit.md) |  | [required] |

### Return type

[**models::InteractionLimitResponse**](interaction-limit-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## interactions_slash_set_restrictions_for_org

> models::InteractionLimitResponse interactions_slash_set_restrictions_for_org(org, interaction_limit)
Set interaction restrictions for an organization

Temporarily restricts interactions to a certain type of GitHub user in any public repository in the given organization. You must be an organization owner to set these restrictions. Setting the interaction limit at the organization level will overwrite any interaction limits that are set for individual repositories owned by the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**interaction_limit** | [**InteractionLimit**](InteractionLimit.md) |  | [required] |

### Return type

[**models::InteractionLimitResponse**](interaction-limit-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## interactions_slash_set_restrictions_for_repo

> models::InteractionLimitResponse interactions_slash_set_restrictions_for_repo(owner, repo, interaction_limit)
Set interaction restrictions for a repository

Temporarily restricts interactions to a certain type of GitHub user within the given repository. You must have owner or admin access to set these restrictions. If an interaction limit is set for the user or organization that owns this repository, you will receive a `409 Conflict` response and will not be able to use this endpoint to change the interaction limit for a single repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**interaction_limit** | [**InteractionLimit**](InteractionLimit.md) |  | [required] |

### Return type

[**models::InteractionLimitResponse**](interaction-limit-response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

