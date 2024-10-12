# WebhookDeploymentReviewRequested

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action** | **String** |  | 
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**environment** | **String** |  | 
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**organization** | [**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md) |  | 
**repository** | [**models::RepositoryWebhooks**](repository-webhooks.md) |  | 
**requestor** | Option<[**models::WebhooksUser**](webhooks_user.md)> |  | 
**reviewers** | [**Vec<models::WebhookDeploymentReviewRequestedReviewersInner>**](webhook_deployment_review_requested_reviewers_inner.md) |  | 
**sender** | [**models::SimpleUser**](simple-user.md) |  | 
**since** | **String** |  | 
**workflow_job_run** | [**models::WebhookDeploymentReviewRequestedWorkflowJobRun**](webhook_deployment_review_requested_workflow_job_run.md) |  | 
**workflow_run** | Option<[**models::DeploymentWorkflowRun3**](Deployment_Workflow_Run_3.md)> |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


