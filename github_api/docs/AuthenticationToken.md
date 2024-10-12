# AuthenticationToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**token** | **String** | The token used for authentication | 
**expires_at** | **String** | The time this token expires | 
**permissions** | Option<[**serde_json::Value**](.md)> |  | [optional]
**repositories** | Option<[**Vec<models::Repository>**](repository.md)> | The repositories this token has access to | [optional]
**single_file** | Option<**String**> |  | [optional]
**repository_selection** | Option<**String**> | Describe whether all repositories have been selected or there's a selection involved | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


