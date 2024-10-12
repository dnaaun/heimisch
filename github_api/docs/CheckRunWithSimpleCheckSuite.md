# CheckRunWithSimpleCheckSuite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**app** | Option<[**models::NullableIntegration**](nullable-integration.md)> |  | 
**check_suite** | [**models::SimpleCheckSuite**](simple-check-suite.md) |  | 
**completed_at** | Option<**String**> |  | 
**conclusion** | Option<**String**> |  | 
**deployment** | Option<[**models::DeploymentSimple**](deployment-simple.md)> |  | [optional]
**details_url** | **String** |  | 
**external_id** | **String** |  | 
**head_sha** | **String** | The SHA of the commit that is being checked. | 
**html_url** | **String** |  | 
**id** | **i32** | The id of the check. | 
**name** | **String** | The name of the check. | 
**node_id** | **String** |  | 
**output** | [**models::CheckRunWithSimpleCheckSuiteOutput**](check_run_with_simple_check_suite_output.md) |  | 
**pull_requests** | [**Vec<models::PullRequestMinimal>**](pull-request-minimal.md) |  | 
**started_at** | **String** |  | 
**status** | **String** | The phase of the lifecycle that the check is currently in. | 
**url** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


