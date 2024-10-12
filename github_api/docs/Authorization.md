# Authorization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**url** | **String** |  | 
**scopes** | Option<**Vec<String>**> | A list of scopes that this authorization is in. | 
**token** | **String** |  | 
**token_last_eight** | Option<**String**> |  | 
**hashed_token** | Option<**String**> |  | 
**app** | [**models::AuthorizationApp**](authorization_app.md) |  | 
**note** | Option<**String**> |  | 
**note_url** | Option<**String**> |  | 
**updated_at** | **String** |  | 
**created_at** | **String** |  | 
**fingerprint** | Option<**String**> |  | 
**user** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | [optional]
**installation** | Option<[**models::NullableScopedInstallation**](nullable-scoped-installation.md)> |  | [optional]
**expires_at** | Option<**String**> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


