# DeploymentStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | 
**id** | **i64** |  | 
**node_id** | **String** |  | 
**state** | **String** | The state of the status. | 
**creator** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**description** | **String** | A short description of the status. | [default to ]
**environment** | Option<**String**> | The environment of the deployment that the status is for. | [optional][default to ]
**target_url** | **String** | Closing down notice: the URL to associate with this status. | [default to ]
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**deployment_url** | **String** |  | 
**repository_url** | **String** |  | 
**environment_url** | Option<**String**> | The URL for accessing your environment. | [optional][default to ]
**log_url** | Option<**String**> | The URL to associate with this status. | [optional][default to ]
**performed_via_github_app** | Option<[**models::NullableIntegration**](nullable-integration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


