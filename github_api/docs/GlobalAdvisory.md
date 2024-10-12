# GlobalAdvisory

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ghsa_id** | **String** | The GitHub Security Advisory ID. | [readonly]
**cve_id** | Option<**String**> | The Common Vulnerabilities and Exposures (CVE) ID. | [readonly]
**url** | **String** | The API URL for the advisory. | [readonly]
**html_url** | **String** | The URL for the advisory. | [readonly]
**repository_advisory_url** | Option<**String**> | The API URL for the repository advisory. | [readonly]
**summary** | **String** | A short summary of the advisory. | 
**description** | Option<**String**> | A detailed description of what the advisory entails. | 
**r#type** | **String** | The type of advisory. | [readonly]
**severity** | **String** | The severity of the advisory. | 
**source_code_location** | Option<**String**> | The URL of the advisory's source code. | 
**identifiers** | Option<[**Vec<models::GlobalAdvisoryIdentifiersInner>**](global_advisory_identifiers_inner.md)> |  | [readonly]
**references** | Option<**Vec<String>**> |  | 
**published_at** | **String** | The date and time of when the advisory was published, in ISO 8601 format. | [readonly]
**updated_at** | **String** | The date and time of when the advisory was last updated, in ISO 8601 format. | [readonly]
**github_reviewed_at** | Option<**String**> | The date and time of when the advisory was reviewed by GitHub, in ISO 8601 format. | [readonly]
**nvd_published_at** | Option<**String**> | The date and time when the advisory was published in the National Vulnerability Database, in ISO 8601 format. This field is only populated when the advisory is imported from the National Vulnerability Database. | [readonly]
**withdrawn_at** | Option<**String**> | The date and time of when the advisory was withdrawn, in ISO 8601 format. | [readonly]
**vulnerabilities** | Option<[**Vec<models::Vulnerability>**](vulnerability.md)> | The products and respective version ranges affected by the advisory. | 
**cvss** | Option<[**models::GlobalAdvisoryCvss**](global_advisory_cvss.md)> |  | 
**cvss_severities** | Option<[**models::CvssSeverities**](cvss-severities.md)> |  | [optional]
**cwes** | Option<[**Vec<models::GlobalAdvisoryCwesInner>**](global_advisory_cwes_inner.md)> |  | 
**epss** | Option<[**models::GlobalAdvisoryEpss**](global_advisory_epss.md)> |  | [optional]
**credits** | Option<[**Vec<models::GlobalAdvisoryCreditsInner>**](global_advisory_credits_inner.md)> | The users who contributed to the advisory. | [readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


