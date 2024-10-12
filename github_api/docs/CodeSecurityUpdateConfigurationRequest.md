# CodeSecurityUpdateConfigurationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the code security configuration. Must be unique within the organization. | [optional]
**description** | Option<**String**> | A description of the code security configuration | [optional]
**advanced_security** | Option<**String**> | The enablement status of GitHub Advanced Security | [optional]
**dependency_graph** | Option<**String**> | The enablement status of Dependency Graph | [optional]
**dependency_graph_autosubmit_action** | Option<**String**> | The enablement status of Automatic dependency submission | [optional]
**dependency_graph_autosubmit_action_options** | Option<[**models::CodeSecurityUpdateConfigurationRequestDependencyGraphAutosubmitActionOptions**](code_security_update_configuration_request_dependency_graph_autosubmit_action_options.md)> |  | [optional]
**dependabot_alerts** | Option<**String**> | The enablement status of Dependabot alerts | [optional]
**dependabot_security_updates** | Option<**String**> | The enablement status of Dependabot security updates | [optional]
**code_scanning_default_setup** | Option<**String**> | The enablement status of code scanning default setup | [optional]
**secret_scanning** | Option<**String**> | The enablement status of secret scanning | [optional]
**secret_scanning_push_protection** | Option<**String**> | The enablement status of secret scanning push protection | [optional]
**secret_scanning_delegated_bypass** | Option<**String**> | The enablement status of secret scanning delegated bypass | [optional]
**secret_scanning_delegated_bypass_options** | Option<[**models::CodeSecurityCreateConfigurationRequestSecretScanningDelegatedBypassOptions**](code_security_create_configuration_request_secret_scanning_delegated_bypass_options.md)> |  | [optional]
**secret_scanning_validity_checks** | Option<**String**> | The enablement status of secret scanning validity checks | [optional]
**secret_scanning_non_provider_patterns** | Option<**String**> | The enablement status of secret scanning non-provider patterns | [optional]
**private_vulnerability_reporting** | Option<**String**> | The enablement status of private vulnerability reporting | [optional]
**enforcement** | Option<**String**> | The enforcement status for a security configuration | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


