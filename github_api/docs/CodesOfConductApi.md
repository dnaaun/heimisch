# \CodesOfConductApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**codes_of_conduct_slash_get_all_codes_of_conduct**](CodesOfConductApi.md#codes_of_conduct_slash_get_all_codes_of_conduct) | **GET** /codes_of_conduct | Get all codes of conduct
[**codes_of_conduct_slash_get_conduct_code**](CodesOfConductApi.md#codes_of_conduct_slash_get_conduct_code) | **GET** /codes_of_conduct/{key} | Get a code of conduct



## codes_of_conduct_slash_get_all_codes_of_conduct

> Vec<models::CodeOfConduct> codes_of_conduct_slash_get_all_codes_of_conduct()
Get all codes of conduct

Returns array of all GitHub's codes of conduct.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::CodeOfConduct>**](code-of-conduct.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## codes_of_conduct_slash_get_conduct_code

> models::CodeOfConduct codes_of_conduct_slash_get_conduct_code(key)
Get a code of conduct

Returns information about the specified GitHub code of conduct.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key** | **String** |  | [required] |

### Return type

[**models::CodeOfConduct**](code-of-conduct.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

