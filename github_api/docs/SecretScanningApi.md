# \SecretScanningApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**secret_scanning_slash_create_push_protection_bypass**](SecretScanningApi.md#secret_scanning_slash_create_push_protection_bypass) | **POST** /repos/{owner}/{repo}/secret-scanning/push-protection-bypasses | Create a push protection bypass
[**secret_scanning_slash_get_alert**](SecretScanningApi.md#secret_scanning_slash_get_alert) | **GET** /repos/{owner}/{repo}/secret-scanning/alerts/{alert_number} | Get a secret scanning alert
[**secret_scanning_slash_list_alerts_for_enterprise**](SecretScanningApi.md#secret_scanning_slash_list_alerts_for_enterprise) | **GET** /enterprises/{enterprise}/secret-scanning/alerts | List secret scanning alerts for an enterprise
[**secret_scanning_slash_list_alerts_for_org**](SecretScanningApi.md#secret_scanning_slash_list_alerts_for_org) | **GET** /orgs/{org}/secret-scanning/alerts | List secret scanning alerts for an organization
[**secret_scanning_slash_list_alerts_for_repo**](SecretScanningApi.md#secret_scanning_slash_list_alerts_for_repo) | **GET** /repos/{owner}/{repo}/secret-scanning/alerts | List secret scanning alerts for a repository
[**secret_scanning_slash_list_locations_for_alert**](SecretScanningApi.md#secret_scanning_slash_list_locations_for_alert) | **GET** /repos/{owner}/{repo}/secret-scanning/alerts/{alert_number}/locations | List locations for a secret scanning alert
[**secret_scanning_slash_update_alert**](SecretScanningApi.md#secret_scanning_slash_update_alert) | **PATCH** /repos/{owner}/{repo}/secret-scanning/alerts/{alert_number} | Update a secret scanning alert



## secret_scanning_slash_create_push_protection_bypass

> models::SecretScanningPushProtectionBypass secret_scanning_slash_create_push_protection_bypass(owner, repo, secret_scanning_create_push_protection_bypass_request)
Create a push protection bypass

Creates a bypass for a previously push protected secret.  The authenticated user must be the original author of the committed secret.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**secret_scanning_create_push_protection_bypass_request** | [**SecretScanningCreatePushProtectionBypassRequest**](SecretScanningCreatePushProtectionBypassRequest.md) |  | [required] |

### Return type

[**models::SecretScanningPushProtectionBypass**](secret-scanning-push-protection-bypass.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_scanning_slash_get_alert

> models::SecretScanningAlert secret_scanning_slash_get_alert(owner, repo, alert_number)
Get a secret scanning alert

Gets a single secret scanning alert detected in an eligible repository.  The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**alert_number** | **i32** | The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation. | [required] |

### Return type

[**models::SecretScanningAlert**](secret-scanning-alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_scanning_slash_list_alerts_for_enterprise

> Vec<models::OrganizationSecretScanningAlert> secret_scanning_slash_list_alerts_for_enterprise(enterprise, state, secret_type, resolution, sort, direction, per_page, before, after, validity, is_publicly_leaked, is_multi_repo)
List secret scanning alerts for an enterprise

Lists secret scanning alerts for eligible repositories in an enterprise, from newest to oldest.  Alerts are only returned for organizations in the enterprise for which the authenticated user is an organization owner or a [security manager](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).  The authenticated user must be a member of the enterprise in order to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope or `security_events` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enterprise** | **String** | The slug version of the enterprise name. You can also substitute this value with the enterprise id. | [required] |
**state** | Option<**String**> | Set to `open` or `resolved` to only list secret scanning alerts in a specific state. |  |
**secret_type** | Option<**String**> | A comma-separated list of secret types to return. By default all secret types are returned. See \"[Supported secret scanning patterns](https://docs.github.com/code-security/secret-scanning/introduction/supported-secret-scanning-patterns#supported-secrets)\" for a complete list of secret types. |  |
**resolution** | Option<**String**> | A comma-separated list of resolutions. Only secret scanning alerts with one of these resolutions are listed. Valid resolutions are `false_positive`, `wont_fix`, `revoked`, `pattern_edited`, `pattern_deleted` or `used_in_tests`. |  |
**sort** | Option<**String**> | The property to sort the results by. `created` means when the alert was created. `updated` means when the alert was updated or resolved. |  |[default to created]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**validity** | Option<**String**> | A comma-separated list of validities that, when present, will return alerts that match the validities in this list. Valid options are `active`, `inactive`, and `unknown`. |  |
**is_publicly_leaked** | Option<**bool**> | A boolean value representing whether or not to filter alerts by the publicly-leaked tag being present. |  |[default to false]
**is_multi_repo** | Option<**bool**> | A boolean value representing whether or not to filter alerts by the multi-repo tag being present. |  |[default to false]

### Return type

[**Vec<models::OrganizationSecretScanningAlert>**](organization-secret-scanning-alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_scanning_slash_list_alerts_for_org

> Vec<models::OrganizationSecretScanningAlert> secret_scanning_slash_list_alerts_for_org(org, state, secret_type, resolution, sort, direction, page, per_page, before, after, validity, is_publicly_leaked, is_multi_repo)
List secret scanning alerts for an organization

Lists secret scanning alerts for eligible repositories in an organization, from newest to oldest.  The authenticated user must be an administrator or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**state** | Option<**String**> | Set to `open` or `resolved` to only list secret scanning alerts in a specific state. |  |
**secret_type** | Option<**String**> | A comma-separated list of secret types to return. By default all secret types are returned. See \"[Supported secret scanning patterns](https://docs.github.com/code-security/secret-scanning/introduction/supported-secret-scanning-patterns#supported-secrets)\" for a complete list of secret types. |  |
**resolution** | Option<**String**> | A comma-separated list of resolutions. Only secret scanning alerts with one of these resolutions are listed. Valid resolutions are `false_positive`, `wont_fix`, `revoked`, `pattern_edited`, `pattern_deleted` or `used_in_tests`. |  |
**sort** | Option<**String**> | The property to sort the results by. `created` means when the alert was created. `updated` means when the alert was updated or resolved. |  |[default to created]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events before this cursor. To receive an initial cursor on your first request, include an empty \"before\" query string. |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events after this cursor.  To receive an initial cursor on your first request, include an empty \"after\" query string. |  |
**validity** | Option<**String**> | A comma-separated list of validities that, when present, will return alerts that match the validities in this list. Valid options are `active`, `inactive`, and `unknown`. |  |
**is_publicly_leaked** | Option<**bool**> | A boolean value representing whether or not to filter alerts by the publicly-leaked tag being present. |  |[default to false]
**is_multi_repo** | Option<**bool**> | A boolean value representing whether or not to filter alerts by the multi-repo tag being present. |  |[default to false]

### Return type

[**Vec<models::OrganizationSecretScanningAlert>**](organization-secret-scanning-alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_scanning_slash_list_alerts_for_repo

> Vec<models::SecretScanningAlert> secret_scanning_slash_list_alerts_for_repo(owner, repo, state, secret_type, resolution, sort, direction, page, per_page, before, after, validity, is_publicly_leaked, is_multi_repo)
List secret scanning alerts for a repository

Lists secret scanning alerts for an eligible repository, from newest to oldest.  The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**state** | Option<**String**> | Set to `open` or `resolved` to only list secret scanning alerts in a specific state. |  |
**secret_type** | Option<**String**> | A comma-separated list of secret types to return. By default all secret types are returned. See \"[Supported secret scanning patterns](https://docs.github.com/code-security/secret-scanning/introduction/supported-secret-scanning-patterns#supported-secrets)\" for a complete list of secret types. |  |
**resolution** | Option<**String**> | A comma-separated list of resolutions. Only secret scanning alerts with one of these resolutions are listed. Valid resolutions are `false_positive`, `wont_fix`, `revoked`, `pattern_edited`, `pattern_deleted` or `used_in_tests`. |  |
**sort** | Option<**String**> | The property to sort the results by. `created` means when the alert was created. `updated` means when the alert was updated or resolved. |  |[default to created]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events before this cursor. To receive an initial cursor on your first request, include an empty \"before\" query string. |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for events after this cursor.  To receive an initial cursor on your first request, include an empty \"after\" query string. |  |
**validity** | Option<**String**> | A comma-separated list of validities that, when present, will return alerts that match the validities in this list. Valid options are `active`, `inactive`, and `unknown`. |  |
**is_publicly_leaked** | Option<**bool**> | A boolean value representing whether or not to filter alerts by the publicly-leaked tag being present. |  |[default to false]
**is_multi_repo** | Option<**bool**> | A boolean value representing whether or not to filter alerts by the multi-repo tag being present. |  |[default to false]

### Return type

[**Vec<models::SecretScanningAlert>**](secret-scanning-alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_scanning_slash_list_locations_for_alert

> Vec<models::SecretScanningLocation> secret_scanning_slash_list_locations_for_alert(owner, repo, alert_number, page, per_page)
List locations for a secret scanning alert

Lists all locations for a given secret scanning alert for an eligible repository.  The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**alert_number** | **i32** | The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**Vec<models::SecretScanningLocation>**](secret-scanning-location.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## secret_scanning_slash_update_alert

> models::SecretScanningAlert secret_scanning_slash_update_alert(owner, repo, alert_number, secret_scanning_update_alert_request)
Update a secret scanning alert

Updates the status of a secret scanning alert in an eligible repository.  The authenticated user must be an administrator for the repository or for the organization that owns the repository to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**alert_number** | **i32** | The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation. | [required] |
**secret_scanning_update_alert_request** | [**SecretScanningUpdateAlertRequest**](SecretScanningUpdateAlertRequest.md) |  | [required] |

### Return type

[**models::SecretScanningAlert**](secret-scanning-alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

