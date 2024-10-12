# Migration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**owner** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**guid** | **String** |  | 
**state** | **String** |  | 
**lock_repositories** | **bool** |  | 
**exclude_metadata** | **bool** |  | 
**exclude_git_data** | **bool** |  | 
**exclude_attachments** | **bool** |  | 
**exclude_releases** | **bool** |  | 
**exclude_owner_projects** | **bool** |  | 
**org_metadata_only** | **bool** |  | 
**repositories** | [**Vec<models::Repository>**](repository.md) | The repositories included in the migration. Only returned for export migrations. | 
**url** | **String** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**node_id** | **String** |  | 
**archive_url** | Option<**String**> |  | [optional]
**exclude** | Option<**Vec<String>**> | Exclude related items from being returned in the response in order to improve performance of the request. The array can include any of: `\"repositories\"`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


