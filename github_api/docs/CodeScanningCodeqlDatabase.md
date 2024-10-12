# CodeScanningCodeqlDatabase

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | The ID of the CodeQL database. | 
**name** | **String** | The name of the CodeQL database. | 
**language** | **String** | The language of the CodeQL database. | 
**uploader** | [**models::SimpleUser**](simple-user.md) |  | 
**content_type** | **String** | The MIME type of the CodeQL database file. | 
**size** | **i32** | The size of the CodeQL database file in bytes. | 
**created_at** | **String** | The date and time at which the CodeQL database was created, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ. | 
**updated_at** | **String** | The date and time at which the CodeQL database was last updated, in ISO 8601 format':' YYYY-MM-DDTHH:MM:SSZ. | 
**url** | **String** | The URL at which to download the CodeQL database. The `Accept` header must be set to the value of the `content_type` property. | 
**commit_oid** | Option<**String**> | The commit SHA of the repository at the time the CodeQL database was created. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


