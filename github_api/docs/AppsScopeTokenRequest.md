# AppsScopeTokenRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token** | **String** | The access token used to authenticate to the GitHub API. | 
**target** | Option<**String**> | The name of the user or organization to scope the user access token to. **Required** unless `target_id` is specified. | [optional]
**target_id** | Option<**i32**> | The ID of the user or organization to scope the user access token to. **Required** unless `target` is specified. | [optional]
**repositories** | Option<**Vec<String>**> | The list of repository names to scope the user access token to. `repositories` may not be specified if `repository_ids` is specified. | [optional]
**repository_ids** | Option<**Vec<i32>**> | The list of repository IDs to scope the user access token to. `repository_ids` may not be specified if `repositories` is specified. | [optional]
**permissions** | Option<[**models::AppPermissions**](app-permissions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


