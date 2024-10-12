# Commit1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**added** | Option<**Vec<String>**> | An array of files added in the commit. | [optional]
**author** | [**models::Committer**](Committer.md) |  | 
**committer** | [**models::Committer**](Committer.md) |  | 
**distinct** | **bool** | Whether this commit is distinct from any that have been pushed before. | 
**id** | **String** |  | 
**message** | **String** | The commit message. | 
**modified** | Option<**Vec<String>**> | An array of files modified by the commit. | [optional]
**removed** | Option<**Vec<String>**> | An array of files removed in the commit. | [optional]
**timestamp** | **String** | The ISO 8601 timestamp of the commit. | 
**tree_id** | **String** |  | 
**url** | **String** | URL that points to the commit API resource. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


