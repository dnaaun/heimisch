# ReposCreateOrUpdateEnvironmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**wait_timer** | Option<**i32**> | The amount of time to delay a job after the job is initially triggered. The time (in minutes) must be an integer between 0 and 43,200 (30 days). | [optional]
**prevent_self_review** | Option<**bool**> | Whether or not a user who created the job is prevented from approving their own job. | [optional]
**reviewers** | Option<[**Vec<models::ReposCreateOrUpdateEnvironmentRequestReviewersInner>**](repos_create_or_update_environment_request_reviewers_inner.md)> | The people or teams that may review jobs that reference the environment. You can list up to six users or teams as reviewers. The reviewers must have at least read access to the repository. Only one of the required reviewers needs to approve the job for it to proceed. | [optional]
**deployment_branch_policy** | Option<[**models::DeploymentBranchPolicySettings**](deployment-branch-policy-settings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


