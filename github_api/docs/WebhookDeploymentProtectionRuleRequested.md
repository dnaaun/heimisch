# WebhookDeploymentProtectionRuleRequested

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | Option<**String**> |  | [optional]
**environment** | Option<**String**> | The name of the environment that has the deployment protection rule. | [optional]
**event** | Option<**String**> | The event that triggered the deployment protection rule. | [optional]
**deployment_callback_url** | Option<**String**> | The URL to review the deployment protection rule. | [optional]
**deployment** | Option<[**models::Deployment**](deployment.md)> |  | [optional]
**pull_requests** | Option<[**Vec<models::PullRequest>**](pull-request.md)> |  | [optional]
**repository** | Option<[**models::RepositoryWebhooks**](repository-webhooks.md)> |  | [optional]
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**sender** | Option<[**models::SimpleUser**](simple-user.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


