# \GitignoreApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gitignore_slash_get_all_templates**](GitignoreApi.md#gitignore_slash_get_all_templates) | **GET** /gitignore/templates | Get all gitignore templates
[**gitignore_slash_get_template**](GitignoreApi.md#gitignore_slash_get_template) | **GET** /gitignore/templates/{name} | Get a gitignore template



## gitignore_slash_get_all_templates

> Vec<String> gitignore_slash_get_all_templates()
Get all gitignore templates

List all templates available to pass as an option when [creating a repository](https://docs.github.com/rest/repos/repos#create-a-repository-for-the-authenticated-user).

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gitignore_slash_get_template

> models::GitignoreTemplate gitignore_slash_get_template(name)
Get a gitignore template

Get the content of a gitignore template.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.raw+json`**: Returns the raw .gitignore contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::GitignoreTemplate**](gitignore-template.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

