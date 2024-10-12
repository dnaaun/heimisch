# CheckRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The id of the check. | 
**head_sha** | **String** | The SHA of the commit that is being checked. | 
**node_id** | **String** |  | 
**external_id** | Option<**String**> |  | 
**url** | **String** |  | 
**html_url** | Option<**String**> |  | 
**details_url** | Option<**String**> |  | 
**status** | **String** | The phase of the lifecycle that the check is currently in. Statuses of waiting, requested, and pending are reserved for GitHub Actions check runs. | 
**conclusion** | Option<**String**> |  | 
**started_at** | Option<**String**> |  | 
**completed_at** | Option<**String**> |  | 
**output** | [**models::CheckRunOutput**](check_run_output.md) |  | 
**name** | **String** | The name of the check. | 
**check_suite** | Option<[**models::CheckRunCheckSuite**](check_run_check_suite.md)> |  | 
**app** | Option<[**models::NullableIntegration**](nullable-integration.md)> |  | 
**pull_requests** | [**Vec<models::PullRequestMinimal>**](pull-request-minimal.md) | Pull requests that are open with a `head_sha` or `head_branch` that matches the check. The returned pull requests do not necessarily indicate pull requests that triggered the check. | 
**deployment** | Option<[**models::DeploymentSimple**](deployment-simple.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


