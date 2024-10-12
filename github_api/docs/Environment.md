# Environment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | The id of the environment. | 
**node_id** | **String** |  | 
**name** | **String** | The name of the environment. | 
**url** | **String** |  | 
**html_url** | **String** |  | 
**created_at** | **String** | The time that the environment was created, in ISO 8601 format. | 
**updated_at** | **String** | The time that the environment was last updated, in ISO 8601 format. | 
**protection_rules** | Option<[**Vec<models::EnvironmentProtectionRulesInner>**](environment_protection_rules_inner.md)> | Built-in deployment protection rules for the environment. | [optional]
**deployment_branch_policy** | Option<[**models::DeploymentBranchPolicySettings**](deployment-branch-policy-settings.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


