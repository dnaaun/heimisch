# \SecurityAdvisoriesApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**security_advisories_slash_create_fork**](SecurityAdvisoriesApi.md#security_advisories_slash_create_fork) | **POST** /repos/{owner}/{repo}/security-advisories/{ghsa_id}/forks | Create a temporary private fork
[**security_advisories_slash_create_private_vulnerability_report**](SecurityAdvisoriesApi.md#security_advisories_slash_create_private_vulnerability_report) | **POST** /repos/{owner}/{repo}/security-advisories/reports | Privately report a security vulnerability
[**security_advisories_slash_create_repository_advisory**](SecurityAdvisoriesApi.md#security_advisories_slash_create_repository_advisory) | **POST** /repos/{owner}/{repo}/security-advisories | Create a repository security advisory
[**security_advisories_slash_create_repository_advisory_cve_request**](SecurityAdvisoriesApi.md#security_advisories_slash_create_repository_advisory_cve_request) | **POST** /repos/{owner}/{repo}/security-advisories/{ghsa_id}/cve | Request a CVE for a repository security advisory
[**security_advisories_slash_get_global_advisory**](SecurityAdvisoriesApi.md#security_advisories_slash_get_global_advisory) | **GET** /advisories/{ghsa_id} | Get a global security advisory
[**security_advisories_slash_get_repository_advisory**](SecurityAdvisoriesApi.md#security_advisories_slash_get_repository_advisory) | **GET** /repos/{owner}/{repo}/security-advisories/{ghsa_id} | Get a repository security advisory
[**security_advisories_slash_list_global_advisories**](SecurityAdvisoriesApi.md#security_advisories_slash_list_global_advisories) | **GET** /advisories | List global security advisories
[**security_advisories_slash_list_org_repository_advisories**](SecurityAdvisoriesApi.md#security_advisories_slash_list_org_repository_advisories) | **GET** /orgs/{org}/security-advisories | List repository security advisories for an organization
[**security_advisories_slash_list_repository_advisories**](SecurityAdvisoriesApi.md#security_advisories_slash_list_repository_advisories) | **GET** /repos/{owner}/{repo}/security-advisories | List repository security advisories
[**security_advisories_slash_update_repository_advisory**](SecurityAdvisoriesApi.md#security_advisories_slash_update_repository_advisory) | **PATCH** /repos/{owner}/{repo}/security-advisories/{ghsa_id} | Update a repository security advisory



## security_advisories_slash_create_fork

> models::FullRepository security_advisories_slash_create_fork(owner, repo, ghsa_id)
Create a temporary private fork

Create a temporary private fork to collaborate on fixing a security vulnerability in your repository.  > [!NOTE] > Forking a repository happens asynchronously. You may have to wait up to 5 minutes before you can access the fork.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**ghsa_id** | **String** | The GHSA (GitHub Security Advisory) identifier of the advisory. | [required] |

### Return type

[**models::FullRepository**](full-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_advisories_slash_create_private_vulnerability_report

> models::RepositoryAdvisory security_advisories_slash_create_private_vulnerability_report(owner, repo, private_vulnerability_report_create)
Privately report a security vulnerability

Report a security vulnerability to the maintainers of the repository. See \"[Privately reporting a security vulnerability](https://docs.github.com/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability)\" for more information about private vulnerability reporting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**private_vulnerability_report_create** | [**PrivateVulnerabilityReportCreate**](PrivateVulnerabilityReportCreate.md) |  | [required] |

### Return type

[**models::RepositoryAdvisory**](repository-advisory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_advisories_slash_create_repository_advisory

> models::RepositoryAdvisory security_advisories_slash_create_repository_advisory(owner, repo, repository_advisory_create)
Create a repository security advisory

Creates a new repository security advisory.  In order to create a draft repository security advisory, the authenticated user must be a security manager or administrator of that repository.  OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**repository_advisory_create** | [**RepositoryAdvisoryCreate**](RepositoryAdvisoryCreate.md) |  | [required] |

### Return type

[**models::RepositoryAdvisory**](repository-advisory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_advisories_slash_create_repository_advisory_cve_request

> serde_json::Value security_advisories_slash_create_repository_advisory_cve_request(owner, repo, ghsa_id)
Request a CVE for a repository security advisory

If you want a CVE identification number for the security vulnerability in your project, and don't already have one, you can request a CVE identification number from GitHub. For more information see \"[Requesting a CVE identification number](https://docs.github.com/code-security/security-advisories/repository-security-advisories/publishing-a-repository-security-advisory#requesting-a-cve-identification-number-optional).\"  You may request a CVE for public repositories, but cannot do so for private repositories.  In order to request a CVE for a repository security advisory, the authenticated user must be a security manager or administrator of that repository.  OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**ghsa_id** | **String** | The GHSA (GitHub Security Advisory) identifier of the advisory. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_advisories_slash_get_global_advisory

> models::GlobalAdvisory security_advisories_slash_get_global_advisory(ghsa_id)
Get a global security advisory

Gets a global security advisory using its GitHub Security Advisory (GHSA) identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ghsa_id** | **String** | The GHSA (GitHub Security Advisory) identifier of the advisory. | [required] |

### Return type

[**models::GlobalAdvisory**](global-advisory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_advisories_slash_get_repository_advisory

> models::RepositoryAdvisory security_advisories_slash_get_repository_advisory(owner, repo, ghsa_id)
Get a repository security advisory

Get a repository security advisory using its GitHub Security Advisory (GHSA) identifier.  Anyone can access any published security advisory on a public repository.  The authenticated user can access an unpublished security advisory from a repository if they are a security manager or administrator of that repository, or if they are a collaborator on the security advisory.  OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:read` scope to to get a published security advisory in a private repository, or any unpublished security advisory that the authenticated user has access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**ghsa_id** | **String** | The GHSA (GitHub Security Advisory) identifier of the advisory. | [required] |

### Return type

[**models::RepositoryAdvisory**](repository-advisory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_advisories_slash_list_global_advisories

> Vec<models::GlobalAdvisory> security_advisories_slash_list_global_advisories(ghsa_id, r#type, cve_id, ecosystem, severity, cwes, is_withdrawn, affects, published, updated, modified, epss_percentage, epss_percentile, before, after, direction, per_page, sort)
List global security advisories

Lists all global security advisories that match the specified parameters. If no other parameters are defined, the request will return only GitHub-reviewed advisories that are not malware.  By default, all responses will exclude advisories for malware, because malware are not standard vulnerabilities. To list advisories for malware, you must include the `type` parameter in your request, with the value `malware`. For more information about the different types of security advisories, see \"[About the GitHub Advisory database](https://docs.github.com/code-security/security-advisories/global-security-advisories/about-the-github-advisory-database#about-types-of-security-advisories).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ghsa_id** | Option<**String**> | If specified, only advisories with this GHSA (GitHub Security Advisory) identifier will be returned. |  |
**r#type** | Option<**String**> | If specified, only advisories of this type will be returned. By default, a request with no other parameters defined will only return reviewed advisories that are not malware. |  |[default to reviewed]
**cve_id** | Option<**String**> | If specified, only advisories with this CVE (Common Vulnerabilities and Exposures) identifier will be returned. |  |
**ecosystem** | Option<[**SecurityAdvisoryEcosystems**](.md)> | If specified, only advisories for these ecosystems will be returned. |  |
**severity** | Option<**String**> | If specified, only advisories with these severities will be returned. |  |
**cwes** | Option<[**SecurityAdvisoriesListGlobalAdvisoriesCwesParameter**](.md)> | If specified, only advisories with these Common Weakness Enumerations (CWEs) will be returned.  Example: `cwes=79,284,22` or `cwes[]=79&cwes[]=284&cwes[]=22` |  |
**is_withdrawn** | Option<**bool**> | Whether to only return advisories that have been withdrawn. |  |
**affects** | Option<[**SecurityAdvisoriesListGlobalAdvisoriesAffectsParameter**](.md)> | If specified, only return advisories that affect any of `package` or `package@version`. A maximum of 1000 packages can be specified. If the query parameter causes the URL to exceed the maximum URL length supported by your client, you must specify fewer packages.  Example: `affects=package1,package2@1.0.0,package3@^2.0.0` or `affects[]=package1&affects[]=package2@1.0.0` |  |
**published** | Option<**String**> | If specified, only return advisories that were published on a date or date range.  For more information on the syntax of the date range, see \"[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates).\" |  |
**updated** | Option<**String**> | If specified, only return advisories that were updated on a date or date range.  For more information on the syntax of the date range, see \"[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates).\" |  |
**modified** | Option<**String**> | If specified, only show advisories that were updated or published on a date or date range.  For more information on the syntax of the date range, see \"[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates).\" |  |
**epss_percentage** | Option<**String**> | If specified, only return advisories that have an EPSS percentage score that matches the provided value. The EPSS percentage represents the likelihood of a CVE being exploited. |  |
**epss_percentile** | Option<**String**> | If specified, only return advisories that have an EPSS percentile score that matches the provided value. The EPSS percentile represents the relative rank of the CVE's likelihood of being exploited compared to other CVEs. |  |
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**sort** | Option<**String**> | The property to sort the results by. |  |[default to published]

### Return type

[**Vec<models::GlobalAdvisory>**](global-advisory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_advisories_slash_list_org_repository_advisories

> Vec<models::RepositoryAdvisory> security_advisories_slash_list_org_repository_advisories(org, direction, sort, before, after, per_page, state)
List repository security advisories for an organization

Lists repository security advisories for an organization.  The authenticated user must be an owner or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**sort** | Option<**String**> | The property to sort the results by. |  |[default to created]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**per_page** | Option<**i32**> | The number of advisories to return per page. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**state** | Option<**String**> | Filter by the state of the repository advisories. Only advisories of this state will be returned. |  |

### Return type

[**Vec<models::RepositoryAdvisory>**](repository-advisory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_advisories_slash_list_repository_advisories

> Vec<models::RepositoryAdvisory> security_advisories_slash_list_repository_advisories(owner, repo, direction, sort, before, after, per_page, state)
List repository security advisories

Lists security advisories in a repository.  The authenticated user can access unpublished security advisories from a repository if they are a security manager or administrator of that repository, or if they are a collaborator on any security advisory.  OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:read` scope to to get a published security advisory in a private repository, or any unpublished security advisory that the authenticated user has access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**sort** | Option<**String**> | The property to sort the results by. |  |[default to created]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**per_page** | Option<**i32**> | The number of advisories to return per page. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**state** | Option<**String**> | Filter by state of the repository advisories. Only advisories of this state will be returned. |  |

### Return type

[**Vec<models::RepositoryAdvisory>**](repository-advisory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_advisories_slash_update_repository_advisory

> models::RepositoryAdvisory security_advisories_slash_update_repository_advisory(owner, repo, ghsa_id, repository_advisory_update)
Update a repository security advisory

Update a repository security advisory using its GitHub Security Advisory (GHSA) identifier.  In order to update any security advisory, the authenticated user must be a security manager or administrator of that repository, or a collaborator on the repository security advisory.  OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**ghsa_id** | **String** | The GHSA (GitHub Security Advisory) identifier of the advisory. | [required] |
**repository_advisory_update** | [**RepositoryAdvisoryUpdate**](RepositoryAdvisoryUpdate.md) |  | [required] |

### Return type

[**models::RepositoryAdvisory**](repository-advisory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

