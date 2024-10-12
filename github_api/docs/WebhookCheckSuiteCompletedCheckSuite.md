# WebhookCheckSuiteCompletedCheckSuite

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**after** | Option<**String**> |  | 
**app** | [**models::App2**](App_2.md) |  | 
**before** | Option<**String**> |  | 
**check_runs_url** | **String** |  | 
**conclusion** | Option<**String**> | The summary conclusion for all check runs that are part of the check suite. This value will be `null` until the check run has `completed`. | 
**created_at** | **String** |  | 
**head_branch** | Option<**String**> | The head branch name the changes are on. | 
**head_commit** | [**models::SimpleCommit**](SimpleCommit.md) |  | 
**head_sha** | **String** | The SHA of the head commit that is being checked. | 
**id** | **i32** |  | 
**latest_check_runs_count** | **i32** |  | 
**node_id** | **String** |  | 
**pull_requests** | [**Vec<models::CheckRunPullRequest>**](Check_Run_Pull_Request.md) | An array of pull requests that match this check suite. A pull request matches a check suite if they have the same `head_sha` and `head_branch`. When the check suite's `head_branch` is in a forked repository it will be `null` and the `pull_requests` array will be empty. | 
**rerequestable** | Option<**bool**> |  | [optional]
**runs_rerequestable** | Option<**bool**> |  | [optional]
**status** | Option<**String**> | The summary status for all check runs that are part of the check suite. Can be `requested`, `in_progress`, or `completed`. | 
**updated_at** | **String** |  | 
**url** | **String** | URL that points to the check suite API resource. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


