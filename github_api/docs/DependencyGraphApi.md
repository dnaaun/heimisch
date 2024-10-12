# \DependencyGraphApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dependency_graph_slash_create_repository_snapshot**](DependencyGraphApi.md#dependency_graph_slash_create_repository_snapshot) | **POST** /repos/{owner}/{repo}/dependency-graph/snapshots | Create a snapshot of dependencies for a repository
[**dependency_graph_slash_diff_range**](DependencyGraphApi.md#dependency_graph_slash_diff_range) | **GET** /repos/{owner}/{repo}/dependency-graph/compare/{basehead} | Get a diff of the dependencies between commits
[**dependency_graph_slash_export_sbom**](DependencyGraphApi.md#dependency_graph_slash_export_sbom) | **GET** /repos/{owner}/{repo}/dependency-graph/sbom | Export a software bill of materials (SBOM) for a repository.



## dependency_graph_slash_create_repository_snapshot

> models::DependencyGraphCreateRepositorySnapshot201Response dependency_graph_slash_create_repository_snapshot(owner, repo, snapshot)
Create a snapshot of dependencies for a repository

Create a new snapshot of a repository's dependencies.  The authenticated user must have access to the repository.  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**snapshot** | [**Snapshot**](Snapshot.md) |  | [required] |

### Return type

[**models::DependencyGraphCreateRepositorySnapshot201Response**](dependency_graph_create_repository_snapshot_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependency_graph_slash_diff_range

> Vec<models::DependencyGraphDiffInner> dependency_graph_slash_diff_range(owner, repo, basehead, name)
Get a diff of the dependencies between commits

Gets the diff of the dependency changes between two commits of a repository, based on the changes to the dependency manifests made in those commits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**basehead** | **String** | The base and head Git revisions to compare. The Git revisions will be resolved to commit SHAs. Named revisions will be resolved to their corresponding HEAD commits, and an appropriate merge base will be determined. This parameter expects the format `{base}...{head}`. | [required] |
**name** | Option<**String**> | The full path, relative to the repository root, of the dependency manifest file. |  |

### Return type

[**Vec<models::DependencyGraphDiffInner>**](dependency_graph_diff_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dependency_graph_slash_export_sbom

> models::DependencyGraphSpdxSbom dependency_graph_slash_export_sbom(owner, repo)
Export a software bill of materials (SBOM) for a repository.

Exports the software bill of materials (SBOM) for a repository in SPDX JSON format.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::DependencyGraphSpdxSbom**](dependency-graph-spdx-sbom.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

