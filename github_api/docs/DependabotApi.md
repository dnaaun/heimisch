# \DependabotApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dependabot_slash_add_selected_repo_to_org_secret**](DependabotApi.md#dependabot_slash_add_selected_repo_to_org_secret) | **PUT** /orgs/{org}/dependabot/secrets/{secret_name}/repositories/{repository_id} | Add selected repository to an organization secret
[**dependabot_slash_create_or_update_org_secret**](DependabotApi.md#dependabot_slash_create_or_update_org_secret) | **PUT** /orgs/{org}/dependabot/secrets/{secret_name} | Create or update an organization secret
[**dependabot_slash_create_or_update_repo_secret**](DependabotApi.md#dependabot_slash_create_or_update_repo_secret) | **PUT** /repos/{owner}/{repo}/dependabot/secrets/{secret_name} | Create or update a repository secret
[**dependabot_slash_delete_org_secret**](DependabotApi.md#dependabot_slash_delete_org_secret) | **DELETE** /orgs/{org}/dependabot/secrets/{secret_name} | Delete an organization secret
[**dependabot_slash_delete_repo_secret**](DependabotApi.md#dependabot_slash_delete_repo_secret) | **DELETE** /repos/{owner}/{repo}/dependabot/secrets/{secret_name} | Delete a repository secret
[**dependabot_slash_get_alert**](DependabotApi.md#dependabot_slash_get_alert) | **GET** /repos/{owner}/{repo}/dependabot/alerts/{alert_number} | Get a Dependabot alert
[**dependabot_slash_get_org_public_key**](DependabotApi.md#dependabot_slash_get_org_public_key) | **GET** /orgs/{org}/dependabot/secrets/public-key | Get an organization public key
[**dependabot_slash_get_org_secret**](DependabotApi.md#dependabot_slash_get_org_secret) | **GET** /orgs/{org}/dependabot/secrets/{secret_name} | Get an organization secret
[**dependabot_slash_get_repo_public_key**](DependabotApi.md#dependabot_slash_get_repo_public_key) | **GET** /repos/{owner}/{repo}/dependabot/secrets/public-key | Get a repository public key
[**dependabot_slash_get_repo_secret**](DependabotApi.md#dependabot_slash_get_repo_secret) | **GET** /repos/{owner}/{repo}/dependabot/secrets/{secret_name} | Get a repository secret
[**dependabot_slash_list_alerts_for_enterprise**](DependabotApi.md#dependabot_slash_list_alerts_for_enterprise) | **GET** /enterprises/{enterprise}/dependabot/alerts | List Dependabot alerts for an enterprise
[**dependabot_slash_list_alerts_for_org**](DependabotApi.md#dependabot_slash_list_alerts_for_org) | **GET** /orgs/{org}/dependabot/alerts | List Dependabot alerts for an organization
[**dependabot_slash_list_alerts_for_repo**](DependabotApi.md#dependabot_slash_list_alerts_for_repo) | **GET** /repos/{owner}/{repo}/dependabot/alerts | List Dependabot alerts for a repository
[**dependabot_slash_list_org_secrets**](DependabotApi.md#dependabot_slash_list_org_secrets) | **GET** /orgs/{org}/dependabot/secrets | List organization secrets
[**dependabot_slash_list_repo_secrets**](DependabotApi.md#dependabot_slash_list_repo_secrets) | **GET** /repos/{owner}/{repo}/dependabot/secrets | List repository secrets
[**dependabot_slash_list_selected_repos_for_org_secret**](DependabotApi.md#dependabot_slash_list_selected_repos_for_org_secret) | **GET** /orgs/{org}/dependabot/secrets/{secret_name}/repositories | List selected repositories for an organization secret
[**dependabot_slash_remove_selected_repo_from_org_secret**](DependabotApi.md#dependabot_slash_remove_selected_repo_from_org_secret) | **DELETE** /orgs/{org}/dependabot/secrets/{secret_name}/repositories/{repository_id} | Remove selected repository from an organization secret
[**dependabot_slash_set_selected_repos_for_org_secret**](DependabotApi.md#dependabot_slash_set_selected_repos_for_org_secret) | **PUT** /orgs/{org}/dependabot/secrets/{secret_name}/repositories | Set selected repositories for an organization secret
[**dependabot_slash_update_alert**](DependabotApi.md#dependabot_slash_update_alert) | **PATCH** /repos/{owner}/{repo}/dependabot/alerts/{alert_number} | Update a Dependabot alert



## dependabot_slash_add_selected_repo_to_org_secret

> dependabot_slash_add_selected_repo_to_org_secret(org, secret_name, repository_id)
Add selected repository to an organization secret

Adds a repository to an organization secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret).  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_create_or_update_org_secret

> serde_json::Value dependabot_slash_create_or_update_org_secret(org, secret_name, dependabot_create_or_update_org_secret_request)
Create or update an organization secret

Creates or updates an organization secret with an encrypted value. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see \"[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api).\"  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**dependabot_create_or_update_org_secret_request** | [**DependabotCreateOrUpdateOrgSecretRequest**](DependabotCreateOrUpdateOrgSecretRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_create_or_update_repo_secret

> serde_json::Value dependabot_slash_create_or_update_repo_secret(owner, repo, secret_name, dependabot_create_or_update_repo_secret_request)
Create or update a repository secret

Creates or updates a repository secret with an encrypted value. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see \"[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api).\"  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**dependabot_create_or_update_repo_secret_request** | [**DependabotCreateOrUpdateRepoSecretRequest**](DependabotCreateOrUpdateRepoSecretRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_delete_org_secret

> dependabot_slash_delete_org_secret(org, secret_name)
Delete an organization secret

Deletes a secret in an organization using the secret name.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_delete_repo_secret

> dependabot_slash_delete_repo_secret(owner, repo, secret_name)
Delete a repository secret

Deletes a secret in a repository using the secret name.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

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


## dependabot_slash_get_alert

> models::DependabotAlert dependabot_slash_get_alert(owner, repo, alert_number)
Get a Dependabot alert

OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**alert_number** | **i32** | The number that identifies a Dependabot alert in its repository. You can find this at the end of the URL for a Dependabot alert within GitHub, or in `number` fields in the response from the `GET /repos/{owner}/{repo}/dependabot/alerts` operation. | [required] |

### Return type

[**models::DependabotAlert**](dependabot-alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_get_org_public_key

> models::DependabotPublicKey dependabot_slash_get_org_public_key(org)
Get an organization public key

Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::DependabotPublicKey**](dependabot-public-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_get_org_secret

> models::OrganizationDependabotSecret dependabot_slash_get_org_secret(org, secret_name)
Get an organization secret

Gets a single organization secret without revealing its encrypted value.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

[**models::OrganizationDependabotSecret**](organization-dependabot-secret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_get_repo_public_key

> models::DependabotPublicKey dependabot_slash_get_repo_public_key(owner, repo)
Get a repository public key

Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets. Anyone with read access to the repository can use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint if the repository is private.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::DependabotPublicKey**](dependabot-public-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_get_repo_secret

> models::DependabotSecret dependabot_slash_get_repo_secret(owner, repo, secret_name)
Get a repository secret

Gets a single repository secret without revealing its encrypted value.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |

### Return type

[**models::DependabotSecret**](dependabot-secret.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_list_alerts_for_enterprise

> Vec<models::DependabotAlertWithRepository> dependabot_slash_list_alerts_for_enterprise(enterprise, state, severity, ecosystem, package, scope, sort, direction, before, after, first, last, per_page)
List Dependabot alerts for an enterprise

Lists Dependabot alerts for repositories that are owned by the specified enterprise.  The authenticated user must be a member of the enterprise to use this endpoint.  Alerts are only returned for organizations in the enterprise for which you are an organization owner or a security manager. For more information about security managers, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enterprise** | **String** | The slug version of the enterprise name. You can also substitute this value with the enterprise id. | [required] |
**state** | Option<**String**> | A comma-separated list of states. If specified, only alerts with these states will be returned.  Can be: `auto_dismissed`, `dismissed`, `fixed`, `open` |  |
**severity** | Option<**String**> | A comma-separated list of severities. If specified, only alerts with these severities will be returned.  Can be: `low`, `medium`, `high`, `critical` |  |
**ecosystem** | Option<**String**> | A comma-separated list of ecosystems. If specified, only alerts for these ecosystems will be returned.  Can be: `composer`, `go`, `maven`, `npm`, `nuget`, `pip`, `pub`, `rubygems`, `rust` |  |
**package** | Option<**String**> | A comma-separated list of package names. If specified, only alerts for these packages will be returned. |  |
**scope** | Option<**String**> | The scope of the vulnerable dependency. If specified, only alerts with this scope will be returned. |  |
**sort** | Option<**String**> | The property by which to sort the results. `created` means when the alert was created. `updated` means when the alert's state last changed. |  |[default to created]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**first** | Option<**i32**> | **Deprecated**. The number of results per page (max 100), starting from the first matching result. This parameter must not be used in combination with `last`. Instead, use `per_page` in combination with `after` to fetch the first page of results. |  |[default to 30]
**last** | Option<**i32**> | **Deprecated**. The number of results per page (max 100), starting from the last matching result. This parameter must not be used in combination with `first`. Instead, use `per_page` in combination with `before` to fetch the last page of results. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**Vec<models::DependabotAlertWithRepository>**](dependabot-alert-with-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_list_alerts_for_org

> Vec<models::DependabotAlertWithRepository> dependabot_slash_list_alerts_for_org(org, state, severity, ecosystem, package, scope, sort, direction, before, after, first, last, per_page)
List Dependabot alerts for an organization

Lists Dependabot alerts for an organization.  The authenticated user must be an owner or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**state** | Option<**String**> | A comma-separated list of states. If specified, only alerts with these states will be returned.  Can be: `auto_dismissed`, `dismissed`, `fixed`, `open` |  |
**severity** | Option<**String**> | A comma-separated list of severities. If specified, only alerts with these severities will be returned.  Can be: `low`, `medium`, `high`, `critical` |  |
**ecosystem** | Option<**String**> | A comma-separated list of ecosystems. If specified, only alerts for these ecosystems will be returned.  Can be: `composer`, `go`, `maven`, `npm`, `nuget`, `pip`, `pub`, `rubygems`, `rust` |  |
**package** | Option<**String**> | A comma-separated list of package names. If specified, only alerts for these packages will be returned. |  |
**scope** | Option<**String**> | The scope of the vulnerable dependency. If specified, only alerts with this scope will be returned. |  |
**sort** | Option<**String**> | The property by which to sort the results. `created` means when the alert was created. `updated` means when the alert's state last changed. |  |[default to created]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**first** | Option<**i32**> | **Deprecated**. The number of results per page (max 100), starting from the first matching result. This parameter must not be used in combination with `last`. Instead, use `per_page` in combination with `after` to fetch the first page of results. |  |[default to 30]
**last** | Option<**i32**> | **Deprecated**. The number of results per page (max 100), starting from the last matching result. This parameter must not be used in combination with `first`. Instead, use `per_page` in combination with `before` to fetch the last page of results. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**Vec<models::DependabotAlertWithRepository>**](dependabot-alert-with-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_list_alerts_for_repo

> Vec<models::DependabotAlert> dependabot_slash_list_alerts_for_repo(owner, repo, state, severity, ecosystem, package, manifest, scope, sort, direction, page, per_page, before, after, first, last)
List Dependabot alerts for a repository

OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**state** | Option<**String**> | A comma-separated list of states. If specified, only alerts with these states will be returned.  Can be: `auto_dismissed`, `dismissed`, `fixed`, `open` |  |
**severity** | Option<**String**> | A comma-separated list of severities. If specified, only alerts with these severities will be returned.  Can be: `low`, `medium`, `high`, `critical` |  |
**ecosystem** | Option<**String**> | A comma-separated list of ecosystems. If specified, only alerts for these ecosystems will be returned.  Can be: `composer`, `go`, `maven`, `npm`, `nuget`, `pip`, `pub`, `rubygems`, `rust` |  |
**package** | Option<**String**> | A comma-separated list of package names. If specified, only alerts for these packages will be returned. |  |
**manifest** | Option<**String**> | A comma-separated list of full manifest paths. If specified, only alerts for these manifests will be returned. |  |
**scope** | Option<**String**> | The scope of the vulnerable dependency. If specified, only alerts with this scope will be returned. |  |
**sort** | Option<**String**> | The property by which to sort the results. `created` means when the alert was created. `updated` means when the alert's state last changed. |  |[default to created]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**page** | Option<**i32**> | **Closing down notice**. Page number of the results to fetch. Use cursor-based pagination with `before` or `after` instead. |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**first** | Option<**i32**> | **Deprecated**. The number of results per page (max 100), starting from the first matching result. This parameter must not be used in combination with `last`. Instead, use `per_page` in combination with `after` to fetch the first page of results. |  |[default to 30]
**last** | Option<**i32**> | **Deprecated**. The number of results per page (max 100), starting from the last matching result. This parameter must not be used in combination with `first`. Instead, use `per_page` in combination with `before` to fetch the last page of results. |  |

### Return type

[**Vec<models::DependabotAlert>**](dependabot-alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_list_org_secrets

> models::DependabotListOrgSecrets200Response dependabot_slash_list_org_secrets(org, per_page, page)
List organization secrets

Lists all secrets available in an organization without revealing their encrypted values.  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::DependabotListOrgSecrets200Response**](dependabot_list_org_secrets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_list_repo_secrets

> models::DependabotListRepoSecrets200Response dependabot_slash_list_repo_secrets(owner, repo, per_page, page)
List repository secrets

Lists all secrets available in a repository without revealing their encrypted values.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::DependabotListRepoSecrets200Response**](dependabot_list_repo_secrets_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_list_selected_repos_for_org_secret

> models::ActionsListSelectedReposForOrgSecret200Response dependabot_slash_list_selected_repos_for_org_secret(org, secret_name, page, per_page)
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


## dependabot_slash_remove_selected_repo_from_org_secret

> dependabot_slash_remove_selected_repo_from_org_secret(org, secret_name, repository_id)
Remove selected repository from an organization secret

Removes a repository from an organization secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret).  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_set_selected_repos_for_org_secret

> dependabot_slash_set_selected_repos_for_org_secret(org, secret_name, dependabot_set_selected_repos_for_org_secret_request)
Set selected repositories for an organization secret

Replaces all repositories for an organization secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret).  OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**secret_name** | **String** | The name of the secret. | [required] |
**dependabot_set_selected_repos_for_org_secret_request** | [**DependabotSetSelectedReposForOrgSecretRequest**](DependabotSetSelectedReposForOrgSecretRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependabot_slash_update_alert

> models::DependabotAlert dependabot_slash_update_alert(owner, repo, alert_number, dependabot_update_alert_request)
Update a Dependabot alert

The authenticated user must have access to security alerts for the repository to use this endpoint. For more information, see \"[Granting access to security alerts](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-security-and-analysis-settings-for-your-repository#granting-access-to-security-alerts).\"  OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**alert_number** | **i32** | The number that identifies a Dependabot alert in its repository. You can find this at the end of the URL for a Dependabot alert within GitHub, or in `number` fields in the response from the `GET /repos/{owner}/{repo}/dependabot/alerts` operation. | [required] |
**dependabot_update_alert_request** | [**DependabotUpdateAlertRequest**](DependabotUpdateAlertRequest.md) |  | [required] |

### Return type

[**models::DependabotAlert**](dependabot-alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

