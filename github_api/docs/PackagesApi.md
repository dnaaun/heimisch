# \PackagesApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**packages_slash_delete_package_for_authenticated_user**](PackagesApi.md#packages_slash_delete_package_for_authenticated_user) | **DELETE** /user/packages/{package_type}/{package_name} | Delete a package for the authenticated user
[**packages_slash_delete_package_for_org**](PackagesApi.md#packages_slash_delete_package_for_org) | **DELETE** /orgs/{org}/packages/{package_type}/{package_name} | Delete a package for an organization
[**packages_slash_delete_package_for_user**](PackagesApi.md#packages_slash_delete_package_for_user) | **DELETE** /users/{username}/packages/{package_type}/{package_name} | Delete a package for a user
[**packages_slash_delete_package_version_for_authenticated_user**](PackagesApi.md#packages_slash_delete_package_version_for_authenticated_user) | **DELETE** /user/packages/{package_type}/{package_name}/versions/{package_version_id} | Delete a package version for the authenticated user
[**packages_slash_delete_package_version_for_org**](PackagesApi.md#packages_slash_delete_package_version_for_org) | **DELETE** /orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id} | Delete package version for an organization
[**packages_slash_delete_package_version_for_user**](PackagesApi.md#packages_slash_delete_package_version_for_user) | **DELETE** /users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id} | Delete package version for a user
[**packages_slash_get_all_package_versions_for_package_owned_by_authenticated_user**](PackagesApi.md#packages_slash_get_all_package_versions_for_package_owned_by_authenticated_user) | **GET** /user/packages/{package_type}/{package_name}/versions | List package versions for a package owned by the authenticated user
[**packages_slash_get_all_package_versions_for_package_owned_by_org**](PackagesApi.md#packages_slash_get_all_package_versions_for_package_owned_by_org) | **GET** /orgs/{org}/packages/{package_type}/{package_name}/versions | List package versions for a package owned by an organization
[**packages_slash_get_all_package_versions_for_package_owned_by_user**](PackagesApi.md#packages_slash_get_all_package_versions_for_package_owned_by_user) | **GET** /users/{username}/packages/{package_type}/{package_name}/versions | List package versions for a package owned by a user
[**packages_slash_get_package_for_authenticated_user**](PackagesApi.md#packages_slash_get_package_for_authenticated_user) | **GET** /user/packages/{package_type}/{package_name} | Get a package for the authenticated user
[**packages_slash_get_package_for_organization**](PackagesApi.md#packages_slash_get_package_for_organization) | **GET** /orgs/{org}/packages/{package_type}/{package_name} | Get a package for an organization
[**packages_slash_get_package_for_user**](PackagesApi.md#packages_slash_get_package_for_user) | **GET** /users/{username}/packages/{package_type}/{package_name} | Get a package for a user
[**packages_slash_get_package_version_for_authenticated_user**](PackagesApi.md#packages_slash_get_package_version_for_authenticated_user) | **GET** /user/packages/{package_type}/{package_name}/versions/{package_version_id} | Get a package version for the authenticated user
[**packages_slash_get_package_version_for_organization**](PackagesApi.md#packages_slash_get_package_version_for_organization) | **GET** /orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id} | Get a package version for an organization
[**packages_slash_get_package_version_for_user**](PackagesApi.md#packages_slash_get_package_version_for_user) | **GET** /users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id} | Get a package version for a user
[**packages_slash_list_docker_migration_conflicting_packages_for_authenticated_user**](PackagesApi.md#packages_slash_list_docker_migration_conflicting_packages_for_authenticated_user) | **GET** /user/docker/conflicts | Get list of conflicting packages during Docker migration for authenticated-user
[**packages_slash_list_docker_migration_conflicting_packages_for_organization**](PackagesApi.md#packages_slash_list_docker_migration_conflicting_packages_for_organization) | **GET** /orgs/{org}/docker/conflicts | Get list of conflicting packages during Docker migration for organization
[**packages_slash_list_docker_migration_conflicting_packages_for_user**](PackagesApi.md#packages_slash_list_docker_migration_conflicting_packages_for_user) | **GET** /users/{username}/docker/conflicts | Get list of conflicting packages during Docker migration for user
[**packages_slash_list_packages_for_authenticated_user**](PackagesApi.md#packages_slash_list_packages_for_authenticated_user) | **GET** /user/packages | List packages for the authenticated user's namespace
[**packages_slash_list_packages_for_organization**](PackagesApi.md#packages_slash_list_packages_for_organization) | **GET** /orgs/{org}/packages | List packages for an organization
[**packages_slash_list_packages_for_user**](PackagesApi.md#packages_slash_list_packages_for_user) | **GET** /users/{username}/packages | List packages for a user
[**packages_slash_restore_package_for_authenticated_user**](PackagesApi.md#packages_slash_restore_package_for_authenticated_user) | **POST** /user/packages/{package_type}/{package_name}/restore | Restore a package for the authenticated user
[**packages_slash_restore_package_for_org**](PackagesApi.md#packages_slash_restore_package_for_org) | **POST** /orgs/{org}/packages/{package_type}/{package_name}/restore | Restore a package for an organization
[**packages_slash_restore_package_for_user**](PackagesApi.md#packages_slash_restore_package_for_user) | **POST** /users/{username}/packages/{package_type}/{package_name}/restore | Restore a package for a user
[**packages_slash_restore_package_version_for_authenticated_user**](PackagesApi.md#packages_slash_restore_package_version_for_authenticated_user) | **POST** /user/packages/{package_type}/{package_name}/versions/{package_version_id}/restore | Restore a package version for the authenticated user
[**packages_slash_restore_package_version_for_org**](PackagesApi.md#packages_slash_restore_package_version_for_org) | **POST** /orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore | Restore package version for an organization
[**packages_slash_restore_package_version_for_user**](PackagesApi.md#packages_slash_restore_package_version_for_user) | **POST** /users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore | Restore package version for a user



## packages_slash_delete_package_for_authenticated_user

> packages_slash_delete_package_for_authenticated_user(package_type, package_name)
Delete a package for the authenticated user

Deletes a package owned by the authenticated user. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_delete_package_for_org

> packages_slash_delete_package_for_org(package_type, package_name, org)
Delete a package for an organization

Deletes an entire package in an organization. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.  The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).\"  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_delete_package_for_user

> packages_slash_delete_package_for_user(package_type, package_name, username)
Delete a package for a user

Deletes an entire package for a user. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.  If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).\"  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_delete_package_version_for_authenticated_user

> packages_slash_delete_package_version_for_authenticated_user(package_type, package_name, package_version_id)
Delete a package version for the authenticated user

Deletes a specific package version for a package owned by the authenticated user.  If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.  The authenticated user must have admin permissions in the organization to use this endpoint.  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**package_version_id** | **i32** | Unique identifier of the package version. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_delete_package_version_for_org

> packages_slash_delete_package_version_for_org(package_type, package_name, org, package_version_id)
Delete package version for an organization

Deletes a specific package version in an organization. If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.  The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).\"  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**package_version_id** | **i32** | Unique identifier of the package version. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_delete_package_version_for_user

> packages_slash_delete_package_version_for_user(package_type, package_name, username, package_version_id)
Delete package version for a user

Deletes a specific package version for a user. If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.  If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).\"  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**package_version_id** | **i32** | Unique identifier of the package version. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_get_all_package_versions_for_package_owned_by_authenticated_user

> Vec<models::PackageVersion> packages_slash_get_all_package_versions_for_package_owned_by_authenticated_user(package_type, package_name, page, per_page, state)
List package versions for a package owned by the authenticated user

Lists package versions for a package owned by the authenticated user.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**state** | Option<**String**> | The state of the package, either active or deleted. |  |[default to active]

### Return type

[**Vec<models::PackageVersion>**](package-version.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_get_all_package_versions_for_package_owned_by_org

> Vec<models::PackageVersion> packages_slash_get_all_package_versions_for_package_owned_by_org(package_type, package_name, org, page, per_page, state)
List package versions for a package owned by an organization

Lists package versions for a package owned by an organization.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**state** | Option<**String**> | The state of the package, either active or deleted. |  |[default to active]

### Return type

[**Vec<models::PackageVersion>**](package-version.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_get_all_package_versions_for_package_owned_by_user

> Vec<models::PackageVersion> packages_slash_get_all_package_versions_for_package_owned_by_user(package_type, package_name, username)
List package versions for a package owned by a user

Lists package versions for a public package owned by a specified user.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**Vec<models::PackageVersion>**](package-version.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_get_package_for_authenticated_user

> models::Package packages_slash_get_package_for_authenticated_user(package_type, package_name)
Get a package for the authenticated user

Gets a specific package for a package owned by the authenticated user.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |

### Return type

[**models::Package**](package.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_get_package_for_organization

> models::Package packages_slash_get_package_for_organization(package_type, package_name, org)
Get a package for an organization

Gets a specific package in an organization.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::Package**](package.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_get_package_for_user

> models::Package packages_slash_get_package_for_user(package_type, package_name, username)
Get a package for a user

Gets a specific package metadata for a public package owned by a user.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::Package**](package.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_get_package_version_for_authenticated_user

> models::PackageVersion packages_slash_get_package_version_for_authenticated_user(package_type, package_name, package_version_id)
Get a package version for the authenticated user

Gets a specific package version for a package owned by the authenticated user.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**package_version_id** | **i32** | Unique identifier of the package version. | [required] |

### Return type

[**models::PackageVersion**](package-version.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_get_package_version_for_organization

> models::PackageVersion packages_slash_get_package_version_for_organization(package_type, package_name, org, package_version_id)
Get a package version for an organization

Gets a specific package version in an organization.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**package_version_id** | **i32** | Unique identifier of the package version. | [required] |

### Return type

[**models::PackageVersion**](package-version.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_get_package_version_for_user

> models::PackageVersion packages_slash_get_package_version_for_user(package_type, package_name, package_version_id, username)
Get a package version for a user

Gets a specific package version for a public package owned by a specified user.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**package_version_id** | **i32** | Unique identifier of the package version. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::PackageVersion**](package-version.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_list_docker_migration_conflicting_packages_for_authenticated_user

> Vec<models::Package> packages_slash_list_docker_migration_conflicting_packages_for_authenticated_user()
Get list of conflicting packages during Docker migration for authenticated-user

Lists all packages that are owned by the authenticated user within the user's namespace, and that encountered a conflict during a Docker migration.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Package>**](package.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_list_docker_migration_conflicting_packages_for_organization

> Vec<models::Package> packages_slash_list_docker_migration_conflicting_packages_for_organization(org)
Get list of conflicting packages during Docker migration for organization

Lists all packages that are in a specific organization, are readable by the requesting user, and that encountered a conflict during a Docker migration.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::Package>**](package.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_list_docker_migration_conflicting_packages_for_user

> Vec<models::Package> packages_slash_list_docker_migration_conflicting_packages_for_user(username)
Get list of conflicting packages during Docker migration for user

Lists all packages that are in a specific user's namespace, that the requesting user has access to, and that encountered a conflict during Docker migration.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**Vec<models::Package>**](package.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_list_packages_for_authenticated_user

> Vec<models::Package> packages_slash_list_packages_for_authenticated_user(package_type, visibility, page, per_page)
List packages for the authenticated user's namespace

Lists packages owned by the authenticated user within the user's namespace.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**visibility** | Option<**String**> | The selected visibility of the packages.  This parameter is optional and only filters an existing result set.  The `internal` visibility is only supported for GitHub Packages registries that allow for granular permissions. For other ecosystems `internal` is synonymous with `private`. For the list of GitHub Packages registries that support granular permissions, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).\" |  |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**Vec<models::Package>**](package.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_list_packages_for_organization

> Vec<models::Package> packages_slash_list_packages_for_organization(package_type, org, visibility, page, per_page)
List packages for an organization

Lists packages in an organization readable by the user.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**visibility** | Option<**String**> | The selected visibility of the packages.  This parameter is optional and only filters an existing result set.  The `internal` visibility is only supported for GitHub Packages registries that allow for granular permissions. For other ecosystems `internal` is synonymous with `private`. For the list of GitHub Packages registries that support granular permissions, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).\" |  |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**Vec<models::Package>**](package.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_list_packages_for_user

> Vec<models::Package> packages_slash_list_packages_for_user(package_type, username, visibility, page, per_page)
List packages for a user

Lists all packages in a user's namespace for which the requesting user has access.  OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**visibility** | Option<**String**> | The selected visibility of the packages.  This parameter is optional and only filters an existing result set.  The `internal` visibility is only supported for GitHub Packages registries that allow for granular permissions. For other ecosystems `internal` is synonymous with `private`. For the list of GitHub Packages registries that support granular permissions, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).\" |  |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**Vec<models::Package>**](package.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_restore_package_for_authenticated_user

> packages_slash_restore_package_for_authenticated_user(package_type, package_name, token)
Restore a package for the authenticated user

Restores a package owned by the authenticated user.  You can restore a deleted package under the following conditions:   - The package was deleted within the last 30 days.   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**token** | Option<**String**> | package token |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_restore_package_for_org

> packages_slash_restore_package_for_org(package_type, package_name, org, token)
Restore a package for an organization

Restores an entire package in an organization.  You can restore a deleted package under the following conditions:   - The package was deleted within the last 30 days.   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.  The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).\"  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**token** | Option<**String**> | package token |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_restore_package_for_user

> packages_slash_restore_package_for_user(package_type, package_name, username, token)
Restore a package for a user

Restores an entire package for a user.  You can restore a deleted package under the following conditions:   - The package was deleted within the last 30 days.   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.  If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).\"  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**token** | Option<**String**> | package token |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_restore_package_version_for_authenticated_user

> packages_slash_restore_package_version_for_authenticated_user(package_type, package_name, package_version_id)
Restore a package version for the authenticated user

Restores a package version owned by the authenticated user.  You can restore a deleted package version under the following conditions:   - The package was deleted within the last 30 days.   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**package_version_id** | **i32** | Unique identifier of the package version. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_restore_package_version_for_org

> packages_slash_restore_package_version_for_org(package_type, package_name, org, package_version_id)
Restore package version for an organization

Restores a specific package version in an organization.  You can restore a deleted package under the following conditions:   - The package was deleted within the last 30 days.   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.  The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).\"  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**package_version_id** | **i32** | Unique identifier of the package version. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## packages_slash_restore_package_version_for_user

> packages_slash_restore_package_version_for_user(package_type, package_name, username, package_version_id)
Restore package version for a user

Restores a specific package version for a user.  You can restore a deleted package under the following conditions:   - The package was deleted within the last 30 days.   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.  If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).\"  OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see \"[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**package_type** | **String** | The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry. | [required] |
**package_name** | **String** | The name of the package. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**package_version_id** | **i32** | Unique identifier of the package version. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

