# \CodeScanningApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**code_scanning_slash_create_variant_analysis**](CodeScanningApi.md#code_scanning_slash_create_variant_analysis) | **POST** /repos/{owner}/{repo}/code-scanning/codeql/variant-analyses | Create a CodeQL variant analysis
[**code_scanning_slash_delete_analysis**](CodeScanningApi.md#code_scanning_slash_delete_analysis) | **DELETE** /repos/{owner}/{repo}/code-scanning/analyses/{analysis_id} | Delete a code scanning analysis from a repository
[**code_scanning_slash_delete_codeql_database**](CodeScanningApi.md#code_scanning_slash_delete_codeql_database) | **DELETE** /repos/{owner}/{repo}/code-scanning/codeql/databases/{language} | Delete a CodeQL database
[**code_scanning_slash_get_alert**](CodeScanningApi.md#code_scanning_slash_get_alert) | **GET** /repos/{owner}/{repo}/code-scanning/alerts/{alert_number} | Get a code scanning alert
[**code_scanning_slash_get_analysis**](CodeScanningApi.md#code_scanning_slash_get_analysis) | **GET** /repos/{owner}/{repo}/code-scanning/analyses/{analysis_id} | Get a code scanning analysis for a repository
[**code_scanning_slash_get_codeql_database**](CodeScanningApi.md#code_scanning_slash_get_codeql_database) | **GET** /repos/{owner}/{repo}/code-scanning/codeql/databases/{language} | Get a CodeQL database for a repository
[**code_scanning_slash_get_default_setup**](CodeScanningApi.md#code_scanning_slash_get_default_setup) | **GET** /repos/{owner}/{repo}/code-scanning/default-setup | Get a code scanning default setup configuration
[**code_scanning_slash_get_sarif**](CodeScanningApi.md#code_scanning_slash_get_sarif) | **GET** /repos/{owner}/{repo}/code-scanning/sarifs/{sarif_id} | Get information about a SARIF upload
[**code_scanning_slash_get_variant_analysis**](CodeScanningApi.md#code_scanning_slash_get_variant_analysis) | **GET** /repos/{owner}/{repo}/code-scanning/codeql/variant-analyses/{codeql_variant_analysis_id} | Get the summary of a CodeQL variant analysis
[**code_scanning_slash_get_variant_analysis_repo_task**](CodeScanningApi.md#code_scanning_slash_get_variant_analysis_repo_task) | **GET** /repos/{owner}/{repo}/code-scanning/codeql/variant-analyses/{codeql_variant_analysis_id}/repos/{repo_owner}/{repo_name} | Get the analysis status of a repository in a CodeQL variant analysis
[**code_scanning_slash_list_alert_instances**](CodeScanningApi.md#code_scanning_slash_list_alert_instances) | **GET** /repos/{owner}/{repo}/code-scanning/alerts/{alert_number}/instances | List instances of a code scanning alert
[**code_scanning_slash_list_alerts_for_org**](CodeScanningApi.md#code_scanning_slash_list_alerts_for_org) | **GET** /orgs/{org}/code-scanning/alerts | List code scanning alerts for an organization
[**code_scanning_slash_list_alerts_for_repo**](CodeScanningApi.md#code_scanning_slash_list_alerts_for_repo) | **GET** /repos/{owner}/{repo}/code-scanning/alerts | List code scanning alerts for a repository
[**code_scanning_slash_list_codeql_databases**](CodeScanningApi.md#code_scanning_slash_list_codeql_databases) | **GET** /repos/{owner}/{repo}/code-scanning/codeql/databases | List CodeQL databases for a repository
[**code_scanning_slash_list_recent_analyses**](CodeScanningApi.md#code_scanning_slash_list_recent_analyses) | **GET** /repos/{owner}/{repo}/code-scanning/analyses | List code scanning analyses for a repository
[**code_scanning_slash_update_alert**](CodeScanningApi.md#code_scanning_slash_update_alert) | **PATCH** /repos/{owner}/{repo}/code-scanning/alerts/{alert_number} | Update a code scanning alert
[**code_scanning_slash_update_default_setup**](CodeScanningApi.md#code_scanning_slash_update_default_setup) | **PATCH** /repos/{owner}/{repo}/code-scanning/default-setup | Update a code scanning default setup configuration
[**code_scanning_slash_upload_sarif**](CodeScanningApi.md#code_scanning_slash_upload_sarif) | **POST** /repos/{owner}/{repo}/code-scanning/sarifs | Upload an analysis as SARIF data



## code_scanning_slash_create_variant_analysis

> models::CodeScanningVariantAnalysis code_scanning_slash_create_variant_analysis(owner, repo, code_scanning_create_variant_analysis_request)
Create a CodeQL variant analysis

Creates a new CodeQL variant analysis, which will run a CodeQL query against one or more repositories.  Get started by learning more about [running CodeQL queries at scale with Multi-Repository Variant Analysis](https://docs.github.com/code-security/codeql-for-vs-code/getting-started-with-codeql-for-vs-code/running-codeql-queries-at-scale-with-multi-repository-variant-analysis).  Use the `owner` and `repo` parameters in the URL to specify the controller repository that will be used for running GitHub Actions workflows and storing the results of the CodeQL variant analysis.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**code_scanning_create_variant_analysis_request** | Option<[**CodeScanningCreateVariantAnalysisRequest**](CodeScanningCreateVariantAnalysisRequest.md)> |  | [required] |

### Return type

[**models::CodeScanningVariantAnalysis**](code-scanning-variant-analysis.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_delete_analysis

> models::CodeScanningAnalysisDeletion code_scanning_slash_delete_analysis(owner, repo, analysis_id, confirm_delete)
Delete a code scanning analysis from a repository

Deletes a specified code scanning analysis from a repository.  You can delete one analysis at a time. To delete a series of analyses, start with the most recent analysis and work backwards. Conceptually, the process is similar to the undo function in a text editor.  When you list the analyses for a repository, one or more will be identified as deletable in the response:  ``` \"deletable\": true ```  An analysis is deletable when it's the most recent in a set of analyses. Typically, a repository will have multiple sets of analyses for each enabled code scanning tool, where a set is determined by a unique combination of analysis values:  * `ref` * `tool` * `category`  If you attempt to delete an analysis that is not the most recent in a set, you'll get a 400 response with the message:  ``` Analysis specified is not deletable. ```  The response from a successful `DELETE` operation provides you with two alternative URLs for deleting the next analysis in the set: `next_analysis_url` and `confirm_delete_url`. Use the `next_analysis_url` URL if you want to avoid accidentally deleting the final analysis in a set. This is a useful option if you want to preserve at least one analysis for the specified tool in your repository. Use the `confirm_delete_url` URL if you are content to remove all analyses for a tool. When you delete the last analysis in a set, the value of `next_analysis_url` and `confirm_delete_url` in the 200 response is `null`.  As an example of the deletion process, let's imagine that you added a workflow that configured a particular code scanning tool to analyze the code in a repository. This tool has added 15 analyses: 10 on the default branch, and another 5 on a topic branch. You therefore have two separate sets of analyses for this tool. You've now decided that you want to remove all of the analyses for the tool. To do this you must make 15 separate deletion requests. To start, you must find an analysis that's identified as deletable. Each set of analyses always has one that's identified as deletable. Having found the deletable analysis for one of the two sets, delete this analysis and then continue deleting the next analysis in the set until they're all deleted. Then repeat the process for the second set. The procedure therefore consists of a nested loop:  **Outer loop**: * List the analyses for the repository, filtered by tool. * Parse this list to find a deletable analysis. If found:    **Inner loop**:   * Delete the identified analysis.   * Parse the response for the value of `confirm_delete_url` and, if found, use this in the next iteration.  The above process assumes that you want to remove all trace of the tool's analyses from the GitHub user interface, for the specified repository, and it therefore uses the `confirm_delete_url` value. Alternatively, you could use the `next_analysis_url` value, which would leave the last analysis in each set undeleted to avoid removing a tool's analysis entirely.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**analysis_id** | **i32** | The ID of the analysis, as returned from the `GET /repos/{owner}/{repo}/code-scanning/analyses` operation. | [required] |
**confirm_delete** | Option<**String**> | Allow deletion if the specified analysis is the last in a set. If you attempt to delete the final analysis in a set without setting this parameter to `true`, you'll get a 400 response with the message: `Analysis is last of its type and deletion may result in the loss of historical alert data. Please specify confirm_delete.` |  |

### Return type

[**models::CodeScanningAnalysisDeletion**](code-scanning-analysis-deletion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_delete_codeql_database

> code_scanning_slash_delete_codeql_database(owner, repo, language)
Delete a CodeQL database

Deletes a CodeQL database for a language in a repository.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**language** | **String** | The language of the CodeQL database. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_get_alert

> models::CodeScanningAlert code_scanning_slash_get_alert(owner, repo, alert_number)
Get a code scanning alert

Gets a single code scanning alert.  OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**alert_number** | **i32** | The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation. | [required] |

### Return type

[**models::CodeScanningAlert**](code-scanning-alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_get_analysis

> models::CodeScanningAnalysis code_scanning_slash_get_analysis(owner, repo, analysis_id)
Get a code scanning analysis for a repository

Gets a specified code scanning analysis for a repository.  The default JSON response contains fields that describe the analysis. This includes the Git reference and commit SHA to which the analysis relates, the datetime of the analysis, the name of the code scanning tool, and the number of alerts.  The `rules_count` field in the default response give the number of rules that were run in the analysis. For very old analyses this data is not available, and `0` is returned in this field.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/sarif+json`**: Instead of returning a summary of the analysis, this endpoint returns a subset of the analysis data that was uploaded. The data is formatted as [SARIF version 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/cs01/sarif-v2.1.0-cs01.html). It also returns additional data such as the `github/alertNumber` and `github/alertUrl` properties.  OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**analysis_id** | **i32** | The ID of the analysis, as returned from the `GET /repos/{owner}/{repo}/code-scanning/analyses` operation. | [required] |

### Return type

[**models::CodeScanningAnalysis**](code-scanning-analysis.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json+sarif

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_get_codeql_database

> models::CodeScanningCodeqlDatabase code_scanning_slash_get_codeql_database(owner, repo, language)
Get a CodeQL database for a repository

Gets a CodeQL database for a language in a repository.  By default this endpoint returns JSON metadata about the CodeQL database. To download the CodeQL database binary content, set the `Accept` header of the request to [`application/zip`](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types), and make sure your HTTP client is configured to follow redirects or use the `Location` header to make a second request to get the redirect URL.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**language** | **String** | The language of the CodeQL database. | [required] |

### Return type

[**models::CodeScanningCodeqlDatabase**](code-scanning-codeql-database.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_get_default_setup

> models::CodeScanningDefaultSetup code_scanning_slash_get_default_setup(owner, repo)
Get a code scanning default setup configuration

Gets a code scanning default setup configuration.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::CodeScanningDefaultSetup**](code-scanning-default-setup.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_get_sarif

> models::CodeScanningSarifsStatus code_scanning_slash_get_sarif(owner, repo, sarif_id)
Get information about a SARIF upload

Gets information about a SARIF upload, including the status and the URL of the analysis that was uploaded so that you can retrieve details of the analysis. For more information, see \"[Get a code scanning analysis for a repository](/rest/code-scanning/code-scanning#get-a-code-scanning-analysis-for-a-repository).\" OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**sarif_id** | **String** | The SARIF ID obtained after uploading. | [required] |

### Return type

[**models::CodeScanningSarifsStatus**](code-scanning-sarifs-status.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_get_variant_analysis

> models::CodeScanningVariantAnalysis code_scanning_slash_get_variant_analysis(owner, repo, codeql_variant_analysis_id)
Get the summary of a CodeQL variant analysis

Gets the summary of a CodeQL variant analysis.  OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**codeql_variant_analysis_id** | **i32** | The unique identifier of the variant analysis. | [required] |

### Return type

[**models::CodeScanningVariantAnalysis**](code-scanning-variant-analysis.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_get_variant_analysis_repo_task

> models::CodeScanningVariantAnalysisRepoTask code_scanning_slash_get_variant_analysis_repo_task(owner, repo, codeql_variant_analysis_id, repo_owner, repo_name)
Get the analysis status of a repository in a CodeQL variant analysis

Gets the analysis status of a repository in a CodeQL variant analysis.  OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the controller repository. | [required] |
**codeql_variant_analysis_id** | **i32** | The ID of the variant analysis. | [required] |
**repo_owner** | **String** | The account owner of the variant analysis repository. The name is not case sensitive. | [required] |
**repo_name** | **String** | The name of the variant analysis repository. | [required] |

### Return type

[**models::CodeScanningVariantAnalysisRepoTask**](code-scanning-variant-analysis-repo-task.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_list_alert_instances

> Vec<models::CodeScanningAlertInstance> code_scanning_slash_list_alert_instances(owner, repo, alert_number, page, per_page, r#ref, pr)
List instances of a code scanning alert

Lists all instances of the specified code scanning alert.  OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**alert_number** | **i32** | The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**r#ref** | Option<**String**> | The Git reference for the results you want to list. The `ref` for a branch can be formatted either as `refs/heads/<branch name>` or simply `<branch name>`. To reference a pull request use `refs/pull/<number>/merge`. |  |
**pr** | Option<**i32**> | The number of the pull request for the results you want to list. |  |

### Return type

[**Vec<models::CodeScanningAlertInstance>**](code-scanning-alert-instance.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_list_alerts_for_org

> Vec<models::CodeScanningOrganizationAlertItems> code_scanning_slash_list_alerts_for_org(org, tool_name, tool_guid, before, after, page, per_page, direction, state, sort, severity)
List code scanning alerts for an organization

Lists code scanning alerts for the default branch for all eligible repositories in an organization. Eligible repositories are repositories that are owned by organizations that you own or for which you are a security manager. For more information, see \"[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization).\"  The authenticated user must be an owner or security manager for the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `security_events` or `repo`s cope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**tool_name** | Option<**String**> | The name of a code scanning tool. Only results by this tool will be listed. You can specify the tool by using either `tool_name` or `tool_guid`, but not both. |  |
**tool_guid** | Option<**String**> | The GUID of a code scanning tool. Only results by this tool will be listed. Note that some code scanning tools may not include a GUID in their analysis data. You can specify the tool by using either `tool_guid` or `tool_name`, but not both. |  |
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**state** | Option<[**CodeScanningAlertStateQuery**](.md)> | If specified, only code scanning alerts with this state will be returned. |  |
**sort** | Option<**String**> | The property by which to sort the results. |  |[default to created]
**severity** | Option<[**CodeScanningAlertSeverity**](.md)> | If specified, only code scanning alerts with this severity will be returned. |  |

### Return type

[**Vec<models::CodeScanningOrganizationAlertItems>**](code-scanning-organization-alert-items.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_list_alerts_for_repo

> Vec<models::CodeScanningAlertItems> code_scanning_slash_list_alerts_for_repo(owner, repo, tool_name, tool_guid, page, per_page, r#ref, pr, direction, before, after, sort, state, severity)
List code scanning alerts for a repository

Lists code scanning alerts.  The response includes a `most_recent_instance` object. This provides details of the most recent instance of this alert for the default branch (or for the specified Git reference if you used `ref` in the request).  OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**tool_name** | Option<**String**> | The name of a code scanning tool. Only results by this tool will be listed. You can specify the tool by using either `tool_name` or `tool_guid`, but not both. |  |
**tool_guid** | Option<**String**> | The GUID of a code scanning tool. Only results by this tool will be listed. Note that some code scanning tools may not include a GUID in their analysis data. You can specify the tool by using either `tool_guid` or `tool_name`, but not both. |  |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**r#ref** | Option<**String**> | The Git reference for the results you want to list. The `ref` for a branch can be formatted either as `refs/heads/<branch name>` or simply `<branch name>`. To reference a pull request use `refs/pull/<number>/merge`. |  |
**pr** | Option<**i32**> | The number of the pull request for the results you want to list. |  |
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**sort** | Option<**String**> | The property by which to sort the results. |  |[default to created]
**state** | Option<[**CodeScanningAlertStateQuery**](.md)> | If specified, only code scanning alerts with this state will be returned. |  |
**severity** | Option<[**CodeScanningAlertSeverity**](.md)> | If specified, only code scanning alerts with this severity will be returned. |  |

### Return type

[**Vec<models::CodeScanningAlertItems>**](code-scanning-alert-items.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_list_codeql_databases

> Vec<models::CodeScanningCodeqlDatabase> code_scanning_slash_list_codeql_databases(owner, repo)
List CodeQL databases for a repository

Lists the CodeQL databases that are available in a repository.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::CodeScanningCodeqlDatabase>**](code-scanning-codeql-database.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_list_recent_analyses

> Vec<models::CodeScanningAnalysis> code_scanning_slash_list_recent_analyses(owner, repo, tool_name, tool_guid, page, per_page, pr, r#ref, sarif_id, direction, sort)
List code scanning analyses for a repository

Lists the details of all code scanning analyses for a repository, starting with the most recent. The response is paginated and you can use the `page` and `per_page` parameters to list the analyses you're interested in. By default 30 analyses are listed per page.  The `rules_count` field in the response give the number of rules that were run in the analysis. For very old analyses this data is not available, and `0` is returned in this field.  > [!WARNING] > **Closing down notice:** The `tool_name` field is closing down and will, in future, not be included in the response for this endpoint. The example response reflects this change. The tool name can now be found inside the `tool` field.  OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**tool_name** | Option<**String**> | The name of a code scanning tool. Only results by this tool will be listed. You can specify the tool by using either `tool_name` or `tool_guid`, but not both. |  |
**tool_guid** | Option<**String**> | The GUID of a code scanning tool. Only results by this tool will be listed. Note that some code scanning tools may not include a GUID in their analysis data. You can specify the tool by using either `tool_guid` or `tool_name`, but not both. |  |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**pr** | Option<**i32**> | The number of the pull request for the results you want to list. |  |
**r#ref** | Option<**String**> | The Git reference for the analyses you want to list. The `ref` for a branch can be formatted either as `refs/heads/<branch name>` or simply `<branch name>`. To reference a pull request use `refs/pull/<number>/merge`. |  |
**sarif_id** | Option<**String**> | Filter analyses belonging to the same SARIF upload. |  |
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**sort** | Option<**String**> | The property by which to sort the results. |  |[default to created]

### Return type

[**Vec<models::CodeScanningAnalysis>**](code-scanning-analysis.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_update_alert

> models::CodeScanningAlert code_scanning_slash_update_alert(owner, repo, alert_number, code_scanning_update_alert_request)
Update a code scanning alert

Updates the status of a single code scanning alert. OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**alert_number** | **i32** | The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation. | [required] |
**code_scanning_update_alert_request** | [**CodeScanningUpdateAlertRequest**](CodeScanningUpdateAlertRequest.md) |  | [required] |

### Return type

[**models::CodeScanningAlert**](code-scanning-alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_update_default_setup

> serde_json::Value code_scanning_slash_update_default_setup(owner, repo, code_scanning_default_setup_update)
Update a code scanning default setup configuration

Updates a code scanning default setup configuration.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**code_scanning_default_setup_update** | [**CodeScanningDefaultSetupUpdate**](CodeScanningDefaultSetupUpdate.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## code_scanning_slash_upload_sarif

> models::CodeScanningSarifsReceipt code_scanning_slash_upload_sarif(owner, repo, code_scanning_upload_sarif_request)
Upload an analysis as SARIF data

Uploads SARIF data containing the results of a code scanning analysis to make the results available in a repository. For troubleshooting information, see \"[Troubleshooting SARIF uploads](https://docs.github.com/code-security/code-scanning/troubleshooting-sarif).\"  There are two places where you can upload code scanning results.  - If you upload to a pull request, for example `--ref refs/pull/42/merge` or `--ref refs/pull/42/head`, then the results appear as alerts in a pull request check. For more information, see \"[Triaging code scanning alerts in pull requests](/code-security/secure-coding/triaging-code-scanning-alerts-in-pull-requests).\"  - If you upload to a branch, for example `--ref refs/heads/my-branch`, then the results appear in the **Security** tab for your repository. For more information, see \"[Managing code scanning alerts for your repository](/code-security/secure-coding/managing-code-scanning-alerts-for-your-repository#viewing-the-alerts-for-a-repository).\"  You must compress the SARIF-formatted analysis data that you want to upload, using `gzip`, and then encode it as a Base64 format string. For example:  ``` gzip -c analysis-data.sarif | base64 -w0 ```  SARIF upload supports a maximum number of entries per the following data objects, and an analysis will be rejected if any of these objects is above its maximum value. For some objects, there are additional values over which the entries will be ignored while keeping the most important entries whenever applicable. To get the most out of your analysis when it includes data above the supported limits, try to optimize the analysis configuration. For example, for the CodeQL tool, identify and remove the most noisy queries. For more information, see \"[SARIF results exceed one or more limits](https://docs.github.com/code-security/code-scanning/troubleshooting-sarif/results-exceed-limit).\"   | **SARIF data**                   | **Maximum values** | **Additional limits**                                                            | |----------------------------------|:------------------:|----------------------------------------------------------------------------------| | Runs per file                    |         20         |                                                                                  | | Results per run                  |       25,000       | Only the top 5,000 results will be included, prioritized by severity.            | | Rules per run                    |       25,000       |                                                                                  | | Tool extensions per run          |        100         |                                                                                  | | Thread Flow Locations per result |       10,000       | Only the top 1,000 Thread Flow Locations will be included, using prioritization. | | Location per result              |       1,000        | Only 100 locations will be included.                                             | | Tags per rule                    |         20         | Only 10 tags will be included.                                                   |   The `202 Accepted` response includes an `id` value. You can use this ID to check the status of the upload by using it in the `/sarifs/{sarif_id}` endpoint. For more information, see \"[Get information about a SARIF upload](/rest/code-scanning/code-scanning#get-information-about-a-sarif-upload).\"  OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.  This endpoint is limited to 1,000 requests per hour for each user or app installation calling it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**code_scanning_upload_sarif_request** | [**CodeScanningUploadSarifRequest**](CodeScanningUploadSarifRequest.md) |  | [required] |

### Return type

[**models::CodeScanningSarifsReceipt**](code-scanning-sarifs-receipt.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

