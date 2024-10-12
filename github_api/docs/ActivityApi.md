# \ActivityApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activity_slash_check_repo_is_starred_by_authenticated_user**](ActivityApi.md#activity_slash_check_repo_is_starred_by_authenticated_user) | **GET** /user/starred/{owner}/{repo} | Check if a repository is starred by the authenticated user
[**activity_slash_delete_repo_subscription**](ActivityApi.md#activity_slash_delete_repo_subscription) | **DELETE** /repos/{owner}/{repo}/subscription | Delete a repository subscription
[**activity_slash_delete_thread_subscription**](ActivityApi.md#activity_slash_delete_thread_subscription) | **DELETE** /notifications/threads/{thread_id}/subscription | Delete a thread subscription
[**activity_slash_get_feeds**](ActivityApi.md#activity_slash_get_feeds) | **GET** /feeds | Get feeds
[**activity_slash_get_repo_subscription**](ActivityApi.md#activity_slash_get_repo_subscription) | **GET** /repos/{owner}/{repo}/subscription | Get a repository subscription
[**activity_slash_get_thread**](ActivityApi.md#activity_slash_get_thread) | **GET** /notifications/threads/{thread_id} | Get a thread
[**activity_slash_get_thread_subscription_for_authenticated_user**](ActivityApi.md#activity_slash_get_thread_subscription_for_authenticated_user) | **GET** /notifications/threads/{thread_id}/subscription | Get a thread subscription for the authenticated user
[**activity_slash_list_events_for_authenticated_user**](ActivityApi.md#activity_slash_list_events_for_authenticated_user) | **GET** /users/{username}/events | List events for the authenticated user
[**activity_slash_list_notifications_for_authenticated_user**](ActivityApi.md#activity_slash_list_notifications_for_authenticated_user) | **GET** /notifications | List notifications for the authenticated user
[**activity_slash_list_org_events_for_authenticated_user**](ActivityApi.md#activity_slash_list_org_events_for_authenticated_user) | **GET** /users/{username}/events/orgs/{org} | List organization events for the authenticated user
[**activity_slash_list_public_events**](ActivityApi.md#activity_slash_list_public_events) | **GET** /events | List public events
[**activity_slash_list_public_events_for_repo_network**](ActivityApi.md#activity_slash_list_public_events_for_repo_network) | **GET** /networks/{owner}/{repo}/events | List public events for a network of repositories
[**activity_slash_list_public_events_for_user**](ActivityApi.md#activity_slash_list_public_events_for_user) | **GET** /users/{username}/events/public | List public events for a user
[**activity_slash_list_public_org_events**](ActivityApi.md#activity_slash_list_public_org_events) | **GET** /orgs/{org}/events | List public organization events
[**activity_slash_list_received_events_for_user**](ActivityApi.md#activity_slash_list_received_events_for_user) | **GET** /users/{username}/received_events | List events received by the authenticated user
[**activity_slash_list_received_public_events_for_user**](ActivityApi.md#activity_slash_list_received_public_events_for_user) | **GET** /users/{username}/received_events/public | List public events received by a user
[**activity_slash_list_repo_events**](ActivityApi.md#activity_slash_list_repo_events) | **GET** /repos/{owner}/{repo}/events | List repository events
[**activity_slash_list_repo_notifications_for_authenticated_user**](ActivityApi.md#activity_slash_list_repo_notifications_for_authenticated_user) | **GET** /repos/{owner}/{repo}/notifications | List repository notifications for the authenticated user
[**activity_slash_list_repos_starred_by_authenticated_user**](ActivityApi.md#activity_slash_list_repos_starred_by_authenticated_user) | **GET** /user/starred | List repositories starred by the authenticated user
[**activity_slash_list_repos_starred_by_user**](ActivityApi.md#activity_slash_list_repos_starred_by_user) | **GET** /users/{username}/starred | List repositories starred by a user
[**activity_slash_list_repos_watched_by_user**](ActivityApi.md#activity_slash_list_repos_watched_by_user) | **GET** /users/{username}/subscriptions | List repositories watched by a user
[**activity_slash_list_stargazers_for_repo**](ActivityApi.md#activity_slash_list_stargazers_for_repo) | **GET** /repos/{owner}/{repo}/stargazers | List stargazers
[**activity_slash_list_watched_repos_for_authenticated_user**](ActivityApi.md#activity_slash_list_watched_repos_for_authenticated_user) | **GET** /user/subscriptions | List repositories watched by the authenticated user
[**activity_slash_list_watchers_for_repo**](ActivityApi.md#activity_slash_list_watchers_for_repo) | **GET** /repos/{owner}/{repo}/subscribers | List watchers
[**activity_slash_mark_notifications_as_read**](ActivityApi.md#activity_slash_mark_notifications_as_read) | **PUT** /notifications | Mark notifications as read
[**activity_slash_mark_repo_notifications_as_read**](ActivityApi.md#activity_slash_mark_repo_notifications_as_read) | **PUT** /repos/{owner}/{repo}/notifications | Mark repository notifications as read
[**activity_slash_mark_thread_as_done**](ActivityApi.md#activity_slash_mark_thread_as_done) | **DELETE** /notifications/threads/{thread_id} | Mark a thread as done
[**activity_slash_mark_thread_as_read**](ActivityApi.md#activity_slash_mark_thread_as_read) | **PATCH** /notifications/threads/{thread_id} | Mark a thread as read
[**activity_slash_set_repo_subscription**](ActivityApi.md#activity_slash_set_repo_subscription) | **PUT** /repos/{owner}/{repo}/subscription | Set a repository subscription
[**activity_slash_set_thread_subscription**](ActivityApi.md#activity_slash_set_thread_subscription) | **PUT** /notifications/threads/{thread_id}/subscription | Set a thread subscription
[**activity_slash_star_repo_for_authenticated_user**](ActivityApi.md#activity_slash_star_repo_for_authenticated_user) | **PUT** /user/starred/{owner}/{repo} | Star a repository for the authenticated user
[**activity_slash_unstar_repo_for_authenticated_user**](ActivityApi.md#activity_slash_unstar_repo_for_authenticated_user) | **DELETE** /user/starred/{owner}/{repo} | Unstar a repository for the authenticated user



## activity_slash_check_repo_is_starred_by_authenticated_user

> activity_slash_check_repo_is_starred_by_authenticated_user(owner, repo)
Check if a repository is starred by the authenticated user

Whether the authenticated user has starred the repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_delete_repo_subscription

> activity_slash_delete_repo_subscription(owner, repo)
Delete a repository subscription

This endpoint should only be used to stop watching a repository. To control whether or not you wish to receive notifications from a repository, [set the repository's subscription manually](https://docs.github.com/rest/activity/watching#set-a-repository-subscription).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_delete_thread_subscription

> activity_slash_delete_thread_subscription(thread_id)
Delete a thread subscription

Mutes all future notifications for a conversation until you comment on the thread or get an **@mention**. If you are watching the repository of the thread, you will still receive notifications. To ignore future notifications for a repository you are watching, use the [Set a thread subscription](https://docs.github.com/rest/activity/notifications#set-a-thread-subscription) endpoint and set `ignore` to `true`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **i32** | The unique identifier of the notification thread. This corresponds to the value returned in the `id` field when you retrieve notifications (for example with the [`GET /notifications` operation](https://docs.github.com/rest/activity/notifications#list-notifications-for-the-authenticated-user)). | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_get_feeds

> models::Feed activity_slash_get_feeds()
Get feeds

Lists the feeds available to the authenticated user. The response provides a URL for each feed. You can then get a specific feed by sending a request to one of the feed URLs.  *   **Timeline**: The GitHub global public timeline *   **User**: The public timeline for any user, using `uri_template`. For more information, see \"[Hypermedia](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#hypermedia).\" *   **Current user public**: The public timeline for the authenticated user *   **Current user**: The private timeline for the authenticated user *   **Current user actor**: The private timeline for activity created by the authenticated user *   **Current user organizations**: The private timeline for the organizations the authenticated user is a member of. *   **Security advisories**: A collection of public announcements that provide information about security-related vulnerabilities in software on GitHub.  By default, timeline resources are returned in JSON. You can specify the `application/atom+xml` type in the `Accept` header to return timeline resources in Atom format. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  > [!NOTE] > Private feeds are only returned when [authenticating via Basic Auth](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) since current feed URIs use the older, non revocable auth tokens.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Feed**](feed.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_get_repo_subscription

> models::RepositorySubscription activity_slash_get_repo_subscription(owner, repo)
Get a repository subscription

Gets information about whether the authenticated user is subscribed to the repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::RepositorySubscription**](repository-subscription.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_get_thread

> models::Thread activity_slash_get_thread(thread_id)
Get a thread

Gets information about a notification thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **i32** | The unique identifier of the notification thread. This corresponds to the value returned in the `id` field when you retrieve notifications (for example with the [`GET /notifications` operation](https://docs.github.com/rest/activity/notifications#list-notifications-for-the-authenticated-user)). | [required] |

### Return type

[**models::Thread**](thread.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_get_thread_subscription_for_authenticated_user

> models::ThreadSubscription activity_slash_get_thread_subscription_for_authenticated_user(thread_id)
Get a thread subscription for the authenticated user

This checks to see if the current user is subscribed to a thread. You can also [get a repository subscription](https://docs.github.com/rest/activity/watching#get-a-repository-subscription).  Note that subscriptions are only generated if a user is participating in a conversation--for example, they've replied to the thread, were **@mentioned**, or manually subscribe to a thread.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **i32** | The unique identifier of the notification thread. This corresponds to the value returned in the `id` field when you retrieve notifications (for example with the [`GET /notifications` operation](https://docs.github.com/rest/activity/notifications#list-notifications-for-the-authenticated-user)). | [required] |

### Return type

[**models::ThreadSubscription**](thread-subscription.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_events_for_authenticated_user

> Vec<models::Event> activity_slash_list_events_for_authenticated_user(username, per_page, page)
List events for the authenticated user

If you are authenticated as the given user, you will see your private events. Otherwise, you'll only see public events. _Optional_: use the fine-grained token with following permission set to view private events: \"Events\" user permissions (read).  > [!NOTE] > This API is not built to serve real-time use cases. Depending on the time of day, event latency can be anywhere from 30s to 6h.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_notifications_for_authenticated_user

> Vec<models::Thread> activity_slash_list_notifications_for_authenticated_user(all, participating, since, before, page, per_page)
List notifications for the authenticated user

List all notifications for the current user, sorted by most recently updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> | If `true`, show notifications marked as read. |  |[default to false]
**participating** | Option<**bool**> | If `true`, only shows notifications in which the user is directly participating or mentioned. |  |[default to false]
**since** | Option<**String**> | Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**before** | Option<**String**> | Only show notifications updated before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 50). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 50]

### Return type

[**Vec<models::Thread>**](thread.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_org_events_for_authenticated_user

> Vec<models::Event> activity_slash_list_org_events_for_authenticated_user(username, org, per_page, page)
List organization events for the authenticated user

This is the user's organization dashboard. You must be authenticated as the user to view this.  > [!NOTE] > This API is not built to serve real-time use cases. Depending on the time of day, event latency can be anywhere from 30s to 6h.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_public_events

> Vec<models::Event> activity_slash_list_public_events(per_page, page)
List public events

> [!NOTE] > This API is not built to serve real-time use cases. Depending on the time of day, event latency can be anywhere from 30s to 6h.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_public_events_for_repo_network

> Vec<models::Event> activity_slash_list_public_events_for_repo_network(owner, repo, per_page, page)
List public events for a network of repositories

> [!NOTE] > This API is not built to serve real-time use cases. Depending on the time of day, event latency can be anywhere from 30s to 6h.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_public_events_for_user

> Vec<models::Event> activity_slash_list_public_events_for_user(username, per_page, page)
List public events for a user

> [!NOTE] > This API is not built to serve real-time use cases. Depending on the time of day, event latency can be anywhere from 30s to 6h.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_public_org_events

> Vec<models::Event> activity_slash_list_public_org_events(org, per_page, page)
List public organization events

> [!NOTE] > This API is not built to serve real-time use cases. Depending on the time of day, event latency can be anywhere from 30s to 6h.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_received_events_for_user

> Vec<models::Event> activity_slash_list_received_events_for_user(username, per_page, page)
List events received by the authenticated user

These are events that you've received by watching repositories and following users. If you are authenticated as the given user, you will see private events. Otherwise, you'll only see public events.  > [!NOTE] > This API is not built to serve real-time use cases. Depending on the time of day, event latency can be anywhere from 30s to 6h.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_received_public_events_for_user

> Vec<models::Event> activity_slash_list_received_public_events_for_user(username, per_page, page)
List public events received by a user

> [!NOTE] > This API is not built to serve real-time use cases. Depending on the time of day, event latency can be anywhere from 30s to 6h.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_repo_events

> Vec<models::Event> activity_slash_list_repo_events(owner, repo, per_page, page)
List repository events

> [!NOTE] > This API is not built to serve real-time use cases. Depending on the time of day, event latency can be anywhere from 30s to 6h.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_repo_notifications_for_authenticated_user

> Vec<models::Thread> activity_slash_list_repo_notifications_for_authenticated_user(owner, repo, all, participating, since, before, per_page, page)
List repository notifications for the authenticated user

Lists all notifications for the current user in the specified repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**all** | Option<**bool**> | If `true`, show notifications marked as read. |  |[default to false]
**participating** | Option<**bool**> | If `true`, only shows notifications in which the user is directly participating or mentioned. |  |[default to false]
**since** | Option<**String**> | Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**before** | Option<**String**> | Only show notifications updated before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Thread>**](thread.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_repos_starred_by_authenticated_user

> Vec<models::Repository> activity_slash_list_repos_starred_by_authenticated_user(sort, direction, per_page, page)
List repositories starred by the authenticated user

Lists repositories the authenticated user has starred.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.star+json`**: Includes a timestamp of when the star was created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort** | Option<**String**> | The property to sort the results by. `created` means when the repository was starred. `updated` means when the repository was last pushed to. |  |[default to created]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Repository>**](repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.github.v3.star+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_repos_starred_by_user

> models::ActivityListReposStarredByUser200Response activity_slash_list_repos_starred_by_user(username, sort, direction, per_page, page)
List repositories starred by a user

Lists repositories a user has starred.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.star+json`**: Includes a timestamp of when the star was created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**sort** | Option<**String**> | The property to sort the results by. `created` means when the repository was starred. `updated` means when the repository was last pushed to. |  |[default to created]
**direction** | Option<**String**> | The direction to sort the results by. |  |[default to desc]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActivityListReposStarredByUser200Response**](activity_list_repos_starred_by_user_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_repos_watched_by_user

> Vec<models::MinimalRepository> activity_slash_list_repos_watched_by_user(username, per_page, page)
List repositories watched by a user

Lists repositories a user is watching.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::MinimalRepository>**](minimal-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_stargazers_for_repo

> models::ActivityListStargazersForRepo200Response activity_slash_list_stargazers_for_repo(owner, repo, per_page, page)
List stargazers

Lists the people that have starred the repository.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).\"  - **`application/vnd.github.star+json`**: Includes a timestamp of when the star was created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::ActivityListStargazersForRepo200Response**](activity_list_stargazers_for_repo_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_watched_repos_for_authenticated_user

> Vec<models::MinimalRepository> activity_slash_list_watched_repos_for_authenticated_user(per_page, page)
List repositories watched by the authenticated user

Lists repositories the authenticated user is watching.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::MinimalRepository>**](minimal-repository.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_list_watchers_for_repo

> Vec<models::SimpleUser> activity_slash_list_watchers_for_repo(owner, repo, per_page, page)
List watchers

Lists the people watching the specified repository.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
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


## activity_slash_mark_notifications_as_read

> models::ActivityMarkNotificationsAsRead202Response activity_slash_mark_notifications_as_read(activity_mark_notifications_as_read_request)
Mark notifications as read

Marks all notifications as \"read\" for the current user. If the number of notifications is too large to complete in one request, you will receive a `202 Accepted` status and GitHub will run an asynchronous process to mark notifications as \"read.\" To check whether any \"unread\" notifications remain, you can use the [List notifications for the authenticated user](https://docs.github.com/rest/activity/notifications#list-notifications-for-the-authenticated-user) endpoint and pass the query parameter `all=false`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**activity_mark_notifications_as_read_request** | Option<[**ActivityMarkNotificationsAsReadRequest**](ActivityMarkNotificationsAsReadRequest.md)> |  |  |

### Return type

[**models::ActivityMarkNotificationsAsRead202Response**](activity_mark_notifications_as_read_202_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_mark_repo_notifications_as_read

> models::ActivityMarkRepoNotificationsAsRead202Response activity_slash_mark_repo_notifications_as_read(owner, repo, activity_mark_repo_notifications_as_read_request)
Mark repository notifications as read

Marks all notifications in a repository as \"read\" for the current user. If the number of notifications is too large to complete in one request, you will receive a `202 Accepted` status and GitHub will run an asynchronous process to mark notifications as \"read.\" To check whether any \"unread\" notifications remain, you can use the [List repository notifications for the authenticated user](https://docs.github.com/rest/activity/notifications#list-repository-notifications-for-the-authenticated-user) endpoint and pass the query parameter `all=false`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**activity_mark_repo_notifications_as_read_request** | Option<[**ActivityMarkRepoNotificationsAsReadRequest**](ActivityMarkRepoNotificationsAsReadRequest.md)> |  |  |

### Return type

[**models::ActivityMarkRepoNotificationsAsRead202Response**](activity_mark_repo_notifications_as_read_202_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_mark_thread_as_done

> activity_slash_mark_thread_as_done(thread_id)
Mark a thread as done

Marks a thread as \"done.\" Marking a thread as \"done\" is equivalent to marking a notification in your notification inbox on GitHub as done: https://github.com/notifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **i32** | The unique identifier of the notification thread. This corresponds to the value returned in the `id` field when you retrieve notifications (for example with the [`GET /notifications` operation](https://docs.github.com/rest/activity/notifications#list-notifications-for-the-authenticated-user)). | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_mark_thread_as_read

> activity_slash_mark_thread_as_read(thread_id)
Mark a thread as read

Marks a thread as \"read.\" Marking a thread as \"read\" is equivalent to clicking a notification in your notification inbox on GitHub: https://github.com/notifications.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **i32** | The unique identifier of the notification thread. This corresponds to the value returned in the `id` field when you retrieve notifications (for example with the [`GET /notifications` operation](https://docs.github.com/rest/activity/notifications#list-notifications-for-the-authenticated-user)). | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_set_repo_subscription

> models::RepositorySubscription activity_slash_set_repo_subscription(owner, repo, activity_set_repo_subscription_request)
Set a repository subscription

If you would like to watch a repository, set `subscribed` to `true`. If you would like to ignore notifications made within a repository, set `ignored` to `true`. If you would like to stop watching a repository, [delete the repository's subscription](https://docs.github.com/rest/activity/watching#delete-a-repository-subscription) completely.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**activity_set_repo_subscription_request** | Option<[**ActivitySetRepoSubscriptionRequest**](ActivitySetRepoSubscriptionRequest.md)> |  |  |

### Return type

[**models::RepositorySubscription**](repository-subscription.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_set_thread_subscription

> models::ThreadSubscription activity_slash_set_thread_subscription(thread_id, activity_set_thread_subscription_request)
Set a thread subscription

If you are watching a repository, you receive notifications for all threads by default. Use this endpoint to ignore future notifications for threads until you comment on the thread or get an **@mention**.  You can also use this endpoint to subscribe to threads that you are currently not receiving notifications for or to subscribed to threads that you have previously ignored.  Unsubscribing from a conversation in a repository that you are not watching is functionally equivalent to the [Delete a thread subscription](https://docs.github.com/rest/activity/notifications#delete-a-thread-subscription) endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_id** | **i32** | The unique identifier of the notification thread. This corresponds to the value returned in the `id` field when you retrieve notifications (for example with the [`GET /notifications` operation](https://docs.github.com/rest/activity/notifications#list-notifications-for-the-authenticated-user)). | [required] |
**activity_set_thread_subscription_request** | Option<[**ActivitySetThreadSubscriptionRequest**](ActivitySetThreadSubscriptionRequest.md)> |  |  |

### Return type

[**models::ThreadSubscription**](thread-subscription.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_star_repo_for_authenticated_user

> activity_slash_star_repo_for_authenticated_user(owner, repo)
Star a repository for the authenticated user

Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method).\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## activity_slash_unstar_repo_for_authenticated_user

> activity_slash_unstar_repo_for_authenticated_user(owner, repo)
Unstar a repository for the authenticated user

Unstar a repository that the authenticated user has previously starred.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

