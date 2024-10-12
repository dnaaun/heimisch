# DependencyGraphDiffInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**change_type** | **String** |  | 
**manifest** | **String** |  | 
**ecosystem** | **String** |  | 
**name** | **String** |  | 
**version** | **String** |  | 
**package_url** | Option<**String**> |  | 
**license** | Option<**String**> |  | 
**source_repository_url** | Option<**String**> |  | 
**vulnerabilities** | [**Vec<models::DependencyGraphDiffInnerVulnerabilitiesInner>**](dependency_graph_diff_inner_vulnerabilities_inner.md) |  | 
**scope** | **String** | Where the dependency is utilized. `development` means that the dependency is only utilized in the development environment. `runtime` means that the dependency is utilized at runtime and in the development environment. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


