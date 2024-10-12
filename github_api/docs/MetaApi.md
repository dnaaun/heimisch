# \MetaApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**meta_slash_get**](MetaApi.md#meta_slash_get) | **GET** /meta | Get GitHub meta information
[**meta_slash_get_all_versions**](MetaApi.md#meta_slash_get_all_versions) | **GET** /versions | Get all API versions
[**meta_slash_get_octocat**](MetaApi.md#meta_slash_get_octocat) | **GET** /octocat | Get Octocat
[**meta_slash_get_zen**](MetaApi.md#meta_slash_get_zen) | **GET** /zen | Get the Zen of GitHub
[**meta_slash_root**](MetaApi.md#meta_slash_root) | **GET** / | GitHub API Root



## meta_slash_get

> models::ApiOverview meta_slash_get()
Get GitHub meta information

Returns meta information about GitHub, including a list of GitHub's IP addresses. For more information, see \"[About GitHub's IP addresses](https://docs.github.com/articles/about-github-s-ip-addresses/).\"  The API's response also includes a list of GitHub's domain names.  The values shown in the documentation's response are example values. You must always query the API directly to get the latest values.  > [!NOTE] > This endpoint returns both IPv4 and IPv6 addresses. However, not all features support IPv6. You should refer to the specific documentation for each feature to determine if IPv6 is supported.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiOverview**](api-overview.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meta_slash_get_all_versions

> Vec<String> meta_slash_get_all_versions()
Get all API versions

Get all supported GitHub API versions.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<String>**](string.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meta_slash_get_octocat

> String meta_slash_get_octocat(s)
Get Octocat

Get the octocat as ASCII art

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**s** | Option<**String**> | The words to show in Octocat's speech bubble |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octocat-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meta_slash_get_zen

> String meta_slash_get_zen()
Get the Zen of GitHub

Get a random sentence from the Zen of GitHub

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## meta_slash_root

> models::Root meta_slash_root()
GitHub API Root

Get Hypermedia links to resources accessible in GitHub's REST API

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Root**](root.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

