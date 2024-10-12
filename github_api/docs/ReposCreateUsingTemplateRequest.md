# ReposCreateUsingTemplateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**owner** | Option<**String**> | The organization or person who will own the new repository. To create a new repository in an organization, the authenticated user must be a member of the specified organization. | [optional]
**name** | **String** | The name of the new repository. | 
**description** | Option<**String**> | A short description of the new repository. | [optional]
**include_all_branches** | Option<**bool**> | Set to `true` to include the directory structure and files from all branches in the template repository, and not just the default branch. Default: `false`. | [optional][default to false]
**private** | Option<**bool**> | Either `true` to create a new private repository or `false` to create a new public one. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


