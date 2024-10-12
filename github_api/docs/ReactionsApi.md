# \ReactionsApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reactions_slash_create_for_commit_comment**](ReactionsApi.md#reactions_slash_create_for_commit_comment) | **POST** /repos/{owner}/{repo}/comments/{comment_id}/reactions | Create reaction for a commit comment
[**reactions_slash_create_for_issue**](ReactionsApi.md#reactions_slash_create_for_issue) | **POST** /repos/{owner}/{repo}/issues/{issue_number}/reactions | Create reaction for an issue
[**reactions_slash_create_for_issue_comment**](ReactionsApi.md#reactions_slash_create_for_issue_comment) | **POST** /repos/{owner}/{repo}/issues/comments/{comment_id}/reactions | Create reaction for an issue comment
[**reactions_slash_create_for_pull_request_review_comment**](ReactionsApi.md#reactions_slash_create_for_pull_request_review_comment) | **POST** /repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions | Create reaction for a pull request review comment
[**reactions_slash_create_for_release**](ReactionsApi.md#reactions_slash_create_for_release) | **POST** /repos/{owner}/{repo}/releases/{release_id}/reactions | Create reaction for a release
[**reactions_slash_create_for_team_discussion_comment_in_org**](ReactionsApi.md#reactions_slash_create_for_team_discussion_comment_in_org) | **POST** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions | Create reaction for a team discussion comment
[**reactions_slash_create_for_team_discussion_comment_legacy**](ReactionsApi.md#reactions_slash_create_for_team_discussion_comment_legacy) | **POST** /teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions | Create reaction for a team discussion comment (Legacy)
[**reactions_slash_create_for_team_discussion_in_org**](ReactionsApi.md#reactions_slash_create_for_team_discussion_in_org) | **POST** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions | Create reaction for a team discussion
[**reactions_slash_create_for_team_discussion_legacy**](ReactionsApi.md#reactions_slash_create_for_team_discussion_legacy) | **POST** /teams/{team_id}/discussions/{discussion_number}/reactions | Create reaction for a team discussion (Legacy)
[**reactions_slash_delete_for_commit_comment**](ReactionsApi.md#reactions_slash_delete_for_commit_comment) | **DELETE** /repos/{owner}/{repo}/comments/{comment_id}/reactions/{reaction_id} | Delete a commit comment reaction
[**reactions_slash_delete_for_issue**](ReactionsApi.md#reactions_slash_delete_for_issue) | **DELETE** /repos/{owner}/{repo}/issues/{issue_number}/reactions/{reaction_id} | Delete an issue reaction
[**reactions_slash_delete_for_issue_comment**](ReactionsApi.md#reactions_slash_delete_for_issue_comment) | **DELETE** /repos/{owner}/{repo}/issues/comments/{comment_id}/reactions/{reaction_id} | Delete an issue comment reaction
[**reactions_slash_delete_for_pull_request_comment**](ReactionsApi.md#reactions_slash_delete_for_pull_request_comment) | **DELETE** /repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions/{reaction_id} | Delete a pull request comment reaction
[**reactions_slash_delete_for_release**](ReactionsApi.md#reactions_slash_delete_for_release) | **DELETE** /repos/{owner}/{repo}/releases/{release_id}/reactions/{reaction_id} | Delete a release reaction
[**reactions_slash_delete_for_team_discussion**](ReactionsApi.md#reactions_slash_delete_for_team_discussion) | **DELETE** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions/{reaction_id} | Delete team discussion reaction
[**reactions_slash_delete_for_team_discussion_comment**](ReactionsApi.md#reactions_slash_delete_for_team_discussion_comment) | **DELETE** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions/{reaction_id} | Delete team discussion comment reaction
[**reactions_slash_list_for_commit_comment**](ReactionsApi.md#reactions_slash_list_for_commit_comment) | **GET** /repos/{owner}/{repo}/comments/{comment_id}/reactions | List reactions for a commit comment
[**reactions_slash_list_for_issue**](ReactionsApi.md#reactions_slash_list_for_issue) | **GET** /repos/{owner}/{repo}/issues/{issue_number}/reactions | List reactions for an issue
[**reactions_slash_list_for_issue_comment**](ReactionsApi.md#reactions_slash_list_for_issue_comment) | **GET** /repos/{owner}/{repo}/issues/comments/{comment_id}/reactions | List reactions for an issue comment
[**reactions_slash_list_for_pull_request_review_comment**](ReactionsApi.md#reactions_slash_list_for_pull_request_review_comment) | **GET** /repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions | List reactions for a pull request review comment
[**reactions_slash_list_for_release**](ReactionsApi.md#reactions_slash_list_for_release) | **GET** /repos/{owner}/{repo}/releases/{release_id}/reactions | List reactions for a release
[**reactions_slash_list_for_team_discussion_comment_in_org**](ReactionsApi.md#reactions_slash_list_for_team_discussion_comment_in_org) | **GET** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/comments/{comment_number}/reactions | List reactions for a team discussion comment
[**reactions_slash_list_for_team_discussion_comment_legacy**](ReactionsApi.md#reactions_slash_list_for_team_discussion_comment_legacy) | **GET** /teams/{team_id}/discussions/{discussion_number}/comments/{comment_number}/reactions | List reactions for a team discussion comment (Legacy)
[**reactions_slash_list_for_team_discussion_in_org**](ReactionsApi.md#reactions_slash_list_for_team_discussion_in_org) | **GET** /orgs/{org}/teams/{team_slug}/discussions/{discussion_number}/reactions | List reactions for a team discussion
[**reactions_slash_list_for_team_discussion_legacy**](ReactionsApi.md#reactions_slash_list_for_team_discussion_legacy) | **GET** /teams/{team_id}/discussions/{discussion_number}/reactions | List reactions for a team discussion (Legacy)



## reactions_slash_create_for_commit_comment

> models::Reaction reactions_slash_create_for_commit_comment(owner, repo, comment_id, reactions_create_for_commit_comment_request)
Create reaction for a commit comment

Create a reaction to a [commit comment](https://docs.github.com/rest/commits/comments#get-a-commit-comment). A response with an HTTP `200` status means that you already added the reaction type to this commit comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |
**reactions_create_for_commit_comment_request** | [**ReactionsCreateForCommitCommentRequest**](ReactionsCreateForCommitCommentRequest.md) |  | [required] |

### Return type

[**models::Reaction**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_create_for_issue

> models::Reaction reactions_slash_create_for_issue(owner, repo, issue_number, reactions_create_for_issue_request)
Create reaction for an issue

Create a reaction to an [issue](https://docs.github.com/rest/issues/issues#get-an-issue). A response with an HTTP `200` status means that you already added the reaction type to this issue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**issue_number** | **i32** | The number that identifies the issue. | [required] |
**reactions_create_for_issue_request** | [**ReactionsCreateForIssueRequest**](ReactionsCreateForIssueRequest.md) |  | [required] |

### Return type

[**models::Reaction**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_create_for_issue_comment

> models::Reaction reactions_slash_create_for_issue_comment(owner, repo, comment_id, reactions_create_for_issue_comment_request)
Create reaction for an issue comment

Create a reaction to an [issue comment](https://docs.github.com/rest/issues/comments#get-an-issue-comment). A response with an HTTP `200` status means that you already added the reaction type to this issue comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |
**reactions_create_for_issue_comment_request** | [**ReactionsCreateForIssueCommentRequest**](ReactionsCreateForIssueCommentRequest.md) |  | [required] |

### Return type

[**models::Reaction**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_create_for_pull_request_review_comment

> models::Reaction reactions_slash_create_for_pull_request_review_comment(owner, repo, comment_id, reactions_create_for_pull_request_review_comment_request)
Create reaction for a pull request review comment

Create a reaction to a [pull request review comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request). A response with an HTTP `200` status means that you already added the reaction type to this pull request review comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |
**reactions_create_for_pull_request_review_comment_request** | [**ReactionsCreateForPullRequestReviewCommentRequest**](ReactionsCreateForPullRequestReviewCommentRequest.md) |  | [required] |

### Return type

[**models::Reaction**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_create_for_release

> models::Reaction reactions_slash_create_for_release(owner, repo, release_id, reactions_create_for_release_request)
Create reaction for a release

Create a reaction to a [release](https://docs.github.com/rest/releases/releases#get-a-release). A response with a `Status: 200 OK` means that you already added the reaction type to this release.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**release_id** | **i32** | The unique identifier of the release. | [required] |
**reactions_create_for_release_request** | [**ReactionsCreateForReleaseRequest**](ReactionsCreateForReleaseRequest.md) |  | [required] |

### Return type

[**models::Reaction**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_create_for_team_discussion_comment_in_org

> models::Reaction reactions_slash_create_for_team_discussion_comment_in_org(org, team_slug, discussion_number, comment_number, reactions_create_for_team_discussion_comment_in_org_request)
Create reaction for a team discussion comment

Create a reaction to a [team discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment).  A response with an HTTP `200` status means that you already added the reaction type to this team discussion comment.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions`.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**comment_number** | **i32** | The number that identifies the comment. | [required] |
**reactions_create_for_team_discussion_comment_in_org_request** | [**ReactionsCreateForTeamDiscussionCommentInOrgRequest**](ReactionsCreateForTeamDiscussionCommentInOrgRequest.md) |  | [required] |

### Return type

[**models::Reaction**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_create_for_team_discussion_comment_legacy

> models::Reaction reactions_slash_create_for_team_discussion_comment_legacy(team_id, discussion_number, comment_number, reactions_create_for_team_discussion_comment_in_org_request)
Create reaction for a team discussion comment (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new \"[Create reaction for a team discussion comment](https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion-comment)\" endpoint.  Create a reaction to a [team discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment).  A response with an HTTP `200` status means that you already added the reaction type to this team discussion comment.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**comment_number** | **i32** | The number that identifies the comment. | [required] |
**reactions_create_for_team_discussion_comment_in_org_request** | [**ReactionsCreateForTeamDiscussionCommentInOrgRequest**](ReactionsCreateForTeamDiscussionCommentInOrgRequest.md) |  | [required] |

### Return type

[**models::Reaction**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_create_for_team_discussion_in_org

> models::Reaction reactions_slash_create_for_team_discussion_in_org(org, team_slug, discussion_number, reactions_create_for_team_discussion_in_org_request)
Create reaction for a team discussion

Create a reaction to a [team discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion).  A response with an HTTP `200` status means that you already added the reaction type to this team discussion.  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `POST /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions`.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**reactions_create_for_team_discussion_in_org_request** | [**ReactionsCreateForTeamDiscussionInOrgRequest**](ReactionsCreateForTeamDiscussionInOrgRequest.md) |  | [required] |

### Return type

[**models::Reaction**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_create_for_team_discussion_legacy

> models::Reaction reactions_slash_create_for_team_discussion_legacy(team_id, discussion_number, reactions_create_for_team_discussion_in_org_request)
Create reaction for a team discussion (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create reaction for a team discussion`](https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-team-discussion) endpoint.  Create a reaction to a [team discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion).  A response with an HTTP `200` status means that you already added the reaction type to this team discussion.  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**reactions_create_for_team_discussion_in_org_request** | [**ReactionsCreateForTeamDiscussionInOrgRequest**](ReactionsCreateForTeamDiscussionInOrgRequest.md) |  | [required] |

### Return type

[**models::Reaction**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_delete_for_commit_comment

> reactions_slash_delete_for_commit_comment(owner, repo, comment_id, reaction_id)
Delete a commit comment reaction

> [!NOTE] > You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/comments/:comment_id/reactions/:reaction_id`.  Delete a reaction to a [commit comment](https://docs.github.com/rest/commits/comments#get-a-commit-comment).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |
**reaction_id** | **i32** | The unique identifier of the reaction. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_delete_for_issue

> reactions_slash_delete_for_issue(owner, repo, issue_number, reaction_id)
Delete an issue reaction

> [!NOTE] > You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/issues/:issue_number/reactions/:reaction_id`.  Delete a reaction to an [issue](https://docs.github.com/rest/issues/issues#get-an-issue).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**issue_number** | **i32** | The number that identifies the issue. | [required] |
**reaction_id** | **i32** | The unique identifier of the reaction. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_delete_for_issue_comment

> reactions_slash_delete_for_issue_comment(owner, repo, comment_id, reaction_id)
Delete an issue comment reaction

> [!NOTE] > You can also specify a repository by `repository_id` using the route `DELETE delete /repositories/:repository_id/issues/comments/:comment_id/reactions/:reaction_id`.  Delete a reaction to an [issue comment](https://docs.github.com/rest/issues/comments#get-an-issue-comment).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |
**reaction_id** | **i32** | The unique identifier of the reaction. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_delete_for_pull_request_comment

> reactions_slash_delete_for_pull_request_comment(owner, repo, comment_id, reaction_id)
Delete a pull request comment reaction

> [!NOTE] > You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/pulls/comments/:comment_id/reactions/:reaction_id.`  Delete a reaction to a [pull request review comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |
**reaction_id** | **i32** | The unique identifier of the reaction. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_delete_for_release

> reactions_slash_delete_for_release(owner, repo, release_id, reaction_id)
Delete a release reaction

> [!NOTE] > You can also specify a repository by `repository_id` using the route `DELETE delete /repositories/:repository_id/releases/:release_id/reactions/:reaction_id`.  Delete a reaction to a [release](https://docs.github.com/rest/releases/releases#get-a-release).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**release_id** | **i32** | The unique identifier of the release. | [required] |
**reaction_id** | **i32** | The unique identifier of the reaction. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_delete_for_team_discussion

> reactions_slash_delete_for_team_discussion(org, team_slug, discussion_number, reaction_id)
Delete team discussion reaction

> [!NOTE] > You can also specify a team or organization with `team_id` and `org_id` using the route `DELETE /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions/:reaction_id`.  Delete a reaction to a [team discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion).  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**reaction_id** | **i32** | The unique identifier of the reaction. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_delete_for_team_discussion_comment

> reactions_slash_delete_for_team_discussion_comment(org, team_slug, discussion_number, comment_number, reaction_id)
Delete team discussion comment reaction

> [!NOTE] > You can also specify a team or organization with `team_id` and `org_id` using the route `DELETE /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions/:reaction_id`.  Delete a reaction to a [team discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment).  OAuth app tokens and personal access tokens (classic) need the `write:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**comment_number** | **i32** | The number that identifies the comment. | [required] |
**reaction_id** | **i32** | The unique identifier of the reaction. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_list_for_commit_comment

> Vec<models::Reaction> reactions_slash_list_for_commit_comment(owner, repo, comment_id, content, per_page, page)
List reactions for a commit comment

List the reactions to a [commit comment](https://docs.github.com/rest/commits/comments#get-a-commit-comment).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |
**content** | Option<**String**> | Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a commit comment. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Reaction>**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_list_for_issue

> Vec<models::Reaction> reactions_slash_list_for_issue(owner, repo, issue_number, content, per_page, page)
List reactions for an issue

List the reactions to an [issue](https://docs.github.com/rest/issues/issues#get-an-issue).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**issue_number** | **i32** | The number that identifies the issue. | [required] |
**content** | Option<**String**> | Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to an issue. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Reaction>**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_list_for_issue_comment

> Vec<models::Reaction> reactions_slash_list_for_issue_comment(owner, repo, comment_id, content, per_page, page)
List reactions for an issue comment

List the reactions to an [issue comment](https://docs.github.com/rest/issues/comments#get-an-issue-comment).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |
**content** | Option<**String**> | Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to an issue comment. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Reaction>**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_list_for_pull_request_review_comment

> Vec<models::Reaction> reactions_slash_list_for_pull_request_review_comment(owner, repo, comment_id, content, per_page, page)
List reactions for a pull request review comment

List the reactions to a [pull request review comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**comment_id** | **i64** | The unique identifier of the comment. | [required] |
**content** | Option<**String**> | Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a pull request review comment. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Reaction>**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_list_for_release

> Vec<models::Reaction> reactions_slash_list_for_release(owner, repo, release_id, content, per_page, page)
List reactions for a release

List the reactions to a [release](https://docs.github.com/rest/releases/releases#get-a-release).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**release_id** | **i32** | The unique identifier of the release. | [required] |
**content** | Option<**String**> | Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a release. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Reaction>**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_list_for_team_discussion_comment_in_org

> Vec<models::Reaction> reactions_slash_list_for_team_discussion_comment_in_org(org, team_slug, discussion_number, comment_number, content, per_page, page)
List reactions for a team discussion comment

List the reactions to a [team discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment).  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/:org_id/team/:team_id/discussions/:discussion_number/comments/:comment_number/reactions`.  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**comment_number** | **i32** | The number that identifies the comment. | [required] |
**content** | Option<**String**> | Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a team discussion comment. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Reaction>**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_list_for_team_discussion_comment_legacy

> Vec<models::Reaction> reactions_slash_list_for_team_discussion_comment_legacy(team_id, discussion_number, comment_number, content, per_page, page)
List reactions for a team discussion comment (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List reactions for a team discussion comment`](https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion-comment) endpoint.  List the reactions to a [team discussion comment](https://docs.github.com/rest/teams/discussion-comments#get-a-discussion-comment).  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**comment_number** | **i32** | The number that identifies the comment. | [required] |
**content** | Option<**String**> | Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a team discussion comment. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Reaction>**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_list_for_team_discussion_in_org

> Vec<models::Reaction> reactions_slash_list_for_team_discussion_in_org(org, team_slug, discussion_number, content, per_page, page)
List reactions for a team discussion

List the reactions to a [team discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion).  > [!NOTE] > You can also specify a team by `org_id` and `team_id` using the route `GET /organizations/:org_id/team/:team_id/discussions/:discussion_number/reactions`.  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**team_slug** | **String** | The slug of the team name. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**content** | Option<**String**> | Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a team discussion. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Reaction>**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reactions_slash_list_for_team_discussion_legacy

> Vec<models::Reaction> reactions_slash_list_for_team_discussion_legacy(team_id, discussion_number, content, per_page, page)
List reactions for a team discussion (Legacy)

> [!WARNING] > **Endpoint closing down notice:** This endpoint route is closing down and will be removed from the Teams API. We recommend migrating your existing code to use the new [`List reactions for a team discussion`](https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-team-discussion) endpoint.  List the reactions to a [team discussion](https://docs.github.com/rest/teams/discussions#get-a-discussion).  OAuth app tokens and personal access tokens (classic) need the `read:discussion` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | **i32** | The unique identifier of the team. | [required] |
**discussion_number** | **i32** | The number that identifies the discussion. | [required] |
**content** | Option<**String**> | Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a team discussion. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Reaction>**](reaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

