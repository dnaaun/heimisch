# ReposCreateDeploymentStatusRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | **String** | The state of the status. When you set a transient deployment to `inactive`, the deployment will be shown as `destroyed` in GitHub. | 
**target_url** | Option<**String**> | The target URL to associate with this status. This URL should contain output to keep the user updated while the task is running or serve as historical information for what happened in the deployment.  > [!NOTE] > It's recommended to use the `log_url` parameter, which replaces `target_url`. | [optional][default to ]
**log_url** | Option<**String**> | The full URL of the deployment's output. This parameter replaces `target_url`. We will continue to accept `target_url` to support legacy uses, but we recommend replacing `target_url` with `log_url`. Setting `log_url` will automatically set `target_url` to the same value. Default: `\"\"` | [optional][default to ]
**description** | Option<**String**> | A short description of the status. The maximum description length is 140 characters. | [optional][default to ]
**environment** | Option<**String**> | Name for the target deployment environment, which can be changed when setting a deploy status. For example, `production`, `staging`, or `qa`. If not defined, the environment of the previous status on the deployment will be used, if it exists. Otherwise, the environment of the deployment will be used. | [optional]
**environment_url** | Option<**String**> | Sets the URL for accessing your environment. Default: `\"\"` | [optional][default to ]
**auto_inactive** | Option<**bool**> | Adds a new `inactive` status to all prior non-transient, non-production environment deployments with the same repository and `environment` name as the created status's deployment. An `inactive` status is only added to deployments that had a `success` state. Default: `true` | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


