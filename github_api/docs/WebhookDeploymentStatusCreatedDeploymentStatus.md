# WebhookDeploymentStatusCreatedDeploymentStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** |  | 
**creator** | Option<[**models::User2**](User_2.md)> |  | 
**deployment_url** | **String** |  | 
**description** | **String** | The optional human-readable description added to the status. | 
**environment** | **String** |  | 
**environment_url** | Option<**String**> |  | [optional]
**id** | **i32** |  | 
**log_url** | Option<**String**> |  | [optional]
**node_id** | **String** |  | 
**performed_via_github_app** | Option<[**models::App7**](App_7.md)> |  | [optional]
**repository_url** | **String** |  | 
**state** | **String** | The new state. Can be `pending`, `success`, `failure`, or `error`. | 
**target_url** | **String** | The optional link added to the status. | 
**updated_at** | **String** |  | 
**url** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


