# \CodeSecurityApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**code_security_slash_attach_configuration**](CodeSecurityApi.md#code_security_slash_attach_configuration) | **POST** /orgs/{org}/code-security/configurations/{configuration_id}/attach | Attach a configuration to repositories
[**code_security_slash_create_configuration**](CodeSecurityApi.md#code_security_slash_create_configuration) | **POST** /orgs/{org}/code-security/configurations | Create a code security configuration
[**code_security_slash_delete_configuration**](CodeSecurityApi.md#code_security_slash_delete_configuration) | **DELETE** /orgs/{org}/code-security/configurations/{configuration_id} | Delete a code security configuration
[**code_security_slash_detach_configuration**](CodeSecurityApi.md#code_security_slash_detach_configuration) | **DELETE** /orgs/{org}/code-security/configurations/detach | Detach configurations from repositories
[**code_security_slash_get_configuration**](CodeSecurityApi.md#code_security_slash_get_configuration) | **GET** /orgs/{org}/code-security/configurations/{configuration_id} | Get a code security configuration
[**code_security_slash_get_configuration_for_repository**](CodeSecurityApi.md#code_security_slash_get_configuration_for_repository) | **GET** /repos/{owner}/{repo}/code-security-configuration | Get the code security configuration associated with a repository
[**code_security_slash_get_configurations_for_org**](CodeSecurityApi.md#code_security_slash_get_configurations_for_org) | **GET** /orgs/{org}/code-security/configurations | Get code security configurations for an organization
[**code_security_slash_get_default_configurations**](CodeSecurityApi.md#code_security_slash_get_default_configurations) | **GET** /orgs/{org}/code-security/configurations/defaults | Get default code security configurations
[**code_security_slash_get_repositories_for_configuration**](CodeSecurityApi.md#code_security_slash_get_repositories_for_configuration) | **GET** /orgs/{org}/code-security/configurations/{configuration_id}/repositories | Get repositories associated with a code security configuration
[**code_security_slash_set_configuration_as_default**](CodeSecurityApi.md#code_security_slash_set_configuration_as_default) | **PUT** /orgs/{org}/code-security/configurations/{configuration_id}/defaults | Set a code security configuration as a default for an organization
[**code_security_slash_update_configuration**](CodeSecurityApi.md#code_security_slash_update_configuration) | **PATCH** /orgs/{org}/code-security/configurations/{configuration_id} | Update a code security configuration



## code_security_slash_attach_configuration

> serde_json::Value code_security_slash_attach_configuration(org, configuration_id, code_security_attach_configuration_request)
Attach a configuration to repositories

Attach a code security configuration to a set of repositories. If the repositories specified are already attached to a configuration, they will be re-attached to the provided configuration.  If insufficient GHAS licenses are available to attach the configuration to a repository, only free features will be enabled.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**configuration_id** | **i32** | The unique identifier of the code security configuration. | [required] |
**code_security_attach_configuration_request** | [**CodeSecurityAttachConfigurationRequest**](CodeSecurityAttachConfigurationRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_security_slash_create_configuration

> models::CodeSecurityConfiguration code_security_slash_create_configuration(org, code_security_create_configuration_request)
Create a code security configuration

Creates a code security configuration in an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**code_security_create_configuration_request** | [**CodeSecurityCreateConfigurationRequest**](CodeSecurityCreateConfigurationRequest.md) |  | [required] |

### Return type

[**models::CodeSecurityConfiguration**](code-security-configuration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_security_slash_delete_configuration

> code_security_slash_delete_configuration(org, configuration_id)
Delete a code security configuration

Deletes the desired code security configuration from an organization. Repositories attached to the configuration will retain their settings but will no longer be associated with the configuration.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**configuration_id** | **i32** | The unique identifier of the code security configuration. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_security_slash_detach_configuration

> code_security_slash_detach_configuration(org, code_security_detach_configuration_request)
Detach configurations from repositories

Detach code security configuration(s) from a set of repositories. Repositories will retain their settings but will no longer be associated with the configuration.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**code_security_detach_configuration_request** | [**CodeSecurityDetachConfigurationRequest**](CodeSecurityDetachConfigurationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_security_slash_get_configuration

> models::CodeSecurityConfiguration code_security_slash_get_configuration(org, configuration_id)
Get a code security configuration

Gets a code security configuration available in an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**configuration_id** | **i32** | The unique identifier of the code security configuration. | [required] |

### Return type

[**models::CodeSecurityConfiguration**](code-security-configuration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_security_slash_get_configuration_for_repository

> models::CodeSecurityConfigurationForRepository code_security_slash_get_configuration_for_repository(owner, repo)
Get the code security configuration associated with a repository

Get the code security configuration that manages a repository's code security settings.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::CodeSecurityConfigurationForRepository**](code-security-configuration-for-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_security_slash_get_configurations_for_org

> Vec<models::CodeSecurityConfiguration> code_security_slash_get_configurations_for_org(org, target_type, per_page, before, after)
Get code security configurations for an organization

Lists all code security configurations available in an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**target_type** | Option<**String**> | The target type of the code security configuration |  |[default to all]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |

### Return type

[**Vec<models::CodeSecurityConfiguration>**](code-security-configuration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_security_slash_get_default_configurations

> Vec<models::CodeSecurityDefaultConfigurationsInner> code_security_slash_get_default_configurations(org)
Get default code security configurations

Lists the default code security configurations for an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::CodeSecurityDefaultConfigurationsInner>**](code_security_default_configurations_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_security_slash_get_repositories_for_configuration

> Vec<models::CodeSecurityConfigurationRepositories> code_security_slash_get_repositories_for_configuration(org, configuration_id, per_page, before, after, status)
Get repositories associated with a code security configuration

Lists the repositories associated with a code security configuration in an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**configuration_id** | **i32** | The unique identifier of the code security configuration. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**status** | Option<**String**> | A comma-separated list of statuses. If specified, only repositories with these attachment statuses will be returned.  Can be: `all`, `attached`, `attaching`, `detached`, `removed`, `enforced`, `failed`, `updating`, `removed_by_enterprise` |  |[default to all]

### Return type

[**Vec<models::CodeSecurityConfigurationRepositories>**](code-security-configuration-repositories.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_security_slash_set_configuration_as_default

> models::CodeSecuritySetConfigurationAsDefault200Response code_security_slash_set_configuration_as_default(org, configuration_id, code_security_set_configuration_as_default_request)
Set a code security configuration as a default for an organization

Sets a code security configuration as a default to be applied to new repositories in your organization.  This configuration will be applied to the matching repository type (all, none, public, private and internal) by default when they are created.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**configuration_id** | **i32** | The unique identifier of the code security configuration. | [required] |
**code_security_set_configuration_as_default_request** | [**CodeSecuritySetConfigurationAsDefaultRequest**](CodeSecuritySetConfigurationAsDefaultRequest.md) |  | [required] |

### Return type

[**models::CodeSecuritySetConfigurationAsDefault200Response**](code_security_set_configuration_as_default_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_security_slash_update_configuration

> models::CodeSecurityConfiguration code_security_slash_update_configuration(org, configuration_id, code_security_update_configuration_request)
Update a code security configuration

Updates a code security configuration in an organization.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**configuration_id** | **i32** | The unique identifier of the code security configuration. | [required] |
**code_security_update_configuration_request** | [**CodeSecurityUpdateConfigurationRequest**](CodeSecurityUpdateConfigurationRequest.md) |  | [required] |

### Return type

[**models::CodeSecurityConfiguration**](code-security-configuration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

