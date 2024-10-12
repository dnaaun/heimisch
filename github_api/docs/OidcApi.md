# \OidcApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**oidc_slash_get_oidc_custom_sub_template_for_org**](OidcApi.md#oidc_slash_get_oidc_custom_sub_template_for_org) | **GET** /orgs/{org}/actions/oidc/customization/sub | Get the customization template for an OIDC subject claim for an organization
[**oidc_slash_update_oidc_custom_sub_template_for_org**](OidcApi.md#oidc_slash_update_oidc_custom_sub_template_for_org) | **PUT** /orgs/{org}/actions/oidc/customization/sub | Set the customization template for an OIDC subject claim for an organization



## oidc_slash_get_oidc_custom_sub_template_for_org

> models::OidcCustomSub oidc_slash_get_oidc_custom_sub_template_for_org(org)
Get the customization template for an OIDC subject claim for an organization

Gets the customization template for an OpenID Connect (OIDC) subject claim.  OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::OidcCustomSub**](oidc-custom-sub.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## oidc_slash_update_oidc_custom_sub_template_for_org

> serde_json::Value oidc_slash_update_oidc_custom_sub_template_for_org(org, oidc_custom_sub)
Set the customization template for an OIDC subject claim for an organization

Creates or updates the customization template for an OpenID Connect (OIDC) subject claim.  OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**oidc_custom_sub** | [**OidcCustomSub**](OidcCustomSub.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

