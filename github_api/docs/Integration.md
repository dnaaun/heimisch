# Integration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | Unique identifier of the GitHub app | 
**slug** | Option<**String**> | The slug name of the GitHub app | [optional]
**node_id** | **String** |  | 
**client_id** | Option<**String**> |  | [optional]
**owner** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**name** | **String** | The name of the GitHub app | 
**description** | Option<**String**> |  | 
**external_url** | **String** |  | 
**html_url** | **String** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**permissions** | [**models::IntegrationPermissions**](integration_permissions.md) |  | 
**events** | **Vec<String>** | The list of events for the GitHub app | 
**installations_count** | Option<**i32**> | The number of installations associated with the GitHub app | [optional]
**client_secret** | Option<**String**> |  | [optional]
**webhook_secret** | Option<**String**> |  | [optional]
**pem** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


