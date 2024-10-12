# PendingDeployment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**environment** | [**models::PendingDeploymentEnvironment**](pending_deployment_environment.md) |  | 
**wait_timer** | **i32** | The set duration of the wait timer | 
**wait_timer_started_at** | Option<**String**> | The time that the wait timer began. | 
**current_user_can_approve** | **bool** | Whether the currently authenticated user can approve the deployment | 
**reviewers** | [**Vec<models::PendingDeploymentReviewersInner>**](pending_deployment_reviewers_inner.md) | The people or teams that may approve jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


