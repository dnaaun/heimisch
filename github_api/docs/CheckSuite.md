# CheckSuite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**node_id** | **String** |  | 
**head_branch** | Option<**String**> |  | 
**head_sha** | **String** | The SHA of the head commit that is being checked. | 
**status** | Option<**String**> | The phase of the lifecycle that the check suite is currently in. Statuses of waiting, requested, and pending are reserved for GitHub Actions check suites. | 
**conclusion** | Option<**String**> |  | 
**url** | Option<**String**> |  | 
**before** | Option<**String**> |  | 
**after** | Option<**String**> |  | 
**pull_requests** | Option<[**Vec<models::PullRequestMinimal>**](pull-request-minimal.md)> |  | 
**app** | Option<[**models::NullableIntegration**](nullable-integration.md)> |  | 
**repository** | [**models::MinimalRepository**](minimal-repository.md) |  | 
**created_at** | Option<**String**> |  | 
**updated_at** | Option<**String**> |  | 
**head_commit** | [**models::SimpleCommit**](simple-commit.md) |  | 
**latest_check_runs_count** | **i32** |  | 
**check_runs_url** | **String** |  | 
**rerequestable** | Option<**bool**> |  | [optional]
**runs_rerequestable** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


