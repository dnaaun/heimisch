# Installation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | The ID of the installation. | 
**account** | Option<[**models::InstallationAccount**](installation_account.md)> |  | 
**repository_selection** | **String** | Describe whether all repositories have been selected or there's a selection involved | 
**access_tokens_url** | **String** |  | 
**repositories_url** | **String** |  | 
**html_url** | **String** |  | 
**app_id** | **i32** |  | 
**target_id** | **i32** | The ID of the user or organization this token is being scoped to. | 
**target_type** | **String** |  | 
**permissions** | [**models::AppPermissions**](app-permissions.md) |  | 
**events** | **Vec<String>** |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**single_file_name** | Option<**String**> |  | 
**has_multiple_single_files** | Option<**bool**> |  | [optional]
**single_file_paths** | Option<**Vec<String>**> |  | [optional]
**app_slug** | **String** |  | 
**suspended_by** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**suspended_at** | Option<**String**> |  | 
**contact_email** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


