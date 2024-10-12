# RepositoryRuleRequiredStatusChecksParameters

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**do_not_enforce_on_create** | Option<**bool**> | Allow repositories and branches to be created if a check would otherwise prohibit it. | [optional]
**required_status_checks** | [**Vec<models::RepositoryRuleParamsStatusCheckConfiguration>**](repository-rule-params-status-check-configuration.md) | Status checks that are required. | 
**strict_required_status_checks_policy** | **bool** | Whether pull requests targeting a matching branch must be tested with the latest code. This setting will not take effect unless at least one status check is enabled. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


