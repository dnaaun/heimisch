# Activity

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**node_id** | **String** |  | 
**before** | **String** | The SHA of the commit before the activity. | 
**after** | **String** | The SHA of the commit after the activity. | 
**r#ref** | **String** | The full Git reference, formatted as `refs/heads/<branch name>`. | 
**timestamp** | **String** | The time when the activity occurred. | 
**activity_type** | **String** | The type of the activity that was performed. | 
**actor** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


