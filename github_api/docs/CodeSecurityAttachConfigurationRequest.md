# CodeSecurityAttachConfigurationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**scope** | **String** | The type of repositories to attach the configuration to. `selected` means the configuration will be attached to only the repositories specified by `selected_repository_ids` | 
**selected_repository_ids** | Option<**Vec<i32>**> | An array of repository IDs to attach the configuration to. You can only provide a list of repository ids when the `scope` is set to `selected`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


