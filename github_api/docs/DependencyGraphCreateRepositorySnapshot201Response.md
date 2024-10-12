# DependencyGraphCreateRepositorySnapshot201Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | ID of the created snapshot. | 
**created_at** | **String** | The time at which the snapshot was created. | 
**result** | **String** | Either \"SUCCESS\", \"ACCEPTED\", or \"INVALID\". \"SUCCESS\" indicates that the snapshot was successfully created and the repository's dependencies were updated. \"ACCEPTED\" indicates that the snapshot was successfully created, but the repository's dependencies were not updated. \"INVALID\" indicates that the snapshot was malformed. | 
**message** | **String** | A message providing further details about the result, such as why the dependencies were not updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


