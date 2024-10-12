# CodespacesSetCodespacesAccessRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**visibility** | **String** | Which users can access codespaces in the organization. `disabled` means that no users can access codespaces in the organization. | 
**selected_usernames** | Option<**Vec<String>**> | The usernames of the organization members who should have access to codespaces in the organization. Required when `visibility` is `selected_members`. The provided list of usernames will replace any existing value. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


