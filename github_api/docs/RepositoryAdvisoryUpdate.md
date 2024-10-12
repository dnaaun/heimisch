# RepositoryAdvisoryUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**summary** | Option<**String**> | A short summary of the advisory. | [optional]
**description** | Option<**String**> | A detailed description of what the advisory impacts. | [optional]
**cve_id** | Option<**String**> | The Common Vulnerabilities and Exposures (CVE) ID. | [optional]
**vulnerabilities** | Option<[**Vec<models::RepositoryAdvisoryCreateVulnerabilitiesInner>**](repository_advisory_create_vulnerabilities_inner.md)> | A product affected by the vulnerability detailed in a repository security advisory. | [optional]
**cwe_ids** | Option<**Vec<String>**> | A list of Common Weakness Enumeration (CWE) IDs. | [optional]
**credits** | Option<[**Vec<models::RepositoryAdvisoryCreateCreditsInner>**](repository_advisory_create_credits_inner.md)> | A list of users receiving credit for their participation in the security advisory. | [optional]
**severity** | Option<**String**> | The severity of the advisory. You must choose between setting this field or `cvss_vector_string`. | [optional]
**cvss_vector_string** | Option<**String**> | The CVSS vector that calculates the severity of the advisory. You must choose between setting this field or `severity`. | [optional]
**state** | Option<**String**> | The state of the advisory. | [optional]
**collaborating_users** | Option<**Vec<String>**> | A list of usernames who have been granted write access to the advisory. | [optional]
**collaborating_teams** | Option<**Vec<String>**> | A list of team slugs which have been granted write access to the advisory. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


