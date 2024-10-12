# RepositoryAdvisoryCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**summary** | **String** | A short summary of the advisory. | 
**description** | **String** | A detailed description of what the advisory impacts. | 
**cve_id** | Option<**String**> | The Common Vulnerabilities and Exposures (CVE) ID. | [optional]
**vulnerabilities** | [**Vec<models::RepositoryAdvisoryCreateVulnerabilitiesInner>**](repository_advisory_create_vulnerabilities_inner.md) | A product affected by the vulnerability detailed in a repository security advisory. | 
**cwe_ids** | Option<**Vec<String>**> | A list of Common Weakness Enumeration (CWE) IDs. | [optional]
**credits** | Option<[**Vec<models::RepositoryAdvisoryCreateCreditsInner>**](repository_advisory_create_credits_inner.md)> | A list of users receiving credit for their participation in the security advisory. | [optional]
**severity** | Option<**String**> | The severity of the advisory. You must choose between setting this field or `cvss_vector_string`. | [optional]
**cvss_vector_string** | Option<**String**> | The CVSS vector that calculates the severity of the advisory. You must choose between setting this field or `severity`. | [optional]
**start_private_fork** | Option<**bool**> | Whether to create a temporary private fork of the repository to collaborate on a fix. | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


