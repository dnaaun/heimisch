# CodeSecurityCreateConfigurationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the code security configuration. Must be unique within the organization. | 
**description** | **String** | A description of the code security configuration | 
**advanced_security** | Option<**String**> | The enablement status of GitHub Advanced Security | [optional][default to Disabled]
**dependency_graph** | Option<**String**> | The enablement status of Dependency Graph | [optional][default to Enabled]
**dependency_graph_autosubmit_action** | Option<**String**> | The enablement status of Automatic dependency submission | [optional][default to Disabled]
**dependency_graph_autosubmit_action_options** | Option<[**models::CodeSecurityCreateConfigurationRequestDependencyGraphAutosubmitActionOptions**](code_security_create_configuration_request_dependency_graph_autosubmit_action_options.md)> |  | [optional]
**dependabot_alerts** | Option<**String**> | The enablement status of Dependabot alerts | [optional][default to Disabled]
**dependabot_security_updates** | Option<**String**> | The enablement status of Dependabot security updates | [optional][default to Disabled]
**code_scanning_default_setup** | Option<**String**> | The enablement status of code scanning default setup | [optional][default to Disabled]
**secret_scanning** | Option<**String**> | The enablement status of secret scanning | [optional][default to Disabled]
**secret_scanning_push_protection** | Option<**String**> | The enablement status of secret scanning push protection | [optional][default to Disabled]
**secret_scanning_delegated_bypass** | Option<**String**> | The enablement status of secret scanning delegated bypass | [optional][default to Disabled]
**secret_scanning_delegated_bypass_options** | Option<[**models::CodeSecurityCreateConfigurationRequestSecretScanningDelegatedBypassOptions**](code_security_create_configuration_request_secret_scanning_delegated_bypass_options.md)> |  | [optional]
**secret_scanning_validity_checks** | Option<**String**> | The enablement status of secret scanning validity checks | [optional][default to Disabled]
**secret_scanning_non_provider_patterns** | Option<**String**> | The enablement status of secret scanning non provider patterns | [optional][default to Disabled]
**private_vulnerability_reporting** | Option<**String**> | The enablement status of private vulnerability reporting | [optional][default to Disabled]
**enforcement** | Option<**String**> | The enforcement status for a security configuration | [optional][default to Enforced]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


