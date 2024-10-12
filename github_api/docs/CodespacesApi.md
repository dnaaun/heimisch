# \CodespacesApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**codespaces_slash_add_repository_for_secret_for_authenticated_user**](CodespacesApi.md#codespaces_slash_add_repository_for_secret_for_authenticated_user) | **PUT** /user/codespaces/secrets/{secret_name}/repositories/{repository_id} | Add a selected repository to a user secret
[**codespaces_slash_add_selected_repo_to_org_secret**](CodespacesApi.md#codespaces_slash_add_selected_repo_to_org_secret) | **PUT** /orgs/{org}/codespaces/secrets/{secret_name}/repositories/{repository_id} | Add selected repository to an organization secret
[**codespaces_slash_check_permissions_for_devcontainer**](CodespacesApi.md#codespaces_slash_check_permissions_for_devcontainer) | **GET** /repos/{owner}/{repo}/codespaces/permissions_check | Check if permissions defined by a devcontainer have been accepted by the authenticated user
[**codespaces_slash_codespace_machines_for_authenticated_user**](CodespacesApi.md#codespaces_slash_codespace_machines_for_authenticated_user) | **GET** /user/codespaces/{codespace_name}/machines | List machine types for a codespace
[**codespaces_slash_create_for_authenticated_user**](CodespacesApi.md#codespaces_slash_create_for_authenticated_user) | **POST** /user/codespaces | Create a codespace for the authenticated user
[**codespaces_slash_create_or_update_org_secret**](CodespacesApi.md#codespaces_slash_create_or_update_org_secret) | **PUT** /orgs/{org}/codespaces/secrets/{secret_name} | Create or update an organization secret
[**codespaces_slash_create_or_update_repo_secret**](CodespacesApi.md#codespaces_slash_create_or_update_repo_secret) | **PUT** /repos/{owner}/{repo}/codespaces/secrets/{secret_name} | Create or update a repository secret
[**codespaces_slash_create_or_update_secret_for_authenticated_user**](CodespacesApi.md#codespaces_slash_create_or_update_secret_for_authenticated_user) | **PUT** /user/codespaces/secrets/{secret_name} | Create or update a secret for the authenticated user
[**codespaces_slash_create_with_pr_for_authenticated_user**](CodespacesApi.md#codespaces_slash_create_with_pr_for_authenticated_user) | **POST** /repos/{owner}/{repo}/pulls/{pull_number}/codespaces | Create a codespace from a pull request
[**codespaces_slash_create_with_repo_for_authenticated_user**](CodespacesApi.md#codespaces_slash_create_with_repo_for_authenticated_user) | **POST** /repos/{owner}/{repo}/codespaces | Create a codespace in a repository
[**codespaces_slash_delete_codespaces_access_users**](CodespacesApi.md#codespaces_slash_delete_codespaces_access_users) | **DELETE** /orgs/{org}/codespaces/access/selected_users | Remove users from Codespaces access for an organization
[**codespaces_slash_delete_for_authenticated_user**](CodespacesApi.md#codespaces_slash_delete_for_authenticated_user) | **DELETE** /user/codespaces/{codespace_name} | Delete a codespace for the authenticated user
[**codespaces_slash_delete_from_organization**](CodespacesApi.md#codespaces_slash_delete_from_organization) | **DELETE** /orgs/{org}/members/{username}/codespaces/{codespace_name} | Delete a codespace from the organization
[**codespaces_slash_delete_org_secret**](CodespacesApi.md#codespaces_slash_delete_org_secret) | **DELETE** /orgs/{org}/codespaces/secrets/{secret_name} | Delete an organization secret
[**codespaces_slash_delete_repo_secret**](CodespacesApi.md#codespaces_slash_delete_repo_secret) | **DELETE** /repos/{owner}/{repo}/codespaces/secrets/{secret_name} | Delete a repository secret
[**codespaces_slash_delete_secret_for_authenticated_user**](CodespacesApi.md#codespaces_slash_delete_secret_for_authenticated_user) | **DELETE** /user/codespaces/secrets/{secret_name} | Delete a secret for the authenticated user
[**codespaces_slash_export_for_authenticated_user**](CodespacesApi.md#codespaces_slash_export_for_authenticated_user) | **POST** /user/codespaces/{codespace_name}/exports | Export a codespace for the authenticated user
[**codespaces_slash_get_codespaces_for_user_in_org**](CodespacesApi.md#codespaces_slash_get_codespaces_for_user_in_org) | **GET** /orgs/{org}/members/{username}/codespaces | List codespaces for a user in organization
[**codespaces_slash_get_export_details_for_authenticated_user**](CodespacesApi.md#codespaces_slash_get_export_details_for_authenticated_user) | **GET** /user/codespaces/{codespace_name}/exports/{export_id} | Get details about a codespace export
[**codespaces_slash_get_for_authenticated_user**](CodespacesApi.md#codespaces_slash_get_for_authenticated_user) | **GET** /user/codespaces/{codespace_name} | Get a codespace for the authenticated user
[**codespaces_slash_get_org_public_key**](CodespacesApi.md#codespaces_slash_get_org_public_key) | **GET** /orgs/{org}/codespaces/secrets/public-key | Get an organization public key
[**codespaces_slash_get_org_secret**](CodespacesApi.md#codespaces_slash_get_org_secret) | **GET** /orgs/{org}/codespaces/secrets/{secret_name} | Get an organization secret
[**codespaces_slash_get_public_key_for_authenticated_user**](CodespacesApi.md#codespaces_slash_get_public_key_for_authenticated_user) | **GET** /user/codespaces/secrets/public-key | Get public key for the authenticated user
[**codespaces_slash_get_repo_public_key**](CodespacesApi.md#codespaces_slash_get_repo_public_key) | **GET** /repos/{owner}/{repo}/codespaces/secrets/public-key | Get a repository public key
[**codespaces_slash_get_repo_secret**](CodespacesApi.md#codespaces_slash_get_repo_secret) | **GET** /repos/{owner}/{repo}/codespaces/secrets/{secret_name} | Get a repository secret
[**codespaces_slash_get_secret_for_authenticated_user**](CodespacesApi.md#codespaces_slash_get_secret_for_authenticated_user) | **GET** /user/codespaces/secrets/{secret_name} | Get a secret for the authenticated user
[**codespaces_slash_list_devcontainers_in_repository_for_authenticated_user**](CodespacesApi.md#codespaces_slash_list_devcontainers_in_repository_for_authenticated_user) | **GET** /repos/{owner}/{repo}/codespaces/devcontainers | List devcontainer configurations in a repository for the authenticated user
[**codespaces_slash_list_for_authenticated_user**](CodespacesApi.md#codespaces_slash_list_for_authenticated_user) | **GET** /user/codespaces | List codespaces for the authenticated user
[**codespaces_slash_list_in_organization**](CodespacesApi.md#codespaces_slash_list_in_organization) | **GET** /orgs/{org}/codespaces | List codespaces for the organization
[**codespaces_slash_list_in_repository_for_authenticated_user**](CodespacesApi.md#codespaces_slash_list_in_repository_for_authenticated_user) | **GET** /repos/{owner}/{repo}/codespaces | List codespaces in a repository for the authenticated user
[**codespaces_slash_list_org_secrets**](CodespacesApi.md#codespaces_slash_list_org_secrets) | **GET** /orgs/{org}/codespaces/secrets | List organization secrets
[**codespaces_slash_list_repo_secrets**](CodespacesApi.md#codespaces_slash_list_repo_secrets) | **GET** /repos/{owner}/{repo}/codespaces/secrets | List repository secrets
[**codespaces_slash_list_repositories_for_secret_for_authenticated_user**](CodespacesApi.md#codespaces_slash_list_repositories_for_secret_for_authenticated_user) | **GET** /user/codespaces/secrets/{secret_name}/repositories | List selected repositories for a user secret
[**codespaces_slash_list_secrets_for_authenticated_user**](CodespacesApi.md#codespaces_slash_list_secrets_for_authenticated_user) | **GET** /user/codespaces/secrets | List secrets for the authenticated user
[**codespaces_slash_list_selected_repos_for_org_secret**](CodespacesApi.md#codespaces_slash_list_selected_repos_for_org_secret) | **GET** /orgs/{org}/codespaces/secrets/{secret_name}/repositories | List selected repositories for an organization secret
[**codespaces_slash_pre_flight_with_repo_for_authenticated_user**](CodespacesApi.md#codespaces_slash_pre_flight_with_repo_for_authenticated_user) | **GET** /repos/{owner}/{repo}/codespaces/new | Get default attributes for a codespace
[**codespaces_slash_publish_for_authenticated_user**](CodespacesApi.md#codespaces_slash_publish_for_authenticated_user) | **POST** /user/codespaces/{codespace_name}/publish | Create a repository from an unpublished codespace
[**codespaces_slash_remove_repository_for_secret_for_authenticated_user**](CodespacesApi.md#codespaces_slash_remove_repository_for_secret_for_authenticated_user) | **DELETE** /user/codespaces/secrets/{secret_name}/repositories/{repository_id} | Remove a selected repository from a user secret
[**codespaces_slash_remove_selected_repo_from_org_secret**](CodespacesApi.md#codespaces_slash_remove_selected_repo_from_org_secret) | **DELETE** /orgs/{org}/codespaces/secrets/{secret_name}/repositories/{repository_id} | Remove selected repository from an organization secret
[**codespaces_slash_repo_machines_for_authenticated_user**](CodespacesApi.md#codespaces_slash_repo_machines_for_authenticated_user) | **GET** /repos/{owner}/{repo}/codespaces/machines | List available machine types for a repository
[**codespaces_slash_set_codespaces_access**](CodespacesApi.md#codespaces_slash_set_codespaces_access) | **PUT** /orgs/{org}/codespaces/access | Manage access control for organization codespaces
[**codespaces_slash_set_codespaces_access_users**](CodespacesApi.md#codespaces_slash_set_codespaces_access_users) | **POST** /orgs/{org}/codespaces/access/selected_users | Add users to Codespaces access for an organization
[**codespaces_slash_set_repositories_for_secret_for_authenticated_user**](CodespacesApi.md#codespaces_slash_set_repositories_for_secret_for_authenticated_user) | **PUT** /user/codespaces/secrets/{secret_name}/repositories | Set selected repositories for a user secret
[**codespaces_slash_set_selected_repos_for_org_secret**](CodespacesApi.md#codespaces_slash_set_selected_repos_for_org_secret) | **PUT** /orgs/{org}/codespaces/secrets/{secret_name}/repositories | Set selected repositories for an organization secret
[**codespaces_slash_start_for_authenticated_user**](CodespacesApi.md#codespaces_slash_start_for_authenticated_user) | **POST** /user/codespaces/{codespace_name}/start | Start a codespace for the authenticated user
[**codespaces_slash_stop_for_authenticated_user**](CodespacesApi.md#codespaces_slash_stop_for_authenticated_user) | **POST** /user/codespaces/{codespace_name}/stop | Stop a codespace for the authenticated user
[**codespaces_slash_stop_in_organization**](CodespacesApi.md#codespaces_slash_stop_in_organization) | **POST** /orgs/{org}/members/{username}/codespaces/{codespace_name}/stop | Stop a codespace for an organization user
[**codespaces_slash_update_for_authenticated_user**](CodespacesApi.md#codespaces_slash_update_for_authenticated_user) | **PATCH** /user/codespaces/{codespace_name} | Update a codespace for the authenticated user



## codespaces_slash_add_repository_for_secret_for_authenticated_user

> codespaces_slash_add_repository_for_secret_for_authenticated_user(secret_name, repository_id)
Add a selected repository to a user secret

Adds a repository to the selected repositories for a user's development environment secret.  The authenticated user must have Codespaces access to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |
**repository_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_add_selected_repo_to_org_secret

> codespaces_slash_add_selected_repo_to_org_secret(org, secret_name, repository_id)
Add selected repository to an organization secret

Adds a repository to an organization development environment secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret). OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**repository_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_check_permissions_for_devcontainer

> models::CodespacesPermissionsCheckForDevcontainer codespaces_slash_check_permissions_for_devcontainer(owner, repo, r#ref, devcontainer_path)
Check if permissions defined by a devcontainer have been accepted by the authenticated user

Checks whether the permissions defined by a given devcontainer configuration have been accepted by the authenticated user.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | **String** | The git reference that points to the location of the devcontainer configuration to use for the permission check. The value of `ref` will typically be a branch name (`heads/BRANCH_NAME`). For more information, see \"[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)\" in the Git documentation. | [required] |
**devcontainer_path** | **String** | Path to the devcontainer.json configuration to use for the permission check. | [required] |

### Return type

[**models::CodespacesPermissionsCheckForDevcontainer**](codespaces-permissions-check-for-devcontainer.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_codespace_machines_for_authenticated_user

> models::CodespacesRepoMachinesForAuthenticatedUser200Response codespaces_slash_codespace_machines_for_authenticated_user(codespace_name)
List machine types for a codespace

List the machine types a codespace can transition to use.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codespace_name** | **String** | The name of the codespace. | [required] |

### Return type

[**models::CodespacesRepoMachinesForAuthenticatedUser200Response**](codespaces_repo_machines_for_authenticated_user_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_create_for_authenticated_user

> models::Codespace codespaces_slash_create_for_authenticated_user(codespaces_create_for_authenticated_user_request)
Create a codespace for the authenticated user

Creates a new codespace, owned by the authenticated user.  This endpoint requires either a `repository_id` OR a `pull_request` but not both.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codespaces_create_for_authenticated_user_request** | [**CodespacesCreateForAuthenticatedUserRequest**](CodespacesCreateForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**models::Codespace**](codespace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_create_or_update_org_secret

> serde_json::Value codespaces_slash_create_or_update_org_secret(org, secret_name, codespaces_create_or_update_org_secret_request)
Create or update an organization secret

Creates or updates an organization development environment secret with an encrypted value. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see \"[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api).\"  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**codespaces_create_or_update_org_secret_request** | [**CodespacesCreateOrUpdateOrgSecretRequest**](CodespacesCreateOrUpdateOrgSecretRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_create_or_update_repo_secret

> serde_json::Value codespaces_slash_create_or_update_repo_secret(owner, repo, secret_name, codespaces_create_or_update_repo_secret_request)
Create or update a repository secret

Creates or updates a repository development environment secret with an encrypted value. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see \"[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api).\"  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**codespaces_create_or_update_repo_secret_request** | [**CodespacesCreateOrUpdateRepoSecretRequest**](CodespacesCreateOrUpdateRepoSecretRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_create_or_update_secret_for_authenticated_user

> serde_json::Value codespaces_slash_create_or_update_secret_for_authenticated_user(secret_name, codespaces_create_or_update_secret_for_authenticated_user_request)
Create or update a secret for the authenticated user

Creates or updates a development environment secret for a user's codespace with an encrypted value. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see \"[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api).\"  The authenticated user must have Codespaces access to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |
**codespaces_create_or_update_secret_for_authenticated_user_request** | [**CodespacesCreateOrUpdateSecretForAuthenticatedUserRequest**](CodespacesCreateOrUpdateSecretForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_create_with_pr_for_authenticated_user

> models::Codespace codespaces_slash_create_with_pr_for_authenticated_user(owner, repo, pull_number, codespaces_create_with_pr_for_authenticated_user_request)
Create a codespace from a pull request

Creates a codespace owned by the authenticated user for the specified pull request.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**pull_number** | **i32** | The number that identifies the pull request. | [required] |
**codespaces_create_with_pr_for_authenticated_user_request** | Option<[**CodespacesCreateWithPrForAuthenticatedUserRequest**](CodespacesCreateWithPrForAuthenticatedUserRequest.md)> |  | [required] |

### Return type

[**models::Codespace**](codespace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_create_with_repo_for_authenticated_user

> models::Codespace codespaces_slash_create_with_repo_for_authenticated_user(owner, repo, codespaces_create_with_repo_for_authenticated_user_request)
Create a codespace in a repository

Creates a codespace owned by the authenticated user in the specified repository.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**codespaces_create_with_repo_for_authenticated_user_request** | Option<[**CodespacesCreateWithRepoForAuthenticatedUserRequest**](CodespacesCreateWithRepoForAuthenticatedUserRequest.md)> |  | [required] |

### Return type

[**models::Codespace**](codespace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_delete_codespaces_access_users

> codespaces_slash_delete_codespaces_access_users(org, codespaces_delete_codespaces_access_users_request)
Remove users from Codespaces access for an organization

Codespaces for the specified users will no longer be billed to the organization.  To use this endpoint, the access settings for the organization must be set to `selected_members`. For information on how to change this setting, see \"[Manage access control for organization codespaces](https://docs.github.com/rest/codespaces/organizations#manage-access-control-for-organization-codespaces).\"  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**codespaces_delete_codespaces_access_users_request** | [**CodespacesDeleteCodespacesAccessUsersRequest**](CodespacesDeleteCodespacesAccessUsersRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_delete_for_authenticated_user

> serde_json::Value codespaces_slash_delete_for_authenticated_user(codespace_name)
Delete a codespace for the authenticated user

Deletes a user's codespace.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codespace_name** | **String** | The name of the codespace. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_delete_from_organization

> serde_json::Value codespaces_slash_delete_from_organization(org, username, codespace_name)
Delete a codespace from the organization

Deletes a user's codespace.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**codespace_name** | **String** | The name of the codespace. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_delete_org_secret

> codespaces_slash_delete_org_secret(org, secret_name)
Delete an organization secret

Deletes an organization development environment secret using the secret name.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_delete_repo_secret

> codespaces_slash_delete_repo_secret(owner, repo, secret_name)
Delete a repository secret

Deletes a development environment secret in a repository using the secret name.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_delete_secret_for_authenticated_user

> codespaces_slash_delete_secret_for_authenticated_user(secret_name)
Delete a secret for the authenticated user

Deletes a development environment secret from a user's codespaces using the secret name. Deleting the secret will remove access from all codespaces that were allowed to access the secret.  The authenticated user must have Codespaces access to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_export_for_authenticated_user

> models::CodespaceExportDetails codespaces_slash_export_for_authenticated_user(codespace_name)
Export a codespace for the authenticated user

Triggers an export of the specified codespace and returns a URL and ID where the status of the export can be monitored.  If changes cannot be pushed to the codespace's repository, they will be pushed to a new or previously-existing fork instead.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codespace_name** | **String** | The name of the codespace. | [required] |

### Return type

[**models::CodespaceExportDetails**](codespace-export-details.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_get_codespaces_for_user_in_org

> models::CodespacesListInOrganization200Response codespaces_slash_get_codespaces_for_user_in_org(org, username, per_page, page)
List codespaces for a user in organization

Lists the codespaces that a member of an organization has for repositories in that organization.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::CodespacesListInOrganization200Response**](codespaces_list_in_organization_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_get_export_details_for_authenticated_user

> models::CodespaceExportDetails codespaces_slash_get_export_details_for_authenticated_user(codespace_name, export_id)
Get details about a codespace export

Gets information about an export of a codespace.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codespace_name** | **String** | The name of the codespace. | [required] |
**export_id** | **String** | The ID of the export operation, or `latest`. Currently only `latest` is currently supported. | [required] |

### Return type

[**models::CodespaceExportDetails**](codespace-export-details.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_get_for_authenticated_user

> models::Codespace codespaces_slash_get_for_authenticated_user(codespace_name)
Get a codespace for the authenticated user

Gets information about a user's codespace.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codespace_name** | **String** | The name of the codespace. | [required] |

### Return type

[**models::Codespace**](codespace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_get_org_public_key

> models::CodespacesPublicKey codespaces_slash_get_org_public_key(org)
Get an organization public key

Gets a public key for an organization, which is required in order to encrypt secrets. You need to encrypt the value of a secret before you can create or update secrets. OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::CodespacesPublicKey**](codespaces-public-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_get_org_secret

> models::CodespacesOrgSecret codespaces_slash_get_org_secret(org, secret_name)
Get an organization secret

Gets an organization development environment secret without revealing its encrypted value.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

[**models::CodespacesOrgSecret**](codespaces-org-secret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_get_public_key_for_authenticated_user

> models::CodespacesUserPublicKey codespaces_slash_get_public_key_for_authenticated_user()
Get public key for the authenticated user

Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets.  The authenticated user must have Codespaces access to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CodespacesUserPublicKey**](codespaces-user-public-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_get_repo_public_key

> models::CodespacesPublicKey codespaces_slash_get_repo_public_key(owner, repo)
Get a repository public key

Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets.  If the repository is private, OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::CodespacesPublicKey**](codespaces-public-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_get_repo_secret

> models::RepoCodespacesSecret codespaces_slash_get_repo_secret(owner, repo, secret_name)
Get a repository secret

Gets a single repository development environment secret without revealing its encrypted value.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

[**models::RepoCodespacesSecret**](repo-codespaces-secret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_get_secret_for_authenticated_user

> models::CodespacesSecret codespaces_slash_get_secret_for_authenticated_user(secret_name)
Get a secret for the authenticated user

Gets a development environment secret available to a user's codespaces without revealing its encrypted value.  The authenticated user must have Codespaces access to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

[**models::CodespacesSecret**](codespaces-secret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_list_devcontainers_in_repository_for_authenticated_user

> models::CodespacesListDevcontainersInRepositoryForAuthenticatedUser200Response codespaces_slash_list_devcontainers_in_repository_for_authenticated_user(owner, repo, per_page, page)
List devcontainer configurations in a repository for the authenticated user

Lists the devcontainer.json files associated with a specified repository and the authenticated user. These files specify launchpoint configurations for codespaces created within the repository.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::CodespacesListDevcontainersInRepositoryForAuthenticatedUser200Response**](codespaces_list_devcontainers_in_repository_for_authenticated_user_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_list_for_authenticated_user

> models::CodespacesListInOrganization200Response codespaces_slash_list_for_authenticated_user(per_page, page, repository_id)
List codespaces for the authenticated user

Lists the authenticated user's codespaces.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**repository_id** | Option<**i32**> | ID of the Repository to filter on |  |

### Return type

[**models::CodespacesListInOrganization200Response**](codespaces_list_in_organization_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_list_in_organization

> models::CodespacesListInOrganization200Response codespaces_slash_list_in_organization(org, per_page, page)
List codespaces for the organization

Lists the codespaces associated to a specified organization.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::CodespacesListInOrganization200Response**](codespaces_list_in_organization_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_list_in_repository_for_authenticated_user

> models::CodespacesListInOrganization200Response codespaces_slash_list_in_repository_for_authenticated_user(owner, repo, per_page, page)
List codespaces in a repository for the authenticated user

Lists the codespaces associated to a specified repository and the authenticated user.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::CodespacesListInOrganization200Response**](codespaces_list_in_organization_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_list_org_secrets

> models::CodespacesListOrgSecrets200Response codespaces_slash_list_org_secrets(org, per_page, page)
List organization secrets

Lists all Codespaces development environment secrets available at the organization-level without revealing their encrypted values.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::CodespacesListOrgSecrets200Response**](codespaces_list_org_secrets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_list_repo_secrets

> models::CodespacesListRepoSecrets200Response codespaces_slash_list_repo_secrets(owner, repo, per_page, page)
List repository secrets

Lists all development environment secrets available in a repository without revealing their encrypted values.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::CodespacesListRepoSecrets200Response**](codespaces_list_repo_secrets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_list_repositories_for_secret_for_authenticated_user

> models::ActionsListSelectedReposForOrgSecret200Response codespaces_slash_list_repositories_for_secret_for_authenticated_user(secret_name)
List selected repositories for a user secret

List the repositories that have been granted the ability to use a user's development environment secret.  The authenticated user must have Codespaces access to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

[**models::ActionsListSelectedReposForOrgSecret200Response**](actions_list_selected_repos_for_org_secret_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_list_secrets_for_authenticated_user

> models::CodespacesListSecretsForAuthenticatedUser200Response codespaces_slash_list_secrets_for_authenticated_user(per_page, page)
List secrets for the authenticated user

Lists all development environment secrets available for a user's codespaces without revealing their encrypted values.  The authenticated user must have Codespaces access to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::CodespacesListSecretsForAuthenticatedUser200Response**](codespaces_list_secrets_for_authenticated_user_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_list_selected_repos_for_org_secret

> models::ActionsListSelectedReposForOrgSecret200Response codespaces_slash_list_selected_repos_for_org_secret(org, secret_name, page, per_page)
List selected repositories for an organization secret

Lists all repositories that have been selected when the `visibility` for repository access to a secret is set to `selected`.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**models::ActionsListSelectedReposForOrgSecret200Response**](actions_list_selected_repos_for_org_secret_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_pre_flight_with_repo_for_authenticated_user

> models::CodespacesPreFlightWithRepoForAuthenticatedUser200Response codespaces_slash_pre_flight_with_repo_for_authenticated_user(owner, repo, r#ref, client_ip)
Get default attributes for a codespace

Gets the default attributes for codespaces created by the user with the repository.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | Option<**String**> | The branch or commit to check for a default devcontainer path. If not specified, the default branch will be checked. |  |
**client_ip** | Option<**String**> | An alternative IP for default location auto-detection, such as when proxying a request. |  |

### Return type

[**models::CodespacesPreFlightWithRepoForAuthenticatedUser200Response**](codespaces_pre_flight_with_repo_for_authenticated_user_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_publish_for_authenticated_user

> models::CodespaceWithFullRepository codespaces_slash_publish_for_authenticated_user(codespace_name, codespaces_publish_for_authenticated_user_request)
Create a repository from an unpublished codespace

Publishes an unpublished codespace, creating a new repository and assigning it to the codespace.  The codespace's token is granted write permissions to the repository, allowing the user to push their changes.  This will fail for a codespace that is already published, meaning it has an associated repository.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codespace_name** | **String** | The name of the codespace. | [required] |
**codespaces_publish_for_authenticated_user_request** | [**CodespacesPublishForAuthenticatedUserRequest**](CodespacesPublishForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**models::CodespaceWithFullRepository**](codespace-with-full-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_remove_repository_for_secret_for_authenticated_user

> codespaces_slash_remove_repository_for_secret_for_authenticated_user(secret_name, repository_id)
Remove a selected repository from a user secret

Removes a repository from the selected repositories for a user's development environment secret.  The authenticated user must have Codespaces access to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |
**repository_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_remove_selected_repo_from_org_secret

> codespaces_slash_remove_selected_repo_from_org_secret(org, secret_name, repository_id)
Remove selected repository from an organization secret

Removes a repository from an organization development environment secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret).  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**repository_id** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_repo_machines_for_authenticated_user

> models::CodespacesRepoMachinesForAuthenticatedUser200Response codespaces_slash_repo_machines_for_authenticated_user(owner, repo, location, client_ip, r#ref)
List available machine types for a repository

List the machine types available for a given repository based on its configuration.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**location** | Option<**String**> | The location to check for available machines. Assigned by IP if not provided. |  |
**client_ip** | Option<**String**> | IP for location auto-detection when proxying a request |  |
**r#ref** | Option<**String**> | The branch or commit to check for prebuild availability and devcontainer restrictions. |  |

### Return type

[**models::CodespacesRepoMachinesForAuthenticatedUser200Response**](codespaces_repo_machines_for_authenticated_user_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_set_codespaces_access

> codespaces_slash_set_codespaces_access(org, codespaces_set_codespaces_access_request)
Manage access control for organization codespaces

Sets which users can access codespaces in an organization. This is synonymous with granting or revoking codespaces access permissions for users according to the visibility. OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**codespaces_set_codespaces_access_request** | [**CodespacesSetCodespacesAccessRequest**](CodespacesSetCodespacesAccessRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_set_codespaces_access_users

> codespaces_slash_set_codespaces_access_users(org, codespaces_set_codespaces_access_users_request)
Add users to Codespaces access for an organization

Codespaces for the specified users will be billed to the organization.  To use this endpoint, the access settings for the organization must be set to `selected_members`. For information on how to change this setting, see \"[Manage access control for organization codespaces](https://docs.github.com/rest/codespaces/organizations#manage-access-control-for-organization-codespaces).\"  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**codespaces_set_codespaces_access_users_request** | [**CodespacesSetCodespacesAccessUsersRequest**](CodespacesSetCodespacesAccessUsersRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_set_repositories_for_secret_for_authenticated_user

> codespaces_slash_set_repositories_for_secret_for_authenticated_user(secret_name, codespaces_set_repositories_for_secret_for_authenticated_user_request)
Set selected repositories for a user secret

Select the repositories that will use a user's development environment secret.  The authenticated user must have Codespaces access to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** | The name of the secret. | [required] |
**codespaces_set_repositories_for_secret_for_authenticated_user_request** | [**CodespacesSetRepositoriesForSecretForAuthenticatedUserRequest**](CodespacesSetRepositoriesForSecretForAuthenticatedUserRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_set_selected_repos_for_org_secret

> codespaces_slash_set_selected_repos_for_org_secret(org, secret_name, codespaces_set_selected_repos_for_org_secret_request)
Set selected repositories for an organization secret

Replaces all repositories for an organization development environment secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret).  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**codespaces_set_selected_repos_for_org_secret_request** | [**CodespacesSetSelectedReposForOrgSecretRequest**](CodespacesSetSelectedReposForOrgSecretRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_start_for_authenticated_user

> models::Codespace codespaces_slash_start_for_authenticated_user(codespace_name)
Start a codespace for the authenticated user

Starts a user's codespace.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codespace_name** | **String** | The name of the codespace. | [required] |

### Return type

[**models::Codespace**](codespace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_stop_for_authenticated_user

> models::Codespace codespaces_slash_stop_for_authenticated_user(codespace_name)
Stop a codespace for the authenticated user

Stops a user's codespace.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codespace_name** | **String** | The name of the codespace. | [required] |

### Return type

[**models::Codespace**](codespace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_stop_in_organization

> models::Codespace codespaces_slash_stop_in_organization(org, username, codespace_name)
Stop a codespace for an organization user

Stops a user's codespace.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**codespace_name** | **String** | The name of the codespace. | [required] |

### Return type

[**models::Codespace**](codespace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codespaces_slash_update_for_authenticated_user

> models::Codespace codespaces_slash_update_for_authenticated_user(codespace_name, codespaces_update_for_authenticated_user_request)
Update a codespace for the authenticated user

Updates a codespace owned by the authenticated user. Currently only the codespace's machine type and recent folders can be modified using this endpoint.  If you specify a new machine type it will be applied the next time your codespace is started.  OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codespace_name** | **String** | The name of the codespace. | [required] |
**codespaces_update_for_authenticated_user_request** | Option<[**CodespacesUpdateForAuthenticatedUserRequest**](CodespacesUpdateForAuthenticatedUserRequest.md)> |  |  |

### Return type

[**models::Codespace**](codespace.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

