# EnvironmentProtectionRulesInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** |  | 
**node_id** | **String** |  | 
**r#type** | **String** |  | 
**wait_timer** | Option<**i32**> | The amount of time to delay a job after the job is initially triggered. The time (in minutes) must be an integer between 0 and 43,200 (30 days). | [optional]
**prevent_self_review** | Option<**bool**> | Whether deployments to this environment can be approved by the user who created the deployment. | [optional]
**reviewers** | Option<[**Vec<models::PendingDeploymentReviewersInner>**](pending_deployment_reviewers_inner.md)> | The people or teams that may approve jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


