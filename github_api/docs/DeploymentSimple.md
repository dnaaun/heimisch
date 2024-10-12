# DeploymentSimple

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** |  | 
**id** | **i32** | Unique identifier of the deployment | 
**node_id** | **String** |  | 
**task** | **String** | Parameter to specify a task to execute | 
**original_environment** | Option<**String**> |  | [optional]
**environment** | **String** | Name for the target deployment environment. | 
**description** | Option<**String**> |  | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**statuses_url** | **String** |  | 
**repository_url** | **String** |  | 
**transient_environment** | Option<**bool**> | Specifies if the given environment is will no longer exist at some point in the future. Default: false. | [optional]
**production_environment** | Option<**bool**> | Specifies if the given environment is one that end-users directly interact with. Default: false. | [optional]
**performed_via_github_app** | Option<[**models::NullableIntegration**](nullable-integration.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


