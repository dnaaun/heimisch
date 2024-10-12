# \AppsApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_slash_add_repo_to_installation_for_authenticated_user**](AppsApi.md#apps_slash_add_repo_to_installation_for_authenticated_user) | **PUT** /user/installations/{installation_id}/repositories/{repository_id} | Add a repository to an app installation
[**apps_slash_check_token**](AppsApi.md#apps_slash_check_token) | **POST** /applications/{client_id}/token | Check a token
[**apps_slash_create_from_manifest**](AppsApi.md#apps_slash_create_from_manifest) | **POST** /app-manifests/{code}/conversions | Create a GitHub App from a manifest
[**apps_slash_create_installation_access_token**](AppsApi.md#apps_slash_create_installation_access_token) | **POST** /app/installations/{installation_id}/access_tokens | Create an installation access token for an app
[**apps_slash_delete_authorization**](AppsApi.md#apps_slash_delete_authorization) | **DELETE** /applications/{client_id}/grant | Delete an app authorization
[**apps_slash_delete_installation**](AppsApi.md#apps_slash_delete_installation) | **DELETE** /app/installations/{installation_id} | Delete an installation for the authenticated app
[**apps_slash_delete_token**](AppsApi.md#apps_slash_delete_token) | **DELETE** /applications/{client_id}/token | Delete an app token
[**apps_slash_get_authenticated**](AppsApi.md#apps_slash_get_authenticated) | **GET** /app | Get the authenticated app
[**apps_slash_get_by_slug**](AppsApi.md#apps_slash_get_by_slug) | **GET** /apps/{app_slug} | Get an app
[**apps_slash_get_installation**](AppsApi.md#apps_slash_get_installation) | **GET** /app/installations/{installation_id} | Get an installation for the authenticated app
[**apps_slash_get_org_installation**](AppsApi.md#apps_slash_get_org_installation) | **GET** /orgs/{org}/installation | Get an organization installation for the authenticated app
[**apps_slash_get_repo_installation**](AppsApi.md#apps_slash_get_repo_installation) | **GET** /repos/{owner}/{repo}/installation | Get a repository installation for the authenticated app
[**apps_slash_get_subscription_plan_for_account**](AppsApi.md#apps_slash_get_subscription_plan_for_account) | **GET** /marketplace_listing/accounts/{account_id} | Get a subscription plan for an account
[**apps_slash_get_subscription_plan_for_account_stubbed**](AppsApi.md#apps_slash_get_subscription_plan_for_account_stubbed) | **GET** /marketplace_listing/stubbed/accounts/{account_id} | Get a subscription plan for an account (stubbed)
[**apps_slash_get_user_installation**](AppsApi.md#apps_slash_get_user_installation) | **GET** /users/{username}/installation | Get a user installation for the authenticated app
[**apps_slash_get_webhook_config_for_app**](AppsApi.md#apps_slash_get_webhook_config_for_app) | **GET** /app/hook/config | Get a webhook configuration for an app
[**apps_slash_get_webhook_delivery**](AppsApi.md#apps_slash_get_webhook_delivery) | **GET** /app/hook/deliveries/{delivery_id} | Get a delivery for an app webhook
[**apps_slash_list_accounts_for_plan**](AppsApi.md#apps_slash_list_accounts_for_plan) | **GET** /marketplace_listing/plans/{plan_id}/accounts | List accounts for a plan
[**apps_slash_list_accounts_for_plan_stubbed**](AppsApi.md#apps_slash_list_accounts_for_plan_stubbed) | **GET** /marketplace_listing/stubbed/plans/{plan_id}/accounts | List accounts for a plan (stubbed)
[**apps_slash_list_installation_repos_for_authenticated_user**](AppsApi.md#apps_slash_list_installation_repos_for_authenticated_user) | **GET** /user/installations/{installation_id}/repositories | List repositories accessible to the user access token
[**apps_slash_list_installation_requests_for_authenticated_app**](AppsApi.md#apps_slash_list_installation_requests_for_authenticated_app) | **GET** /app/installation-requests | List installation requests for the authenticated app
[**apps_slash_list_installations**](AppsApi.md#apps_slash_list_installations) | **GET** /app/installations | List installations for the authenticated app
[**apps_slash_list_installations_for_authenticated_user**](AppsApi.md#apps_slash_list_installations_for_authenticated_user) | **GET** /user/installations | List app installations accessible to the user access token
[**apps_slash_list_plans**](AppsApi.md#apps_slash_list_plans) | **GET** /marketplace_listing/plans | List plans
[**apps_slash_list_plans_stubbed**](AppsApi.md#apps_slash_list_plans_stubbed) | **GET** /marketplace_listing/stubbed/plans | List plans (stubbed)
[**apps_slash_list_repos_accessible_to_installation**](AppsApi.md#apps_slash_list_repos_accessible_to_installation) | **GET** /installation/repositories | List repositories accessible to the app installation
[**apps_slash_list_subscriptions_for_authenticated_user**](AppsApi.md#apps_slash_list_subscriptions_for_authenticated_user) | **GET** /user/marketplace_purchases | List subscriptions for the authenticated user
[**apps_slash_list_subscriptions_for_authenticated_user_stubbed**](AppsApi.md#apps_slash_list_subscriptions_for_authenticated_user_stubbed) | **GET** /user/marketplace_purchases/stubbed | List subscriptions for the authenticated user (stubbed)
[**apps_slash_list_webhook_deliveries**](AppsApi.md#apps_slash_list_webhook_deliveries) | **GET** /app/hook/deliveries | List deliveries for an app webhook
[**apps_slash_redeliver_webhook_delivery**](AppsApi.md#apps_slash_redeliver_webhook_delivery) | **POST** /app/hook/deliveries/{delivery_id}/attempts | Redeliver a delivery for an app webhook
[**apps_slash_remove_repo_from_installation_for_authenticated_user**](AppsApi.md#apps_slash_remove_repo_from_installation_for_authenticated_user) | **DELETE** /user/installations/{installation_id}/repositories/{repository_id} | Remove a repository from an app installation
[**apps_slash_reset_token**](AppsApi.md#apps_slash_reset_token) | **PATCH** /applications/{client_id}/token | Reset a token
[**apps_slash_revoke_installation_access_token**](AppsApi.md#apps_slash_revoke_installation_access_token) | **DELETE** /installation/token | Revoke an installation access token
[**apps_slash_scope_token**](AppsApi.md#apps_slash_scope_token) | **POST** /applications/{client_id}/token/scoped | Create a scoped access token
[**apps_slash_suspend_installation**](AppsApi.md#apps_slash_suspend_installation) | **PUT** /app/installations/{installation_id}/suspended | Suspend an app installation
[**apps_slash_unsuspend_installation**](AppsApi.md#apps_slash_unsuspend_installation) | **DELETE** /app/installations/{installation_id}/suspended | Unsuspend an app installation
[**apps_slash_update_webhook_config_for_app**](AppsApi.md#apps_slash_update_webhook_config_for_app) | **PATCH** /app/hook/config | Update a webhook configuration for an app



## apps_slash_add_repo_to_installation_for_authenticated_user

> apps_slash_add_repo_to_installation_for_authenticated_user(installation_id, repository_id)
Add a repository to an app installation

Add a single repository to an installation. The authenticated user must have admin access to the repository.      This endpoint only works for PATs (classic) with the `repo` scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**installation_id** | **i32** | The unique identifier of the installation. | [required] |
**repository_id** | **i32** | The unique identifier of the repository. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_check_token

> models::Authorization apps_slash_check_token(client_id, apps_check_token_request)
Check a token

OAuth applications and GitHub applications with OAuth authorizations can use this API method for checking OAuth token validity without exceeding the normal rate limits for failed login attempts. Authentication works differently with this particular endpoint. Invalid tokens will return `404 NOT FOUND`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client ID of the GitHub app. | [required] |
**apps_check_token_request** | [**AppsCheckTokenRequest**](AppsCheckTokenRequest.md) |  | [required] |

### Return type

[**models::Authorization**](authorization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_create_from_manifest

> models::AppsCreateFromManifest201Response apps_slash_create_from_manifest(code)
Create a GitHub App from a manifest

Use this endpoint to complete the handshake necessary when implementing the [GitHub App Manifest flow](https://docs.github.com/apps/building-github-apps/creating-github-apps-from-a-manifest/). When you create a GitHub App with the manifest flow, you receive a temporary `code` used to retrieve the GitHub App's `id`, `pem` (private key), and `webhook_secret`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** |  | [required] |

### Return type

[**models::AppsCreateFromManifest201Response**](apps_create_from_manifest_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_create_installation_access_token

> models::InstallationToken apps_slash_create_installation_access_token(installation_id, apps_create_installation_access_token_request)
Create an installation access token for an app

Creates an installation access token that enables a GitHub App to make authenticated API requests for the app's installation on an organization or individual account. Installation tokens expire one hour from the time you create them. Using an expired token produces a status code of `401 - Unauthorized`, and requires creating a new installation token. By default the installation token has access to all repositories that the installation can access.  Optionally, you can use the `repositories` or `repository_ids` body parameters to specify individual repositories that the installation access token can access. If you don't use `repositories` or `repository_ids` to grant access to specific repositories, the installation access token will have access to all repositories that the installation was granted access to. The installation access token cannot be granted access to repositories that the installation was not granted access to. Up to 500 repositories can be listed in this manner.  Optionally, use the `permissions` body parameter to specify the permissions that the installation access token should have. If `permissions` is not specified, the installation access token will have all of the permissions that were granted to the app. The installation access token cannot be granted permissions that the app was not granted.  When using the repository or permission parameters to reduce the access of the token, the complexity of the token is increased due to both the number of permissions in the request and the number of repositories the token will have access to. If the complexity is too large, the token will fail to be issued. If this occurs, the error message will indicate the maximum number of repositories that should be requested. For the average application requesting 8 permissions, this limit is around 5000 repositories. With fewer permissions requested, more repositories are supported.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**installation_id** | **i32** | The unique identifier of the installation. | [required] |
**apps_create_installation_access_token_request** | Option<[**AppsCreateInstallationAccessTokenRequest**](AppsCreateInstallationAccessTokenRequest.md)> |  |  |

### Return type

[**models::InstallationToken**](installation-token.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_delete_authorization

> apps_slash_delete_authorization(client_id, apps_delete_authorization_request)
Delete an app authorization

OAuth and GitHub application owners can revoke a grant for their application and a specific user. You must provide a valid OAuth `access_token` as an input parameter and the grant for the token's owner will be deleted. Deleting an application's grant will also delete all OAuth tokens associated with the application for the user. Once deleted, the application will have no access to the user's account and will no longer be listed on [the application authorizations settings screen within GitHub](https://github.com/settings/applications#authorized).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client ID of the GitHub app. | [required] |
**apps_delete_authorization_request** | [**AppsDeleteAuthorizationRequest**](AppsDeleteAuthorizationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_delete_installation

> apps_slash_delete_installation(installation_id)
Delete an installation for the authenticated app

Uninstalls a GitHub App on a user, organization, or business account. If you prefer to temporarily suspend an app's access to your account's resources, then we recommend the \"[Suspend an app installation](https://docs.github.com/rest/apps/apps#suspend-an-app-installation)\" endpoint.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**installation_id** | **i32** | The unique identifier of the installation. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_delete_token

> apps_slash_delete_token(client_id, apps_delete_authorization_request)
Delete an app token

OAuth  or GitHub application owners can revoke a single token for an OAuth application or a GitHub application with an OAuth authorization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client ID of the GitHub app. | [required] |
**apps_delete_authorization_request** | [**AppsDeleteAuthorizationRequest**](AppsDeleteAuthorizationRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_get_authenticated

> models::Integration apps_slash_get_authenticated()
Get the authenticated app

Returns the GitHub App associated with the authentication credentials used. To see how many app installations are associated with this GitHub App, see the `installations_count` in the response. For more details about your app's installations, see the \"[List installations for the authenticated app](https://docs.github.com/rest/apps/apps#list-installations-for-the-authenticated-app)\" endpoint.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Integration**](integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_get_by_slug

> models::Integration apps_slash_get_by_slug(app_slug)
Get an app

> [!NOTE] > The `:app_slug` is just the URL-friendly name of your GitHub App. You can find this on the settings page for your GitHub App (e.g., `https://github.com/settings/apps/:app_slug`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_slug** | **String** |  | [required] |

### Return type

[**models::Integration**](integration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_get_installation

> models::Installation apps_slash_get_installation(installation_id)
Get an installation for the authenticated app

Enables an authenticated GitHub App to find an installation's information using the installation id.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**installation_id** | **i32** | The unique identifier of the installation. | [required] |

### Return type

[**models::Installation**](installation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_get_org_installation

> models::Installation apps_slash_get_org_installation(org)
Get an organization installation for the authenticated app

Enables an authenticated GitHub App to find the organization's installation information.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |

### Return type

[**models::Installation**](installation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_get_repo_installation

> models::Installation apps_slash_get_repo_installation(owner, repo)
Get a repository installation for the authenticated app

Enables an authenticated GitHub App to find the repository's installation information. The installation's account type will be either an organization or a user account, depending which account the repository belongs to.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::Installation**](installation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_get_subscription_plan_for_account

> models::MarketplacePurchase apps_slash_get_subscription_plan_for_account(account_id)
Get a subscription plan for an account

Shows whether the user or organization account actively subscribes to a plan listed by the authenticated GitHub App. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.  GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | account_id parameter | [required] |

### Return type

[**models::MarketplacePurchase**](marketplace-purchase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_get_subscription_plan_for_account_stubbed

> models::MarketplacePurchase apps_slash_get_subscription_plan_for_account_stubbed(account_id)
Get a subscription plan for an account (stubbed)

Shows whether the user or organization account actively subscribes to a plan listed by the authenticated GitHub App. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.  GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | account_id parameter | [required] |

### Return type

[**models::MarketplacePurchase**](marketplace-purchase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_get_user_installation

> models::Installation apps_slash_get_user_installation(username)
Get a user installation for the authenticated app

Enables an authenticated GitHub App to find the user’s installation information.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::Installation**](installation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_get_webhook_config_for_app

> models::WebhookConfig apps_slash_get_webhook_config_for_app()
Get a webhook configuration for an app

Returns the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see \"[Creating a GitHub App](/developers/apps/creating-a-github-app).\"  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::WebhookConfig**](webhook-config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_get_webhook_delivery

> models::HookDelivery apps_slash_get_webhook_delivery(delivery_id)
Get a delivery for an app webhook

Returns a delivery for the webhook configured for a GitHub App.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_id** | **i32** |  | [required] |

### Return type

[**models::HookDelivery**](hook-delivery.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_accounts_for_plan

> Vec<models::MarketplacePurchase> apps_slash_list_accounts_for_plan(plan_id, sort, direction, per_page, page)
List accounts for a plan

Returns user and organization accounts associated with the specified plan, including free plans. For per-seat pricing, you see the list of accounts that have purchased the plan, including the number of seats purchased. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.  GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i32** | The unique identifier of the plan. | [required] |
**sort** | Option<**String**> | The property to sort the results by. |  |[default to created]
**direction** | Option<**String**> | To return the oldest accounts first, set to `asc`. Ignored without the `sort` parameter. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::MarketplacePurchase>**](marketplace-purchase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_accounts_for_plan_stubbed

> Vec<models::MarketplacePurchase> apps_slash_list_accounts_for_plan_stubbed(plan_id, sort, direction, per_page, page)
List accounts for a plan (stubbed)

Returns repository and organization accounts associated with the specified plan, including free plans. For per-seat pricing, you see the list of accounts that have purchased the plan, including the number of seats purchased. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.  GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **i32** | The unique identifier of the plan. | [required] |
**sort** | Option<**String**> | The property to sort the results by. |  |[default to created]
**direction** | Option<**String**> | To return the oldest accounts first, set to `asc`. Ignored without the `sort` parameter. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::MarketplacePurchase>**](marketplace-purchase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_installation_repos_for_authenticated_user

> models::AppsListInstallationReposForAuthenticatedUser200Response apps_slash_list_installation_repos_for_authenticated_user(installation_id, per_page, page)
List repositories accessible to the user access token

List repositories that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access for an installation.  The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.  The access the user has to each repository is included in the hash under the `permissions` key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**installation_id** | **i32** | The unique identifier of the installation. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::AppsListInstallationReposForAuthenticatedUser200Response**](apps_list_installation_repos_for_authenticated_user_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_installation_requests_for_authenticated_app

> Vec<models::IntegrationInstallationRequest> apps_slash_list_installation_requests_for_authenticated_app(per_page, page)
List installation requests for the authenticated app

Lists all the pending installation requests for the authenticated GitHub App.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::IntegrationInstallationRequest>**](integration-installation-request.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_installations

> Vec<models::Installation> apps_slash_list_installations(per_page, page, since, outdated)
List installations for the authenticated app

The permissions the installation has are included under the `permissions` key.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**since** | Option<**String**> | Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. |  |
**outdated** | Option<**String**> |  |  |

### Return type

[**Vec<models::Installation>**](installation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_installations_for_authenticated_user

> models::OrgsListAppInstallations200Response apps_slash_list_installations_for_authenticated_user(per_page, page)
List app installations accessible to the user access token

Lists installations of your GitHub App that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access.  The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.  You can find the permissions for the installation under the `permissions` key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::OrgsListAppInstallations200Response**](orgs_list_app_installations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_plans

> Vec<models::MarketplaceListingPlan> apps_slash_list_plans(per_page, page)
List plans

Lists all plans that are part of your GitHub Marketplace listing.  GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::MarketplaceListingPlan>**](marketplace-listing-plan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_plans_stubbed

> Vec<models::MarketplaceListingPlan> apps_slash_list_plans_stubbed(per_page, page)
List plans (stubbed)

Lists all plans that are part of your GitHub Marketplace listing.  GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::MarketplaceListingPlan>**](marketplace-listing-plan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_repos_accessible_to_installation

> models::AppsListReposAccessibleToInstallation200Response apps_slash_list_repos_accessible_to_installation(per_page, page)
List repositories accessible to the app installation

List repositories that an app installation can access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**models::AppsListReposAccessibleToInstallation200Response**](apps_list_repos_accessible_to_installation_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_subscriptions_for_authenticated_user

> Vec<models::UserMarketplacePurchase> apps_slash_list_subscriptions_for_authenticated_user(per_page, page)
List subscriptions for the authenticated user

Lists the active subscriptions for the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::UserMarketplacePurchase>**](user-marketplace-purchase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_subscriptions_for_authenticated_user_stubbed

> Vec<models::UserMarketplacePurchase> apps_slash_list_subscriptions_for_authenticated_user_stubbed(per_page, page)
List subscriptions for the authenticated user (stubbed)

Lists the active subscriptions for the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::UserMarketplacePurchase>**](user-marketplace-purchase.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_list_webhook_deliveries

> Vec<models::HookDeliveryItem> apps_slash_list_webhook_deliveries(per_page, cursor)
List deliveries for an app webhook

Returns a list of webhook deliveries for the webhook configured for a GitHub App.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**cursor** | Option<**String**> | Used for pagination: the starting delivery from which the page of deliveries is fetched. Refer to the `link` header for the next and previous page cursors. |  |

### Return type

[**Vec<models::HookDeliveryItem>**](hook-delivery-item.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_redeliver_webhook_delivery

> serde_json::Value apps_slash_redeliver_webhook_delivery(delivery_id)
Redeliver a delivery for an app webhook

Redeliver a delivery for the webhook configured for a GitHub App.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delivery_id** | **i32** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/scim+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_remove_repo_from_installation_for_authenticated_user

> apps_slash_remove_repo_from_installation_for_authenticated_user(installation_id, repository_id)
Remove a repository from an app installation

Remove a single repository from an installation. The authenticated user must have admin access to the repository. The installation must have the `repository_selection` of `selected`.   This endpoint only works for PATs (classic) with the `repo` scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**installation_id** | **i32** | The unique identifier of the installation. | [required] |
**repository_id** | **i32** | The unique identifier of the repository. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_reset_token

> models::Authorization apps_slash_reset_token(client_id, apps_check_token_request)
Reset a token

OAuth applications and GitHub applications with OAuth authorizations can use this API method to reset a valid OAuth token without end-user involvement. Applications must save the \"token\" property in the response because changes take effect immediately. Invalid tokens will return `404 NOT FOUND`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client ID of the GitHub app. | [required] |
**apps_check_token_request** | [**AppsCheckTokenRequest**](AppsCheckTokenRequest.md) |  | [required] |

### Return type

[**models::Authorization**](authorization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_revoke_installation_access_token

> apps_slash_revoke_installation_access_token()
Revoke an installation access token

Revokes the installation token you're using to authenticate as an installation and access this endpoint.  Once an installation token is revoked, the token is invalidated and cannot be used. Other endpoints that require the revoked installation token must have a new installation token to work. You can create a new token using the \"[Create an installation access token for an app](https://docs.github.com/rest/apps/apps#create-an-installation-access-token-for-an-app)\" endpoint.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_scope_token

> models::Authorization apps_slash_scope_token(client_id, apps_scope_token_request)
Create a scoped access token

Use a non-scoped user access token to create a repository-scoped and/or permission-scoped user access token. You can specify which repositories the token can access and which permissions are granted to the token.  Invalid tokens will return `404 NOT FOUND`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client ID of the GitHub app. | [required] |
**apps_scope_token_request** | [**AppsScopeTokenRequest**](AppsScopeTokenRequest.md) |  | [required] |

### Return type

[**models::Authorization**](authorization.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_suspend_installation

> apps_slash_suspend_installation(installation_id)
Suspend an app installation

Suspends a GitHub App on a user, organization, or business account, which blocks the app from accessing the account's resources. When a GitHub App is suspended, the app's access to the GitHub API or webhook events is blocked for that account.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**installation_id** | **i32** | The unique identifier of the installation. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_unsuspend_installation

> apps_slash_unsuspend_installation(installation_id)
Unsuspend an app installation

Removes a GitHub App installation suspension.  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**installation_id** | **i32** | The unique identifier of the installation. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_slash_update_webhook_config_for_app

> models::WebhookConfig apps_slash_update_webhook_config_for_app(apps_update_webhook_config_for_app_request)
Update a webhook configuration for an app

Updates the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see \"[Creating a GitHub App](/developers/apps/creating-a-github-app).\"  You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apps_update_webhook_config_for_app_request** | [**AppsUpdateWebhookConfigForAppRequest**](AppsUpdateWebhookConfigForAppRequest.md) |  | [required] |

### Return type

[**models::WebhookConfig**](webhook-config.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

