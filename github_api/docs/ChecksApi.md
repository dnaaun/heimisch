# \ChecksApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**checks_slash_create**](ChecksApi.md#checks_slash_create) | **POST** /repos/{owner}/{repo}/check-runs | Create a check run
[**checks_slash_create_suite**](ChecksApi.md#checks_slash_create_suite) | **POST** /repos/{owner}/{repo}/check-suites | Create a check suite
[**checks_slash_get**](ChecksApi.md#checks_slash_get) | **GET** /repos/{owner}/{repo}/check-runs/{check_run_id} | Get a check run
[**checks_slash_get_suite**](ChecksApi.md#checks_slash_get_suite) | **GET** /repos/{owner}/{repo}/check-suites/{check_suite_id} | Get a check suite
[**checks_slash_list_annotations**](ChecksApi.md#checks_slash_list_annotations) | **GET** /repos/{owner}/{repo}/check-runs/{check_run_id}/annotations | List check run annotations
[**checks_slash_list_for_ref**](ChecksApi.md#checks_slash_list_for_ref) | **GET** /repos/{owner}/{repo}/commits/{ref}/check-runs | List check runs for a Git reference
[**checks_slash_list_for_suite**](ChecksApi.md#checks_slash_list_for_suite) | **GET** /repos/{owner}/{repo}/check-suites/{check_suite_id}/check-runs | List check runs in a check suite
[**checks_slash_list_suites_for_ref**](ChecksApi.md#checks_slash_list_suites_for_ref) | **GET** /repos/{owner}/{repo}/commits/{ref}/check-suites | List check suites for a Git reference
[**checks_slash_rerequest_run**](ChecksApi.md#checks_slash_rerequest_run) | **POST** /repos/{owner}/{repo}/check-runs/{check_run_id}/rerequest | Rerequest a check run
[**checks_slash_rerequest_suite**](ChecksApi.md#checks_slash_rerequest_suite) | **POST** /repos/{owner}/{repo}/check-suites/{check_suite_id}/rerequest | Rerequest a check suite
[**checks_slash_set_suites_preferences**](ChecksApi.md#checks_slash_set_suites_preferences) | **PATCH** /repos/{owner}/{repo}/check-suites/preferences | Update repository preferences for check suites
[**checks_slash_update**](ChecksApi.md#checks_slash_update) | **PATCH** /repos/{owner}/{repo}/check-runs/{check_run_id} | Update a check run



## checks_slash_create

> models::CheckRun checks_slash_create(owner, repo, checks_create_request)
Create a check run

Creates a new check run for a specific commit in a repository.  To create a check run, you must use a GitHub App. OAuth apps and authenticated users are not able to create a check suite.  In a check suite, GitHub limits the number of check runs with the same name to 1000. Once these check runs exceed 1000, GitHub will start to automatically delete older check runs.  > [!NOTE] > The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**checks_create_request** | [**ChecksCreateRequest**](ChecksCreateRequest.md) |  | [required] |

### Return type

[**models::CheckRun**](check-run.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checks_slash_create_suite

> models::CheckSuite checks_slash_create_suite(owner, repo, checks_create_suite_request)
Create a check suite

Creates a check suite manually. By default, check suites are automatically created when you create a [check run](https://docs.github.com/rest/checks/runs). You only need to use this endpoint for manually creating check suites when you've disabled automatic creation using \"[Update repository preferences for check suites](https://docs.github.com/rest/checks/suites#update-repository-preferences-for-check-suites)\".  > [!NOTE] > The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.  OAuth apps and personal access tokens (classic) cannot use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**checks_create_suite_request** | [**ChecksCreateSuiteRequest**](ChecksCreateSuiteRequest.md) |  | [required] |

### Return type

[**models::CheckSuite**](check-suite.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checks_slash_get

> models::CheckRun checks_slash_get(owner, repo, check_run_id)
Get a check run

Gets a single check run using its `id`.  > [!NOTE] > The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**check_run_id** | **i32** | The unique identifier of the check run. | [required] |

### Return type

[**models::CheckRun**](check-run.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checks_slash_get_suite

> models::CheckSuite checks_slash_get_suite(owner, repo, check_suite_id)
Get a check suite

Gets a single check suite using its `id`.  > [!NOTE] > The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**check_suite_id** | **i32** | The unique identifier of the check suite. | [required] |

### Return type

[**models::CheckSuite**](check-suite.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checks_slash_list_annotations

> Vec<models::CheckAnnotation> checks_slash_list_annotations(owner, repo, check_run_id, per_page, page)
List check run annotations

Lists annotations for a check run using the annotation `id`.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**check_run_id** | **i32** | The unique identifier of the check run. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::CheckAnnotation>**](check-annotation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checks_slash_list_for_ref

> models::ChecksListForSuite200Response checks_slash_list_for_ref(owner, repo, r#ref, check_name, status, filter, per_page, page, app_id)
List check runs for a Git reference

Lists check runs for a commit ref. The `ref` can be a SHA, branch name, or a tag name.  > [!NOTE] > The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.  If there are more than 1000 check suites on a single git reference, this endpoint will limit check runs to the 1000 most recent check suites. To iterate over all possible check runs, use the [List check suites for a Git reference](https://docs.github.com/rest/reference/checks#list-check-suites-for-a-git-reference) endpoint and provide the `check_suite_id` parameter to the [List check runs in a check suite](https://docs.github.com/rest/reference/checks#list-check-runs-in-a-check-suite) endpoint.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | **String** | The commit reference. Can be a commit SHA, branch name (`heads/BRANCH_NAME`), or tag name (`tags/TAG_NAME`). For more information, see \"[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)\" in the Git documentation. | [required] |
**check_name** | Option<**String**> | Returns check runs with the specified `name`. |  |
**status** | Option<**String**> | Returns check runs with the specified `status`. |  |
**filter** | Option<**String**> | Filters check runs by their `completed_at` timestamp. `latest` returns the most recent check runs. |  |[default to latest]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**app_id** | Option<**i32**> |  |  |

### Return type

[**models::ChecksListForSuite200Response**](checks_list_for_suite_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checks_slash_list_for_suite

> models::ChecksListForSuite200Response checks_slash_list_for_suite(owner, repo, check_suite_id, check_name, status, filter, per_page, page)
List check runs in a check suite

Lists check runs for a check suite using its `id`.  > [!NOTE] > The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**check_suite_id** | **i32** | The unique identifier of the check suite. | [required] |
**check_name** | Option<**String**> | Returns check runs with the specified `name`. |  |
**status** | Option<**String**> | Returns check runs with the specified `status`. |  |
**filter** | Option<**String**> | Filters check runs by their `completed_at` timestamp. `latest` returns the most recent check runs. |  |[default to latest]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ChecksListForSuite200Response**](checks_list_for_suite_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checks_slash_list_suites_for_ref

> models::ChecksListSuitesForRef200Response checks_slash_list_suites_for_ref(owner, repo, r#ref, app_id, check_name, per_page, page)
List check suites for a Git reference

Lists check suites for a commit `ref`. The `ref` can be a SHA, branch name, or a tag name.  > [!NOTE] > The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**r#ref** | **String** | The commit reference. Can be a commit SHA, branch name (`heads/BRANCH_NAME`), or tag name (`tags/TAG_NAME`). For more information, see \"[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)\" in the Git documentation. | [required] |
**app_id** | Option<**i32**> | Filters check suites by GitHub App `id`. |  |
**check_name** | Option<**String**> | Returns check runs with the specified `name`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ChecksListSuitesForRef200Response**](checks_list_suites_for_ref_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checks_slash_rerequest_run

> serde_json::Value checks_slash_rerequest_run(owner, repo, check_run_id)
Rerequest a check run

Triggers GitHub to rerequest an existing check run, without pushing new code to a repository. This endpoint will trigger the [`check_run` webhook](https://docs.github.com/webhooks/event-payloads/#check_run) event with the action `rerequested`. When a check run is `rerequested`, its `status` is reset to `queued` and the `conclusion` is cleared.  For more information about how to re-run GitHub Actions jobs, see \"[Re-run a job from a workflow run](https://docs.github.com/rest/actions/workflow-runs#re-run-a-job-from-a-workflow-run)\".  OAuth apps and personal access tokens (classic) cannot use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**check_run_id** | **i32** | The unique identifier of the check run. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checks_slash_rerequest_suite

> serde_json::Value checks_slash_rerequest_suite(owner, repo, check_suite_id)
Rerequest a check suite

Triggers GitHub to rerequest an existing check suite, without pushing new code to a repository. This endpoint will trigger the [`check_suite` webhook](https://docs.github.com/webhooks/event-payloads/#check_suite) event with the action `rerequested`. When a check suite is `rerequested`, its `status` is reset to `queued` and the `conclusion` is cleared.  OAuth apps and personal access tokens (classic) cannot use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**check_suite_id** | **i32** | The unique identifier of the check suite. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checks_slash_set_suites_preferences

> models::CheckSuitePreference checks_slash_set_suites_preferences(owner, repo, checks_set_suites_preferences_request)
Update repository preferences for check suites

Changes the default automatic flow when creating check suites. By default, a check suite is automatically created each time code is pushed to a repository. When you disable the automatic creation of check suites, you can manually [Create a check suite](https://docs.github.com/rest/checks/suites#create-a-check-suite). You must have admin permissions in the repository to set preferences for check suites.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**checks_set_suites_preferences_request** | [**ChecksSetSuitesPreferencesRequest**](ChecksSetSuitesPreferencesRequest.md) |  | [required] |

### Return type

[**models::CheckSuitePreference**](check-suite-preference.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## checks_slash_update

> models::CheckRun checks_slash_update(owner, repo, check_run_id, checks_update_request)
Update a check run

Updates a check run for a specific commit in a repository.  > [!NOTE] > The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.  OAuth apps and personal access tokens (classic) cannot use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**check_run_id** | **i32** | The unique identifier of the check run. | [required] |
**checks_update_request** | [**ChecksUpdateRequest**](ChecksUpdateRequest.md) |  | [required] |

### Return type

[**models::CheckRun**](check-run.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

