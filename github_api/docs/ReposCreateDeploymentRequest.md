# ReposCreateDeploymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#ref** | **String** | The ref to deploy. This can be a branch, tag, or SHA. | 
**task** | Option<**String**> | Specifies a task to execute (e.g., `deploy` or `deploy:migrations`). | [optional][default to deploy]
**auto_merge** | Option<**bool**> | Attempts to automatically merge the default branch into the requested ref, if it's behind the default branch. | [optional][default to true]
**required_contexts** | Option<**Vec<String>**> | The [status](https://docs.github.com/rest/commits/statuses) contexts to verify against commit status checks. If you omit this parameter, GitHub verifies all unique contexts before creating a deployment. To bypass checking entirely, pass an empty array. Defaults to all unique contexts. | [optional]
**payload** | Option<[**models::ReposCreateDeploymentRequestPayload**](repos_create_deployment_request_payload.md)> |  | [optional]
**environment** | Option<**String**> | Name for the target deployment environment (e.g., `production`, `staging`, `qa`). | [optional][default to production]
**description** | Option<**String**> | Short description of the deployment. | [optional][default to ]
**transient_environment** | Option<**bool**> | Specifies if the given environment is specific to the deployment and will no longer exist at some point in the future. Default: `false` | [optional][default to false]
**production_environment** | Option<**bool**> | Specifies if the given environment is one that end-users directly interact with. Default: `true` when `environment` is `production` and `false` otherwise. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


