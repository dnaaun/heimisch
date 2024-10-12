# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i32** | The id of the job. | 
**run_id** | **i32** | The id of the associated workflow run. | 
**run_url** | **String** |  | 
**run_attempt** | Option<**i32**> | Attempt number of the associated workflow run, 1 for first attempt and higher if the workflow was re-run. | [optional]
**node_id** | **String** |  | 
**head_sha** | **String** | The SHA of the commit that is being run. | 
**url** | **String** |  | 
**html_url** | Option<**String**> |  | 
**status** | **String** | The phase of the lifecycle that the job is currently in. | 
**conclusion** | Option<**String**> | The outcome of the job. | 
**created_at** | **String** | The time that the job created, in ISO 8601 format. | 
**started_at** | **String** | The time that the job started, in ISO 8601 format. | 
**completed_at** | Option<**String**> | The time that the job finished, in ISO 8601 format. | 
**name** | **String** | The name of the job. | 
**steps** | Option<[**Vec<models::JobStepsInner>**](job_steps_inner.md)> | Steps in this job. | [optional]
**check_run_url** | **String** |  | 
**labels** | **Vec<String>** | Labels for the workflow job. Specified by the \"runs_on\" attribute in the action's workflow file. | 
**runner_id** | Option<**i32**> | The ID of the runner to which this job has been assigned. (If a runner hasn't yet been assigned, this will be null.) | 
**runner_name** | Option<**String**> | The name of the runner to which this job has been assigned. (If a runner hasn't yet been assigned, this will be null.) | 
**runner_group_id** | Option<**i32**> | The ID of the runner group to which this job has been assigned. (If a runner hasn't yet been assigned, this will be null.) | 
**runner_group_name** | Option<**String**> | The name of the runner group to which this job has been assigned. (If a runner hasn't yet been assigned, this will be null.) | 
**workflow_name** | Option<**String**> | The name of the workflow. | 
**head_branch** | Option<**String**> | The name of the current branch. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


