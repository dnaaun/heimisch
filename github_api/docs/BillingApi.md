# \BillingApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**billing_slash_get_github_actions_billing_org**](BillingApi.md#billing_slash_get_github_actions_billing_org) | **GET** /orgs/{org}/settings/billing/actions | Get GitHub Actions billing for an organization
[**billing_slash_get_github_actions_billing_user**](BillingApi.md#billing_slash_get_github_actions_billing_user) | **GET** /users/{username}/settings/billing/actions | Get GitHub Actions billing for a user
[**billing_slash_get_github_packages_billing_org**](BillingApi.md#billing_slash_get_github_packages_billing_org) | **GET** /orgs/{org}/settings/billing/packages | Get GitHub Packages billing for an organization
[**billing_slash_get_github_packages_billing_user**](BillingApi.md#billing_slash_get_github_packages_billing_user) | **GET** /users/{username}/settings/billing/packages | Get GitHub Packages billing for a user
[**billing_slash_get_shared_storage_billing_org**](BillingApi.md#billing_slash_get_shared_storage_billing_org) | **GET** /orgs/{org}/settings/billing/shared-storage | Get shared storage billing for an organization
[**billing_slash_get_shared_storage_billing_user**](BillingApi.md#billing_slash_get_shared_storage_billing_user) | **GET** /users/{username}/settings/billing/shared-storage | Get shared storage billing for a user



## billing_slash_get_github_actions_billing_org

> models::ActionsBillingUsage billing_slash_get_github_actions_billing_org(org)
Get GitHub Actions billing for an organization

Gets the summary of the free and paid GitHub Actions minutes used.  Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see \"[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)\".  OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::ActionsBillingUsage**](actions-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_slash_get_github_actions_billing_user

> models::ActionsBillingUsage billing_slash_get_github_actions_billing_user(username)
Get GitHub Actions billing for a user

Gets the summary of the free and paid GitHub Actions minutes used.  Paid minutes only apply to workflows in private repositories that use GitHub-hosted runners. Minutes used is listed for each GitHub-hosted runner operating system. Any job re-runs are also included in the usage. The usage returned includes any minute multipliers for macOS and Windows runners, and is rounded up to the nearest whole minute. For more information, see \"[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)\".  OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::ActionsBillingUsage**](actions-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_slash_get_github_packages_billing_org

> models::PackagesBillingUsage billing_slash_get_github_packages_billing_org(org)
Get GitHub Packages billing for an organization

Gets the free and paid storage used for GitHub Packages in gigabytes.  Paid minutes only apply to packages stored for private repositories. For more information, see \"[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages).\"  OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::PackagesBillingUsage**](packages-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_slash_get_github_packages_billing_user

> models::PackagesBillingUsage billing_slash_get_github_packages_billing_user(username)
Get GitHub Packages billing for a user

Gets the free and paid storage used for GitHub Packages in gigabytes.  Paid minutes only apply to packages stored for private repositories. For more information, see \"[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages).\"  OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::PackagesBillingUsage**](packages-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_slash_get_shared_storage_billing_org

> models::CombinedBillingUsage billing_slash_get_shared_storage_billing_org(org)
Get shared storage billing for an organization

Gets the estimated paid and estimated total storage used for GitHub Actions and GitHub Packages.  Paid minutes only apply to packages stored for private repositories. For more information, see \"[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages).\"  OAuth app tokens and personal access tokens (classic) need the `repo` or `admin:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::CombinedBillingUsage**](combined-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## billing_slash_get_shared_storage_billing_user

> models::CombinedBillingUsage billing_slash_get_shared_storage_billing_user(username)
Get shared storage billing for a user

Gets the estimated paid and estimated total storage used for GitHub Actions and GitHub Packages.  Paid minutes only apply to packages stored for private repositories. For more information, see \"[Managing billing for GitHub Packages](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-packages).\"  OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::CombinedBillingUsage**](combined-billing-usage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

