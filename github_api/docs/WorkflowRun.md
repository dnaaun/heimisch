# WorkflowRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actor** | Option<[**models::User2**](User_2.md)> |  | 
**artifacts_url** | **String** |  | 
**cancel_url** | **String** |  | 
**check_suite_id** | **i32** |  | 
**check_suite_node_id** | **String** |  | 
**check_suite_url** | **String** |  | 
**conclusion** | Option<**String**> |  | 
**created_at** | **String** |  | 
**event** | **String** |  | 
**head_branch** | Option<**String**> |  | 
**head_commit** | [**models::SimpleCommit**](SimpleCommit.md) |  | 
**head_repository** | [**models::RepositoryLite**](Repository_Lite.md) |  | 
**head_sha** | **String** |  | 
**html_url** | **String** |  | 
**id** | **i32** |  | 
**jobs_url** | **String** |  | 
**logs_url** | **String** |  | 
**name** | Option<**String**> |  | 
**node_id** | **String** |  | 
**path** | **String** |  | 
**previous_attempt_url** | Option<**String**> |  | 
**pull_requests** | [**Vec<models::WorkflowRunPullRequestsInner>**](Workflow_Run_pull_requests_inner.md) |  | 
**referenced_workflows** | Option<[**Vec<models::DeploymentWorkflowRunReferencedWorkflowsInner>**](Deployment_Workflow_Run_referenced_workflows_inner.md)> |  | [optional]
**repository** | [**models::RepositoryLite**](Repository_Lite.md) |  | 
**rerun_url** | **String** |  | 
**run_attempt** | **i32** |  | 
**run_number** | **i32** |  | 
**run_started_at** | **String** |  | 
**status** | **String** |  | 
**triggering_actor** | Option<[**models::User2**](User_2.md)> |  | 
**updated_at** | **String** |  | 
**url** | **String** |  | 
**workflow_id** | **i32** |  | 
**workflow_url** | **String** |  | 
**display_title** | Option<**String**> | The event-specific title associated with the run or the run-name if set, or the value of `run-name` if it is set in the workflow. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


