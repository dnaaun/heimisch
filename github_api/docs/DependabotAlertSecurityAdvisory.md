# DependabotAlertSecurityAdvisory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ghsa_id** | **String** | The unique GitHub Security Advisory ID assigned to the advisory. | [readonly]
**cve_id** | Option<**String**> | The unique CVE ID assigned to the advisory. | [readonly]
**summary** | **String** | A short, plain text summary of the advisory. | [readonly]
**description** | **String** | A long-form Markdown-supported description of the advisory. | [readonly]
**vulnerabilities** | [**Vec<models::DependabotAlertSecurityVulnerability>**](dependabot-alert-security-vulnerability.md) | Vulnerable version range information for the advisory. | [readonly]
**severity** | **String** | The severity of the advisory. | [readonly]
**cvss** | [**models::DependabotAlertSecurityAdvisoryCvss**](dependabot_alert_security_advisory_cvss.md) |  | 
**cvss_severities** | Option<[**models::CvssSeverities**](cvss-severities.md)> |  | [optional]
**cwes** | [**Vec<models::DependabotAlertSecurityAdvisoryCwesInner>**](dependabot_alert_security_advisory_cwes_inner.md) | Details for the advisory pertaining to Common Weakness Enumeration. | [readonly]
**identifiers** | [**Vec<models::DependabotAlertSecurityAdvisoryIdentifiersInner>**](dependabot_alert_security_advisory_identifiers_inner.md) | Values that identify this advisory among security information sources. | [readonly]
**references** | [**Vec<models::DependabotAlertSecurityAdvisoryReferencesInner>**](dependabot_alert_security_advisory_references_inner.md) | Links to additional advisory information. | [readonly]
**published_at** | **String** | The time that the advisory was published in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [readonly]
**updated_at** | **String** | The time that the advisory was last modified in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [readonly]
**withdrawn_at** | Option<**String**> | The time that the advisory was withdrawn in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


