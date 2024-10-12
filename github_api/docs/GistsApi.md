# \GistsApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gists_slash_check_is_starred**](GistsApi.md#gists_slash_check_is_starred) | **GET** /gists/{gist_id}/star | Check if a gist is starred
[**gists_slash_create**](GistsApi.md#gists_slash_create) | **POST** /gists | Create a gist
[**gists_slash_create_comment**](GistsApi.md#gists_slash_create_comment) | **POST** /gists/{gist_id}/comments | Create a gist comment
[**gists_slash_delete**](GistsApi.md#gists_slash_delete) | **DELETE** /gists/{gist_id} | Delete a gist
[**gists_slash_delete_comment**](GistsApi.md#gists_slash_delete_comment) | **DELETE** /gists/{gist_id}/comments/{comment_id} | Delete a gist comment
[**gists_slash_fork**](GistsApi.md#gists_slash_fork) | **POST** /gists/{gist_id}/forks | Fork a gist
[**gists_slash_get**](GistsApi.md#gists_slash_get) | **GET** /gists/{gist_id} | Get a gist
[**gists_slash_get_comment**](GistsApi.md#gists_slash_get_comment) | **GET** /gists/{gist_id}/comments/{comment_id} | Get a gist comment
[**gists_slash_get_revision**](GistsApi.md#gists_slash_get_revision) | **GET** /gists/{gist_id}/{sha} | Get a gist revision
[**gists_slash_list**](GistsApi.md#gists_slash_list) | **GET** /gists | List gists for the authenticated user
[**gists_slash_list_comments**](GistsApi.md#gists_slash_list_comments) | **GET** /gists/{gist_id}/comments | List gist comments
[**gists_slash_list_commits**](GistsApi.md#gists_slash_list_commits) | **GET** /gists/{gist_id}/commits | List gist commits
[**gists_slash_list_for_user**](GistsApi.md#gists_slash_list_for_user) | **GET** /users/{username}/gists | List gists for a user
[**gists_slash_list_forks**](GistsApi.md#gists_slash_list_forks) | **GET** /gists/{gist_id}/forks | List gist forks
[**gists_slash_list_public**](GistsApi.md#gists_slash_list_public) | **GET** /gists/public | List public gists
[**gists_slash_list_starred**](GistsApi.md#gists_slash_list_starred) | **GET** /gists/starred | List starred gists
[**gists_slash_star**](GistsApi.md#gists_slash_star) | **PUT** /gists/{gist_id}/star | Star a gist
[**gists_slash_unstar**](GistsApi.md#gists_slash_unstar) | **DELETE** /gists/{gist_id}/star | Unstar a gist
[**gists_slash_update**](GistsApi.md#gists_slash_update) | **PATCH** /gists/{gist_id} | Update a gist
[**gists_slash_update_comment**](GistsApi.md#gists_slash_update_comment) | **PATCH** /gists/{gist_id}/comments/{comment_id} | Update a gist comment



## gists_slash_check_is_starred

> gists_slash_check_is_starred(gist_id)
Check if a gist is starred



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_create

> models::GistSimple gists_slash_create(gists_create_request)
Create a gist

Allows you to add a new gist with one or more files.  > [!NOTE] > Don't name your files \"gistfile\" with a numerical suffix. This is the format of the automatic naming scheme that Gist uses internally.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gists_create_request** | [**GistsCreateRequest**](GistsCreateRequest.md) |  | [required] |

### Return type

[**models::GistSimple**](gist-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_create_comment

> models::GistComment gists_slash_create_comment(gist_id, gists_create_comment_request)
Create a gist comment

Creates a comment on a gist.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type. - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |
**gists_create_comment_request** | [**GistsCreateCommentRequest**](GistsCreateCommentRequest.md) |  | [required] |

### Return type

[**models::GistComment**](gist-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_delete

> gists_slash_delete(gist_id)
Delete a gist



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_delete_comment

> gists_slash_delete_comment(gist_id, comment_id)
Delete a gist comment



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_fork

> models::BaseGist gists_slash_fork(gist_id)
Fork a gist



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |

### Return type

[**models::BaseGist**](base-gist.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_get

> models::GistSimple gists_slash_get(gist_id)
Get a gist

Gets a specified gist.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type. - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |

### Return type

[**models::GistSimple**](gist-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_get_comment

> models::GistComment gists_slash_get_comment(gist_id, comment_id)
Get a gist comment

Gets a comment on a gist.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type. - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |

### Return type

[**models::GistComment**](gist-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_get_revision

> models::GistSimple gists_slash_get_revision(gist_id, sha)
Get a gist revision

Gets a specified gist revision.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type. - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |
**sha** | **String** |  | [required] |

### Return type

[**models::GistSimple**](gist-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_list

> Vec<models::BaseGist> gists_slash_list(since, per_page, page)
List gists for the authenticated user

Lists the authenticated user's gists or if called anonymously, this endpoint returns all public gists:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**String**> | Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::BaseGist>**](base-gist.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_list_comments

> Vec<models::GistComment> gists_slash_list_comments(gist_id, per_page, page)
List gist comments

Lists the comments on a gist.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type. - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::GistComment>**](gist-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_list_commits

> Vec<models::GistCommit> gists_slash_list_commits(gist_id, per_page, page)
List gist commits



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::GistCommit>**](gist-commit.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_list_for_user

> Vec<models::BaseGist> gists_slash_list_for_user(username, since, per_page, page)
List gists for a user

Lists public gists for the specified user:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**since** | Option<**String**> | Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::BaseGist>**](base-gist.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_list_forks

> Vec<models::GistSimple> gists_slash_list_forks(gist_id, per_page, page)
List gist forks



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::GistSimple>**](gist-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_list_public

> Vec<models::BaseGist> gists_slash_list_public(since, per_page, page)
List public gists

List public gists sorted by most recently updated to least recently updated.  Note: With [pagination](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api), you can fetch up to 3000 gists. For example, you can fetch 100 pages with 30 gists per page or 30 pages with 100 gists per page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**String**> | Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::BaseGist>**](base-gist.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_list_starred

> Vec<models::BaseGist> gists_slash_list_starred(since, per_page, page)
List starred gists

List the authenticated user's starred gists:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**String**> | Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::BaseGist>**](base-gist.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_star

> gists_slash_star(gist_id)
Star a gist

Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_unstar

> gists_slash_unstar(gist_id)
Unstar a gist



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_update

> models::GistSimple gists_slash_update(gist_id, gists_update_request)
Update a gist

Allows you to update a gist's description and to update, delete, or rename gist files. Files from the previous version of the gist that aren't explicitly changed during an edit are unchanged.  At least one of `description` or `files` is required.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type. - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |
**gists_update_request** | Option<[**GistsUpdateRequest**](GistsUpdateRequest.md)> |  | [required] |

### Return type

[**models::GistSimple**](gist-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gists_slash_update_comment

> models::GistComment gists_slash_update_comment(gist_id, comment_id, gists_create_comment_request)
Update a gist comment

Updates a comment on a gist.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.raw+json`**: Returns the raw markdown. This is the default if you do not pass any specific media type. - **`application/vnd.github.base64+json`**: Returns the base64-encoded contents. This can be useful if your gist contains any invalid UTF-8 sequences.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gist_id** | **String** | The unique identifier of the gist. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |
**gists_create_comment_request** | [**GistsCreateCommentRequest**](GistsCreateCommentRequest.md) |  | [required] |

### Return type

[**models::GistComment**](gist-comment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

