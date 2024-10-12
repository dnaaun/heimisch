# RepositoryInvitation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | Unique identifier of the repository invitation. | 
**repository** | [**models::MinimalRepository**](minimal-repository.md) |  | 
**invitee** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**inviter** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**permissions** | **String** | The permission associated with the invitation. | 
**created_at** | **String** |  | 
**expired** | Option<**bool**> | Whether or not the invitation has expired | [optional]
**url** | **String** | URL for the repository invitation | 
**html_url** | **String** |  | 
**node_id** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


