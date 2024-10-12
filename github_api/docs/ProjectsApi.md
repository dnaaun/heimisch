# \ProjectsApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**projects_slash_add_collaborator**](ProjectsApi.md#projects_slash_add_collaborator) | **PUT** /projects/{project_id}/collaborators/{username} | Add project collaborator
[**projects_slash_create_card**](ProjectsApi.md#projects_slash_create_card) | **POST** /projects/columns/{column_id}/cards | Create a project card
[**projects_slash_create_column**](ProjectsApi.md#projects_slash_create_column) | **POST** /projects/{project_id}/columns | Create a project column
[**projects_slash_create_for_authenticated_user**](ProjectsApi.md#projects_slash_create_for_authenticated_user) | **POST** /user/projects | Create a user project
[**projects_slash_create_for_org**](ProjectsApi.md#projects_slash_create_for_org) | **POST** /orgs/{org}/projects | Create an organization project
[**projects_slash_create_for_repo**](ProjectsApi.md#projects_slash_create_for_repo) | **POST** /repos/{owner}/{repo}/projects | Create a repository project
[**projects_slash_delete**](ProjectsApi.md#projects_slash_delete) | **DELETE** /projects/{project_id} | Delete a project
[**projects_slash_delete_card**](ProjectsApi.md#projects_slash_delete_card) | **DELETE** /projects/columns/cards/{card_id} | Delete a project card
[**projects_slash_delete_column**](ProjectsApi.md#projects_slash_delete_column) | **DELETE** /projects/columns/{column_id} | Delete a project column
[**projects_slash_get**](ProjectsApi.md#projects_slash_get) | **GET** /projects/{project_id} | Get a project
[**projects_slash_get_card**](ProjectsApi.md#projects_slash_get_card) | **GET** /projects/columns/cards/{card_id} | Get a project card
[**projects_slash_get_column**](ProjectsApi.md#projects_slash_get_column) | **GET** /projects/columns/{column_id} | Get a project column
[**projects_slash_get_permission_for_user**](ProjectsApi.md#projects_slash_get_permission_for_user) | **GET** /projects/{project_id}/collaborators/{username}/permission | Get project permission for a user
[**projects_slash_list_cards**](ProjectsApi.md#projects_slash_list_cards) | **GET** /projects/columns/{column_id}/cards | List project cards
[**projects_slash_list_collaborators**](ProjectsApi.md#projects_slash_list_collaborators) | **GET** /projects/{project_id}/collaborators | List project collaborators
[**projects_slash_list_columns**](ProjectsApi.md#projects_slash_list_columns) | **GET** /projects/{project_id}/columns | List project columns
[**projects_slash_list_for_org**](ProjectsApi.md#projects_slash_list_for_org) | **GET** /orgs/{org}/projects | List organization projects
[**projects_slash_list_for_repo**](ProjectsApi.md#projects_slash_list_for_repo) | **GET** /repos/{owner}/{repo}/projects | List repository projects
[**projects_slash_list_for_user**](ProjectsApi.md#projects_slash_list_for_user) | **GET** /users/{username}/projects | List user projects
[**projects_slash_move_card**](ProjectsApi.md#projects_slash_move_card) | **POST** /projects/columns/cards/{card_id}/moves | Move a project card
[**projects_slash_move_column**](ProjectsApi.md#projects_slash_move_column) | **POST** /projects/columns/{column_id}/moves | Move a project column
[**projects_slash_remove_collaborator**](ProjectsApi.md#projects_slash_remove_collaborator) | **DELETE** /projects/{project_id}/collaborators/{username} | Remove user as a collaborator
[**projects_slash_update**](ProjectsApi.md#projects_slash_update) | **PATCH** /projects/{project_id} | Update a project
[**projects_slash_update_card**](ProjectsApi.md#projects_slash_update_card) | **PATCH** /projects/columns/cards/{card_id} | Update an existing project card
[**projects_slash_update_column**](ProjectsApi.md#projects_slash_update_column) | **PATCH** /projects/columns/{column_id} | Update an existing project column



## projects_slash_add_collaborator

> projects_slash_add_collaborator(project_id, username, projects_add_collaborator_request)
Add project collaborator

Adds a collaborator to an organization project and sets their permission level. You must be an organization owner or a project `admin` to add a collaborator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i32** | The unique identifier of the project. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |
**projects_add_collaborator_request** | Option<[**ProjectsAddCollaboratorRequest**](ProjectsAddCollaboratorRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_create_card

> models::ProjectCard projects_slash_create_card(column_id, projects_create_card_request)
Create a project card



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**column_id** | **i32** | The unique identifier of the column. | [required] |
**projects_create_card_request** | [**ProjectsCreateCardRequest**](ProjectsCreateCardRequest.md) |  | [required] |

### Return type

[**models::ProjectCard**](project-card.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_create_column

> models::ProjectColumn projects_slash_create_column(project_id, projects_update_column_request)
Create a project column

Creates a new project column.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i32** | The unique identifier of the project. | [required] |
**projects_update_column_request** | [**ProjectsUpdateColumnRequest**](ProjectsUpdateColumnRequest.md) |  | [required] |

### Return type

[**models::ProjectColumn**](project-column.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_create_for_authenticated_user

> models::Project projects_slash_create_for_authenticated_user(projects_create_for_authenticated_user_request)
Create a user project

Creates a user project board. Returns a `410 Gone` status if the user does not have existing classic projects. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**projects_create_for_authenticated_user_request** | [**ProjectsCreateForAuthenticatedUserRequest**](ProjectsCreateForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**models::Project**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_create_for_org

> models::Project projects_slash_create_for_org(org, projects_create_for_org_request)
Create an organization project

Creates an organization project board. Returns a `410 Gone` status if projects are disabled in the organization or if the organization does not have existing classic projects. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**projects_create_for_org_request** | [**ProjectsCreateForOrgRequest**](ProjectsCreateForOrgRequest.md) |  | [required] |

### Return type

[**models::Project**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_create_for_repo

> models::Project projects_slash_create_for_repo(owner, repo, projects_create_for_org_request)
Create a repository project

Creates a repository project board. Returns a `410 Gone` status if projects are disabled in the repository or if the repository does not have existing classic projects. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**projects_create_for_org_request** | [**ProjectsCreateForOrgRequest**](ProjectsCreateForOrgRequest.md) |  | [required] |

### Return type

[**models::Project**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_delete

> projects_slash_delete(project_id)
Delete a project

Deletes a project board. Returns a `404 Not Found` status if projects are disabled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i32** | The unique identifier of the project. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_delete_card

> projects_slash_delete_card(card_id)
Delete a project card

Deletes a project card

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | The unique identifier of the card. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_delete_column

> projects_slash_delete_column(column_id)
Delete a project column

Deletes a project column.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**column_id** | **i32** | The unique identifier of the column. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_get

> models::Project projects_slash_get(project_id)
Get a project

Gets a project by its `id`. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i32** | The unique identifier of the project. | [required] |

### Return type

[**models::Project**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_get_card

> models::ProjectCard projects_slash_get_card(card_id)
Get a project card

Gets information about a project card.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | The unique identifier of the card. | [required] |

### Return type

[**models::ProjectCard**](project-card.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_get_column

> models::ProjectColumn projects_slash_get_column(column_id)
Get a project column

Gets information about a project column.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**column_id** | **i32** | The unique identifier of the column. | [required] |

### Return type

[**models::ProjectColumn**](project-column.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_get_permission_for_user

> models::ProjectCollaboratorPermission projects_slash_get_permission_for_user(project_id, username)
Get project permission for a user

Returns the collaborator's permission level for an organization project. Possible values for the `permission` key: `admin`, `write`, `read`, `none`. You must be an organization owner or a project `admin` to review a user's permission level.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i32** | The unique identifier of the project. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::ProjectCollaboratorPermission**](project-collaborator-permission.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_list_cards

> Vec<models::ProjectCard> projects_slash_list_cards(column_id, archived_state, per_page, page)
List project cards

Lists the project cards in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**column_id** | **i32** | The unique identifier of the column. | [required] |
**archived_state** | Option<**String**> | Filters the project cards that are returned by the card's state. |  |[default to not_archived]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::ProjectCard>**](project-card.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_list_collaborators

> Vec<models::SimpleUser> projects_slash_list_collaborators(project_id, affiliation, per_page, page)
List project collaborators

Lists the collaborators for an organization project. For a project, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners. You must be an organization owner or a project `admin` to list collaborators.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i32** | The unique identifier of the project. | [required] |
**affiliation** | Option<**String**> | Filters the collaborators by their affiliation. `outside` means outside collaborators of a project that are not a member of the project's organization. `direct` means collaborators with permissions to a project, regardless of organization membership status. `all` means all collaborators the authenticated user can see. |  |[default to all]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::SimpleUser>**](simple-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_list_columns

> Vec<models::ProjectColumn> projects_slash_list_columns(project_id, per_page, page)
List project columns

Lists the project columns in a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i32** | The unique identifier of the project. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::ProjectColumn>**](project-column.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_list_for_org

> Vec<models::Project> projects_slash_list_for_org(org, state, per_page, page)
List organization projects

Lists the projects in an organization. Returns a `404 Not Found` status if projects are disabled in the organization. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**state** | Option<**String**> | Indicates the state of the projects to return. |  |[default to open]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Project>**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_list_for_repo

> Vec<models::Project> projects_slash_list_for_repo(owner, repo, state, per_page, page)
List repository projects

Lists the projects in a repository. Returns a `404 Not Found` status if projects are disabled in the repository. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**state** | Option<**String**> | Indicates the state of the projects to return. |  |[default to open]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Project>**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_list_for_user

> Vec<models::Project> projects_slash_list_for_user(username, state, per_page, page)
List user projects

Lists projects for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**state** | Option<**String**> | Indicates the state of the projects to return. |  |[default to open]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Project>**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_move_card

> serde_json::Value projects_slash_move_card(card_id, projects_move_card_request)
Move a project card



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | The unique identifier of the card. | [required] |
**projects_move_card_request** | [**ProjectsMoveCardRequest**](ProjectsMoveCardRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_move_column

> serde_json::Value projects_slash_move_column(column_id, projects_move_column_request)
Move a project column



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**column_id** | **i32** | The unique identifier of the column. | [required] |
**projects_move_column_request** | [**ProjectsMoveColumnRequest**](ProjectsMoveColumnRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_remove_collaborator

> projects_slash_remove_collaborator(project_id, username)
Remove user as a collaborator

Removes a collaborator from an organization project. You must be an organization owner or a project `admin` to remove a collaborator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i32** | The unique identifier of the project. | [required] |
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_update

> models::Project projects_slash_update(project_id, projects_update_request)
Update a project

Updates a project board's information. Returns a `404 Not Found` status if projects are disabled. If you do not have sufficient privileges to perform this action, a `401 Unauthorized` or `410 Gone` status is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_id** | **i32** | The unique identifier of the project. | [required] |
**projects_update_request** | Option<[**ProjectsUpdateRequest**](ProjectsUpdateRequest.md)> |  |  |

### Return type

[**models::Project**](project.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_update_card

> models::ProjectCard projects_slash_update_card(card_id, projects_update_card_request)
Update an existing project card



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**card_id** | **i32** | The unique identifier of the card. | [required] |
**projects_update_card_request** | Option<[**ProjectsUpdateCardRequest**](ProjectsUpdateCardRequest.md)> |  |  |

### Return type

[**models::ProjectCard**](project-card.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## projects_slash_update_column

> models::ProjectColumn projects_slash_update_column(column_id, projects_update_column_request)
Update an existing project column



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**column_id** | **i32** | The unique identifier of the column. | [required] |
**projects_update_column_request** | [**ProjectsUpdateColumnRequest**](ProjectsUpdateColumnRequest.md) |  | [required] |

### Return type

[**models::ProjectColumn**](project-column.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

