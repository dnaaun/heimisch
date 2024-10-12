# ClassroomAcceptedAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | Unique identifier of the repository. | 
**submitted** | **bool** | Whether an accepted assignment has been submitted. | 
**passing** | **bool** | Whether a submission passed. | 
**commit_count** | **i32** | Count of student commits. | 
**grade** | **String** | Most recent grade. | 
**students** | [**Vec<models::SimpleClassroomUser>**](simple-classroom-user.md) |  | 
**repository** | [**models::SimpleClassroomRepository**](simple-classroom-repository.md) |  | 
**assignment** | [**models::SimpleClassroomAssignment**](simple-classroom-assignment.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


