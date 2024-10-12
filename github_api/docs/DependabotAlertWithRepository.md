# DependabotAlertWithRepository

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**number** | **i32** | The security alert number. | [readonly]
**state** | **String** | The state of the Dependabot alert. | [readonly]
**dependency** | [**models::DependabotAlertWithRepositoryDependency**](dependabot_alert_with_repository_dependency.md) |  | 
**security_advisory** | [**models::DependabotAlertSecurityAdvisory**](dependabot-alert-security-advisory.md) |  | 
**security_vulnerability** | [**models::DependabotAlertSecurityVulnerability**](dependabot-alert-security-vulnerability.md) |  | 
**url** | **String** | The REST API URL of the alert resource. | [readonly]
**html_url** | **String** | The GitHub URL of the alert resource. | [readonly]
**created_at** | **String** | The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [readonly]
**updated_at** | **String** | The time that the alert was last updated in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [readonly]
**dismissed_at** | Option<**String**> | The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [readonly]
**dismissed_by** | Option<[**models::NullableSimpleUser**](nullable-simple-user.md)> |  | 
**dismissed_reason** | Option<**String**> | The reason that the alert was dismissed. | 
**dismissed_comment** | Option<**String**> | An optional comment associated with the alert's dismissal. | 
**fixed_at** | Option<**String**> | The time that the alert was no longer detected and was considered fixed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [readonly]
**auto_dismissed_at** | Option<**String**> | The time that the alert was auto-dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [optional][readonly]
**repository** | [**models::SimpleRepository**](simple-repository.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


