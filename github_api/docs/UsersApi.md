# \UsersApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_slash_add_email_for_authenticated_user**](UsersApi.md#users_slash_add_email_for_authenticated_user) | **POST** /user/emails | Add an email address for the authenticated user
[**users_slash_add_social_account_for_authenticated_user**](UsersApi.md#users_slash_add_social_account_for_authenticated_user) | **POST** /user/social_accounts | Add social accounts for the authenticated user
[**users_slash_block**](UsersApi.md#users_slash_block) | **PUT** /user/blocks/{username} | Block a user
[**users_slash_check_blocked**](UsersApi.md#users_slash_check_blocked) | **GET** /user/blocks/{username} | Check if a user is blocked by the authenticated user
[**users_slash_check_following_for_user**](UsersApi.md#users_slash_check_following_for_user) | **GET** /users/{username}/following/{target_user} | Check if a user follows another user
[**users_slash_check_person_is_followed_by_authenticated**](UsersApi.md#users_slash_check_person_is_followed_by_authenticated) | **GET** /user/following/{username} | Check if a person is followed by the authenticated user
[**users_slash_create_gpg_key_for_authenticated_user**](UsersApi.md#users_slash_create_gpg_key_for_authenticated_user) | **POST** /user/gpg_keys | Create a GPG key for the authenticated user
[**users_slash_create_public_ssh_key_for_authenticated_user**](UsersApi.md#users_slash_create_public_ssh_key_for_authenticated_user) | **POST** /user/keys | Create a public SSH key for the authenticated user
[**users_slash_create_ssh_signing_key_for_authenticated_user**](UsersApi.md#users_slash_create_ssh_signing_key_for_authenticated_user) | **POST** /user/ssh_signing_keys | Create a SSH signing key for the authenticated user
[**users_slash_delete_email_for_authenticated_user**](UsersApi.md#users_slash_delete_email_for_authenticated_user) | **DELETE** /user/emails | Delete an email address for the authenticated user
[**users_slash_delete_gpg_key_for_authenticated_user**](UsersApi.md#users_slash_delete_gpg_key_for_authenticated_user) | **DELETE** /user/gpg_keys/{gpg_key_id} | Delete a GPG key for the authenticated user
[**users_slash_delete_public_ssh_key_for_authenticated_user**](UsersApi.md#users_slash_delete_public_ssh_key_for_authenticated_user) | **DELETE** /user/keys/{key_id} | Delete a public SSH key for the authenticated user
[**users_slash_delete_social_account_for_authenticated_user**](UsersApi.md#users_slash_delete_social_account_for_authenticated_user) | **DELETE** /user/social_accounts | Delete social accounts for the authenticated user
[**users_slash_delete_ssh_signing_key_for_authenticated_user**](UsersApi.md#users_slash_delete_ssh_signing_key_for_authenticated_user) | **DELETE** /user/ssh_signing_keys/{ssh_signing_key_id} | Delete an SSH signing key for the authenticated user
[**users_slash_follow**](UsersApi.md#users_slash_follow) | **PUT** /user/following/{username} | Follow a user
[**users_slash_get_authenticated**](UsersApi.md#users_slash_get_authenticated) | **GET** /user | Get the authenticated user
[**users_slash_get_by_id**](UsersApi.md#users_slash_get_by_id) | **GET** /user/{account_id} | Get a user using their ID
[**users_slash_get_by_username**](UsersApi.md#users_slash_get_by_username) | **GET** /users/{username} | Get a user
[**users_slash_get_context_for_user**](UsersApi.md#users_slash_get_context_for_user) | **GET** /users/{username}/hovercard | Get contextual information for a user
[**users_slash_get_gpg_key_for_authenticated_user**](UsersApi.md#users_slash_get_gpg_key_for_authenticated_user) | **GET** /user/gpg_keys/{gpg_key_id} | Get a GPG key for the authenticated user
[**users_slash_get_public_ssh_key_for_authenticated_user**](UsersApi.md#users_slash_get_public_ssh_key_for_authenticated_user) | **GET** /user/keys/{key_id} | Get a public SSH key for the authenticated user
[**users_slash_get_ssh_signing_key_for_authenticated_user**](UsersApi.md#users_slash_get_ssh_signing_key_for_authenticated_user) | **GET** /user/ssh_signing_keys/{ssh_signing_key_id} | Get an SSH signing key for the authenticated user
[**users_slash_list**](UsersApi.md#users_slash_list) | **GET** /users | List users
[**users_slash_list_attestations**](UsersApi.md#users_slash_list_attestations) | **GET** /users/{username}/attestations/{subject_digest} | List attestations
[**users_slash_list_blocked_by_authenticated_user**](UsersApi.md#users_slash_list_blocked_by_authenticated_user) | **GET** /user/blocks | List users blocked by the authenticated user
[**users_slash_list_emails_for_authenticated_user**](UsersApi.md#users_slash_list_emails_for_authenticated_user) | **GET** /user/emails | List email addresses for the authenticated user
[**users_slash_list_followed_by_authenticated_user**](UsersApi.md#users_slash_list_followed_by_authenticated_user) | **GET** /user/following | List the people the authenticated user follows
[**users_slash_list_followers_for_authenticated_user**](UsersApi.md#users_slash_list_followers_for_authenticated_user) | **GET** /user/followers | List followers of the authenticated user
[**users_slash_list_followers_for_user**](UsersApi.md#users_slash_list_followers_for_user) | **GET** /users/{username}/followers | List followers of a user
[**users_slash_list_following_for_user**](UsersApi.md#users_slash_list_following_for_user) | **GET** /users/{username}/following | List the people a user follows
[**users_slash_list_gpg_keys_for_authenticated_user**](UsersApi.md#users_slash_list_gpg_keys_for_authenticated_user) | **GET** /user/gpg_keys | List GPG keys for the authenticated user
[**users_slash_list_gpg_keys_for_user**](UsersApi.md#users_slash_list_gpg_keys_for_user) | **GET** /users/{username}/gpg_keys | List GPG keys for a user
[**users_slash_list_public_emails_for_authenticated_user**](UsersApi.md#users_slash_list_public_emails_for_authenticated_user) | **GET** /user/public_emails | List public email addresses for the authenticated user
[**users_slash_list_public_keys_for_user**](UsersApi.md#users_slash_list_public_keys_for_user) | **GET** /users/{username}/keys | List public keys for a user
[**users_slash_list_public_ssh_keys_for_authenticated_user**](UsersApi.md#users_slash_list_public_ssh_keys_for_authenticated_user) | **GET** /user/keys | List public SSH keys for the authenticated user
[**users_slash_list_social_accounts_for_authenticated_user**](UsersApi.md#users_slash_list_social_accounts_for_authenticated_user) | **GET** /user/social_accounts | List social accounts for the authenticated user
[**users_slash_list_social_accounts_for_user**](UsersApi.md#users_slash_list_social_accounts_for_user) | **GET** /users/{username}/social_accounts | List social accounts for a user
[**users_slash_list_ssh_signing_keys_for_authenticated_user**](UsersApi.md#users_slash_list_ssh_signing_keys_for_authenticated_user) | **GET** /user/ssh_signing_keys | List SSH signing keys for the authenticated user
[**users_slash_list_ssh_signing_keys_for_user**](UsersApi.md#users_slash_list_ssh_signing_keys_for_user) | **GET** /users/{username}/ssh_signing_keys | List SSH signing keys for a user
[**users_slash_set_primary_email_visibility_for_authenticated_user**](UsersApi.md#users_slash_set_primary_email_visibility_for_authenticated_user) | **PATCH** /user/email/visibility | Set primary email visibility for the authenticated user
[**users_slash_unblock**](UsersApi.md#users_slash_unblock) | **DELETE** /user/blocks/{username} | Unblock a user
[**users_slash_unfollow**](UsersApi.md#users_slash_unfollow) | **DELETE** /user/following/{username} | Unfollow a user
[**users_slash_update_authenticated**](UsersApi.md#users_slash_update_authenticated) | **PATCH** /user | Update the authenticated user



## users_slash_add_email_for_authenticated_user

> Vec<models::Email> users_slash_add_email_for_authenticated_user(users_add_email_for_authenticated_user_request)
Add an email address for the authenticated user

OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_add_email_for_authenticated_user_request** | Option<[**UsersAddEmailForAuthenticatedUserRequest**](UsersAddEmailForAuthenticatedUserRequest.md)> |  |  |

### Return type

[**Vec<models::Email>**](email.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_add_social_account_for_authenticated_user

> Vec<models::SocialAccount> users_slash_add_social_account_for_authenticated_user(users_add_social_account_for_authenticated_user_request)
Add social accounts for the authenticated user

Add one or more social accounts to the authenticated user's profile.  OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_add_social_account_for_authenticated_user_request** | [**UsersAddSocialAccountForAuthenticatedUserRequest**](UsersAddSocialAccountForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**Vec<models::SocialAccount>**](social-account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_block

> users_slash_block(username)
Block a user

Blocks the given user and returns a 204. If the authenticated user cannot block the given user a 422 is returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_check_blocked

> users_slash_check_blocked(username)
Check if a user is blocked by the authenticated user

Returns a 204 if the given user is blocked by the authenticated user. Returns a 404 if the given user is not blocked by the authenticated user, or if the given user account has been identified as spam by GitHub.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_check_following_for_user

> users_slash_check_following_for_user(username, target_user)
Check if a user follows another user



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**target_user** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_check_person_is_followed_by_authenticated

> users_slash_check_person_is_followed_by_authenticated(username)
Check if a person is followed by the authenticated user



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_create_gpg_key_for_authenticated_user

> models::GpgKey users_slash_create_gpg_key_for_authenticated_user(users_create_gpg_key_for_authenticated_user_request)
Create a GPG key for the authenticated user

Adds a GPG key to the authenticated user's GitHub account.  OAuth app tokens and personal access tokens (classic) need the `write:gpg_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_create_gpg_key_for_authenticated_user_request** | [**UsersCreateGpgKeyForAuthenticatedUserRequest**](UsersCreateGpgKeyForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**models::GpgKey**](gpg-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_create_public_ssh_key_for_authenticated_user

> models::Key users_slash_create_public_ssh_key_for_authenticated_user(users_create_public_ssh_key_for_authenticated_user_request)
Create a public SSH key for the authenticated user

Adds a public SSH key to the authenticated user's GitHub account.  OAuth app tokens and personal access tokens (classic) need the `write:gpg_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_create_public_ssh_key_for_authenticated_user_request** | [**UsersCreatePublicSshKeyForAuthenticatedUserRequest**](UsersCreatePublicSshKeyForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**models::Key**](key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_create_ssh_signing_key_for_authenticated_user

> models::SshSigningKey users_slash_create_ssh_signing_key_for_authenticated_user(users_create_ssh_signing_key_for_authenticated_user_request)
Create a SSH signing key for the authenticated user

Creates an SSH signing key for the authenticated user's GitHub account.  OAuth app tokens and personal access tokens (classic) need the `write:ssh_signing_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_create_ssh_signing_key_for_authenticated_user_request** | [**UsersCreateSshSigningKeyForAuthenticatedUserRequest**](UsersCreateSshSigningKeyForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**models::SshSigningKey**](ssh-signing-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_delete_email_for_authenticated_user

> users_slash_delete_email_for_authenticated_user(users_delete_email_for_authenticated_user_request)
Delete an email address for the authenticated user

OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_delete_email_for_authenticated_user_request** | Option<[**UsersDeleteEmailForAuthenticatedUserRequest**](UsersDeleteEmailForAuthenticatedUserRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_delete_gpg_key_for_authenticated_user

> users_slash_delete_gpg_key_for_authenticated_user(gpg_key_id)
Delete a GPG key for the authenticated user

Removes a GPG key from the authenticated user's GitHub account.  OAuth app tokens and personal access tokens (classic) need the `admin:gpg_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gpg_key_id** | **i32** | The unique identifier of the GPG key. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_delete_public_ssh_key_for_authenticated_user

> users_slash_delete_public_ssh_key_for_authenticated_user(key_id)
Delete a public SSH key for the authenticated user

Removes a public SSH key from the authenticated user's GitHub account.  OAuth app tokens and personal access tokens (classic) need the `admin:public_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **i32** | The unique identifier of the key. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_delete_social_account_for_authenticated_user

> users_slash_delete_social_account_for_authenticated_user(users_delete_social_account_for_authenticated_user_request)
Delete social accounts for the authenticated user

Deletes one or more social accounts from the authenticated user's profile.  OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_delete_social_account_for_authenticated_user_request** | [**UsersDeleteSocialAccountForAuthenticatedUserRequest**](UsersDeleteSocialAccountForAuthenticatedUserRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_delete_ssh_signing_key_for_authenticated_user

> users_slash_delete_ssh_signing_key_for_authenticated_user(ssh_signing_key_id)
Delete an SSH signing key for the authenticated user

Deletes an SSH signing key from the authenticated user's GitHub account.  OAuth app tokens and personal access tokens (classic) need the `admin:ssh_signing_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_signing_key_id** | **i32** | The unique identifier of the SSH signing key. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_follow

> users_slash_follow(username)
Follow a user

Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see \"[HTTP verbs](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method).\"  OAuth app tokens and personal access tokens (classic) need the `user:follow` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_get_authenticated

> models::UsersGetAuthenticated200Response users_slash_get_authenticated()
Get the authenticated user

OAuth app tokens and personal access tokens (classic) need the `user` scope in order for the response to include private profile information.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UsersGetAuthenticated200Response**](users_get_authenticated_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_get_by_id

> models::UsersGetAuthenticated200Response users_slash_get_by_id(account_id)
Get a user using their ID

Provides publicly available information about someone with a GitHub account. This method takes their durable user `ID` instead of their `login`, which can change over time.  The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be “public” which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#authentication).  The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see \"[Emails API](https://docs.github.com/rest/users/emails)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **i32** | account_id parameter | [required] |

### Return type

[**models::UsersGetAuthenticated200Response**](users_get_authenticated_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_get_by_username

> models::UsersGetAuthenticated200Response users_slash_get_by_username(username)
Get a user

Provides publicly available information about someone with a GitHub account.  The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be “public” which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#authentication).  The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see \"[Emails API](https://docs.github.com/rest/users/emails)\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

[**models::UsersGetAuthenticated200Response**](users_get_authenticated_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_get_context_for_user

> models::Hovercard users_slash_get_context_for_user(username, subject_type, subject_id)
Get contextual information for a user

Provides hovercard information. You can find out more about someone in relation to their pull requests, issues, repositories, and organizations.    The `subject_type` and `subject_id` parameters provide context for the person's hovercard, which returns more information than without the parameters. For example, if you wanted to find out more about `octocat` who owns the `Spoon-Knife` repository, you would use a `subject_type` value of `repository` and a `subject_id` value of `1300192` (the ID of the `Spoon-Knife` repository).  OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**subject_type** | Option<**String**> | Identifies which additional information you'd like to receive about the person's hovercard. Can be `organization`, `repository`, `issue`, `pull_request`. **Required** when using `subject_id`. |  |
**subject_id** | Option<**String**> | Uses the ID for the `subject_type` you specified. **Required** when using `subject_type`. |  |

### Return type

[**models::Hovercard**](hovercard.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_get_gpg_key_for_authenticated_user

> models::GpgKey users_slash_get_gpg_key_for_authenticated_user(gpg_key_id)
Get a GPG key for the authenticated user

View extended details for a single GPG key.  OAuth app tokens and personal access tokens (classic) need the `read:gpg_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**gpg_key_id** | **i32** | The unique identifier of the GPG key. | [required] |

### Return type

[**models::GpgKey**](gpg-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_get_public_ssh_key_for_authenticated_user

> models::Key users_slash_get_public_ssh_key_for_authenticated_user(key_id)
Get a public SSH key for the authenticated user

View extended details for a single public SSH key.  OAuth app tokens and personal access tokens (classic) need the `read:public_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **i32** | The unique identifier of the key. | [required] |

### Return type

[**models::Key**](key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_get_ssh_signing_key_for_authenticated_user

> models::SshSigningKey users_slash_get_ssh_signing_key_for_authenticated_user(ssh_signing_key_id)
Get an SSH signing key for the authenticated user

Gets extended details for an SSH signing key.  OAuth app tokens and personal access tokens (classic) need the `read:ssh_signing_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ssh_signing_key_id** | **i32** | The unique identifier of the SSH signing key. | [required] |

### Return type

[**models::SshSigningKey**](ssh-signing-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list

> Vec<models::SimpleUser> users_slash_list(since, per_page)
List users

Lists all users, in the order that they signed up on GitHub. This list includes personal user accounts and organization accounts.  Note: Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**since** | Option<**i32**> | A user ID. Only return users with an ID greater than this ID. |  |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**Vec<models::SimpleUser>**](simple-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_attestations

> models::UsersListAttestations200Response users_slash_list_attestations(username, subject_digest, per_page, before, after)
List attestations

List a collection of artifact attestations with a given subject digest that are associated with repositories owned by a user.  The collection of attestations returned by this endpoint is filtered according to the authenticated user's permissions; if the authenticated user cannot read a repository, the attestations associated with that repository will not be included in the response. In addition, when using a fine-grained access token the `attestations:read` permission is required.  **Please note:** in order to offer meaningful security benefits, an attestation's signature and timestamps **must** be cryptographically verified, and the identity of the attestation signer **must** be validated. Attestations can be verified using the [GitHub CLI `attestation verify` command](https://cli.github.com/manual/gh_attestation_verify). For more information, see [our guide on how to use artifact attestations to establish a build's provenance](https://docs.github.com/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**subject_digest** | **String** | Subject Digest | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**before** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |
**after** | Option<**String**> | A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |

### Return type

[**models::UsersListAttestations200Response**](users_list_attestations_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_blocked_by_authenticated_user

> Vec<models::SimpleUser> users_slash_list_blocked_by_authenticated_user(per_page, page)
List users blocked by the authenticated user

List the users you've blocked on your personal account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## users_slash_list_emails_for_authenticated_user

> Vec<models::Email> users_slash_list_emails_for_authenticated_user(per_page, page)
List email addresses for the authenticated user

Lists all of your email addresses, and specifies which one is visible to the public.  OAuth app tokens and personal access tokens (classic) need the `user:email` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Email>**](email.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_followed_by_authenticated_user

> Vec<models::SimpleUser> users_slash_list_followed_by_authenticated_user(per_page, page)
List the people the authenticated user follows

Lists the people who the authenticated user follows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## users_slash_list_followers_for_authenticated_user

> Vec<models::SimpleUser> users_slash_list_followers_for_authenticated_user(per_page, page)
List followers of the authenticated user

Lists the people following the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## users_slash_list_followers_for_user

> Vec<models::SimpleUser> users_slash_list_followers_for_user(username, per_page, page)
List followers of a user

Lists the people following the specified user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
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


## users_slash_list_following_for_user

> Vec<models::SimpleUser> users_slash_list_following_for_user(username, per_page, page)
List the people a user follows

Lists the people who the specified user follows.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
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


## users_slash_list_gpg_keys_for_authenticated_user

> Vec<models::GpgKey> users_slash_list_gpg_keys_for_authenticated_user(per_page, page)
List GPG keys for the authenticated user

Lists the current user's GPG keys.  OAuth app tokens and personal access tokens (classic) need the `read:gpg_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::GpgKey>**](gpg-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_gpg_keys_for_user

> Vec<models::GpgKey> users_slash_list_gpg_keys_for_user(username, per_page, page)
List GPG keys for a user

Lists the GPG keys for a user. This information is accessible by anyone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::GpgKey>**](gpg-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_public_emails_for_authenticated_user

> Vec<models::Email> users_slash_list_public_emails_for_authenticated_user(per_page, page)
List public email addresses for the authenticated user

Lists your publicly visible email address, which you can set with the [Set primary email visibility for the authenticated user](https://docs.github.com/rest/users/emails#set-primary-email-visibility-for-the-authenticated-user) endpoint.  OAuth app tokens and personal access tokens (classic) need the `user:email` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Email>**](email.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_public_keys_for_user

> Vec<models::KeySimple> users_slash_list_public_keys_for_user(username, per_page, page)
List public keys for a user

Lists the _verified_ public SSH keys for a user. This is accessible by anyone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::KeySimple>**](key-simple.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_public_ssh_keys_for_authenticated_user

> Vec<models::Key> users_slash_list_public_ssh_keys_for_authenticated_user(per_page, page)
List public SSH keys for the authenticated user

Lists the public SSH keys for the authenticated user's GitHub account.  OAuth app tokens and personal access tokens (classic) need the `read:public_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Key>**](key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_social_accounts_for_authenticated_user

> Vec<models::SocialAccount> users_slash_list_social_accounts_for_authenticated_user(per_page, page)
List social accounts for the authenticated user

Lists all of your social accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::SocialAccount>**](social-account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_social_accounts_for_user

> Vec<models::SocialAccount> users_slash_list_social_accounts_for_user(username, per_page, page)
List social accounts for a user

Lists social media accounts for a user. This endpoint is accessible by anyone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::SocialAccount>**](social-account.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_ssh_signing_keys_for_authenticated_user

> Vec<models::SshSigningKey> users_slash_list_ssh_signing_keys_for_authenticated_user(per_page, page)
List SSH signing keys for the authenticated user

Lists the SSH signing keys for the authenticated user's GitHub account.  OAuth app tokens and personal access tokens (classic) need the `read:ssh_signing_key` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::SshSigningKey>**](ssh-signing-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_list_ssh_signing_keys_for_user

> Vec<models::SshSigningKey> users_slash_list_ssh_signing_keys_for_user(username, per_page, page)
List SSH signing keys for a user

Lists the SSH signing keys for a user. This operation is accessible by anyone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::SshSigningKey>**](ssh-signing-key.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_set_primary_email_visibility_for_authenticated_user

> Vec<models::Email> users_slash_set_primary_email_visibility_for_authenticated_user(users_set_primary_email_visibility_for_authenticated_user_request)
Set primary email visibility for the authenticated user

Sets the visibility for your primary email addresses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_set_primary_email_visibility_for_authenticated_user_request** | [**UsersSetPrimaryEmailVisibilityForAuthenticatedUserRequest**](UsersSetPrimaryEmailVisibilityForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**Vec<models::Email>**](email.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_unblock

> users_slash_unblock(username)
Unblock a user

Unblocks the given user and returns a 204.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_unfollow

> users_slash_unfollow(username)
Unfollow a user

OAuth app tokens and personal access tokens (classic) need the `user:follow` scope to use this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | The handle for the GitHub user account. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_slash_update_authenticated

> models::PrivateUser users_slash_update_authenticated(users_update_authenticated_request)
Update the authenticated user

**Note:** If your email is set to private and you send an `email` parameter as part of this request to update your profile, your privacy settings are still enforced: the email address will not be displayed on your public profile or via the API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_update_authenticated_request** | Option<[**UsersUpdateAuthenticatedRequest**](UsersUpdateAuthenticatedRequest.md)> |  |  |

### Return type

[**models::PrivateUser**](private-user.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

