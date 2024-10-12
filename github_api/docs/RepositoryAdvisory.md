# RepositoryAdvisory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ghsa_id** | **String** | The GitHub Security Advisory ID. | [readonly]
**cve_id** | Option<**String**> | The Common Vulnerabilities and Exposures (CVE) ID. | 
**url** | **String** | The API URL for the advisory. | [readonly]
**html_url** | **String** | The URL for the advisory. | [readonly]
**summary** | **String** | A short summary of the advisory. | 
**description** | Option<**String**> | A detailed description of what the advisory entails. | 
**severity** | Option<**String**> | The severity of the advisory. | 
**author** | Option<[**models::SimpleUser**](simple-user.md)> | The author of the advisory. | [readonly]
**publisher** | Option<[**models::SimpleUser**](simple-user.md)> | The publisher of the advisory. | [readonly]
**identifiers** | [**Vec<models::GlobalAdvisoryIdentifiersInner>**](global_advisory_identifiers_inner.md) |  | [readonly]
**state** | **String** | The state of the advisory. | 
**created_at** | Option<**String**> | The date and time of when the advisory was created, in ISO 8601 format. | [readonly]
**updated_at** | Option<**String**> | The date and time of when the advisory was last updated, in ISO 8601 format. | [readonly]
**published_at** | Option<**String**> | The date and time of when the advisory was published, in ISO 8601 format. | [readonly]
**closed_at** | Option<**String**> | The date and time of when the advisory was closed, in ISO 8601 format. | [readonly]
**withdrawn_at** | Option<**String**> | The date and time of when the advisory was withdrawn, in ISO 8601 format. | [readonly]
**submission** | Option<[**models::RepositoryAdvisorySubmission**](repository_advisory_submission.md)> |  | 
**vulnerabilities** | Option<[**Vec<models::RepositoryAdvisoryVulnerability>**](repository-advisory-vulnerability.md)> |  | 
**cvss** | Option<[**models::GlobalAdvisoryCvss**](global_advisory_cvss.md)> |  | 
**cvss_severities** | Option<[**models::CvssSeverities**](cvss-severities.md)> |  | [optional]
**cwes** | Option<[**Vec<models::GlobalAdvisoryCwesInner>**](global_advisory_cwes_inner.md)> |  | [readonly]
**cwe_ids** | Option<**Vec<String>**> | A list of only the CWE IDs. | 
**credits** | Option<[**Vec<models::RepositoryAdvisoryCreditsInner>**](repository_advisory_credits_inner.md)> |  | 
**credits_detailed** | Option<[**Vec<models::RepositoryAdvisoryCredit>**](repository-advisory-credit.md)> |  | [readonly]
**collaborating_users** | Option<[**Vec<models::SimpleUser>**](simple-user.md)> | A list of users that collaborate on the advisory. | 
**collaborating_teams** | Option<[**Vec<models::Team>**](team.md)> | A list of teams that collaborate on the advisory. | 
**private_fork** | Option<[**models::SimpleRepository**](simple-repository.md)> | A temporary private fork of the advisory's repository for collaborating on a fix. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


