# \MigrationsApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**migrations_slash_cancel_import**](MigrationsApi.md#migrations_slash_cancel_import) | **DELETE** /repos/{owner}/{repo}/import | Cancel an import
[**migrations_slash_delete_archive_for_authenticated_user**](MigrationsApi.md#migrations_slash_delete_archive_for_authenticated_user) | **DELETE** /user/migrations/{migration_id}/archive | Delete a user migration archive
[**migrations_slash_delete_archive_for_org**](MigrationsApi.md#migrations_slash_delete_archive_for_org) | **DELETE** /orgs/{org}/migrations/{migration_id}/archive | Delete an organization migration archive
[**migrations_slash_download_archive_for_org**](MigrationsApi.md#migrations_slash_download_archive_for_org) | **GET** /orgs/{org}/migrations/{migration_id}/archive | Download an organization migration archive
[**migrations_slash_get_archive_for_authenticated_user**](MigrationsApi.md#migrations_slash_get_archive_for_authenticated_user) | **GET** /user/migrations/{migration_id}/archive | Download a user migration archive
[**migrations_slash_get_commit_authors**](MigrationsApi.md#migrations_slash_get_commit_authors) | **GET** /repos/{owner}/{repo}/import/authors | Get commit authors
[**migrations_slash_get_import_status**](MigrationsApi.md#migrations_slash_get_import_status) | **GET** /repos/{owner}/{repo}/import | Get an import status
[**migrations_slash_get_large_files**](MigrationsApi.md#migrations_slash_get_large_files) | **GET** /repos/{owner}/{repo}/import/large_files | Get large files
[**migrations_slash_get_status_for_authenticated_user**](MigrationsApi.md#migrations_slash_get_status_for_authenticated_user) | **GET** /user/migrations/{migration_id} | Get a user migration status
[**migrations_slash_get_status_for_org**](MigrationsApi.md#migrations_slash_get_status_for_org) | **GET** /orgs/{org}/migrations/{migration_id} | Get an organization migration status
[**migrations_slash_list_for_authenticated_user**](MigrationsApi.md#migrations_slash_list_for_authenticated_user) | **GET** /user/migrations | List user migrations
[**migrations_slash_list_for_org**](MigrationsApi.md#migrations_slash_list_for_org) | **GET** /orgs/{org}/migrations | List organization migrations
[**migrations_slash_list_repos_for_authenticated_user**](MigrationsApi.md#migrations_slash_list_repos_for_authenticated_user) | **GET** /user/migrations/{migration_id}/repositories | List repositories for a user migration
[**migrations_slash_list_repos_for_org**](MigrationsApi.md#migrations_slash_list_repos_for_org) | **GET** /orgs/{org}/migrations/{migration_id}/repositories | List repositories in an organization migration
[**migrations_slash_map_commit_author**](MigrationsApi.md#migrations_slash_map_commit_author) | **PATCH** /repos/{owner}/{repo}/import/authors/{author_id} | Map a commit author
[**migrations_slash_set_lfs_preference**](MigrationsApi.md#migrations_slash_set_lfs_preference) | **PATCH** /repos/{owner}/{repo}/import/lfs | Update Git LFS preference
[**migrations_slash_start_for_authenticated_user**](MigrationsApi.md#migrations_slash_start_for_authenticated_user) | **POST** /user/migrations | Start a user migration
[**migrations_slash_start_for_org**](MigrationsApi.md#migrations_slash_start_for_org) | **POST** /orgs/{org}/migrations | Start an organization migration
[**migrations_slash_start_import**](MigrationsApi.md#migrations_slash_start_import) | **PUT** /repos/{owner}/{repo}/import | Start an import
[**migrations_slash_unlock_repo_for_authenticated_user**](MigrationsApi.md#migrations_slash_unlock_repo_for_authenticated_user) | **DELETE** /user/migrations/{migration_id}/repos/{repo_name}/lock | Unlock a user repository
[**migrations_slash_unlock_repo_for_org**](MigrationsApi.md#migrations_slash_unlock_repo_for_org) | **DELETE** /orgs/{org}/migrations/{migration_id}/repos/{repo_name}/lock | Unlock an organization repository
[**migrations_slash_update_import**](MigrationsApi.md#migrations_slash_update_import) | **PATCH** /repos/{owner}/{repo}/import | Update an import



## migrations_slash_cancel_import

> migrations_slash_cancel_import(owner, repo)
Cancel an import

Stop an import for a repository.  > [!WARNING] > **Endpoint closing down notice:** Due to very low levels of usage and available alternatives, this endpoint is closing down and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).

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


## migrations_slash_delete_archive_for_authenticated_user

> migrations_slash_delete_archive_for_authenticated_user(migration_id)
Delete a user migration archive

Deletes a previous migration archive. Downloadable migration archives are automatically deleted after seven days. Migration metadata, which is returned in the [List user migrations](https://docs.github.com/rest/migrations/users#list-user-migrations) and [Get a user migration status](https://docs.github.com/rest/migrations/users#get-a-user-migration-status) endpoints, will continue to be available even after an archive is deleted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**migration_id** | **i32** | The unique identifier of the migration. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_delete_archive_for_org

> migrations_slash_delete_archive_for_org(org, migration_id)
Delete an organization migration archive

Deletes a previous migration archive. Migration archives are automatically deleted after seven days.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**migration_id** | **i32** | The unique identifier of the migration. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_download_archive_for_org

> migrations_slash_download_archive_for_org(org, migration_id)
Download an organization migration archive

Fetches the URL to a migration archive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**migration_id** | **i32** | The unique identifier of the migration. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_get_archive_for_authenticated_user

> migrations_slash_get_archive_for_authenticated_user(migration_id)
Download a user migration archive

Fetches the URL to download the migration archive as a `tar.gz` file. Depending on the resources your repository uses, the migration archive can contain JSON files with data for these objects:  *   attachments *   bases *   commit\\_comments *   issue\\_comments *   issue\\_events *   issues *   milestones *   organizations *   projects *   protected\\_branches *   pull\\_request\\_reviews *   pull\\_requests *   releases *   repositories *   review\\_comments *   schema *   users  The archive will also contain an `attachments` directory that includes all attachment files uploaded to GitHub.com and a `repositories` directory that contains the repository's Git data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**migration_id** | **i32** | The unique identifier of the migration. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_get_commit_authors

> Vec<models::PorterAuthor> migrations_slash_get_commit_authors(owner, repo, since)
Get commit authors

Each type of source control system represents authors in a different way. For example, a Git commit author has a display name and an email address, but a Subversion commit author just has a username. The GitHub Importer will make the author information valid, but the author might not be correct. For example, it will change the bare Subversion username `hubot` into something like `hubot <hubot@12341234-abab-fefe-8787-fedcba987654>`.  This endpoint and the [Map a commit author](https://docs.github.com/rest/migrations/source-imports#map-a-commit-author) endpoint allow you to provide correct Git author information.  > [!WARNING] > **Endpoint closing down notice:** Due to very low levels of usage and available alternatives, this endpoint is closing down and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**since** | Option<**i32**> | A user ID. Only return users with an ID greater than this ID. |  |

### Return type

[**Vec<models::PorterAuthor>**](porter-author.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_get_import_status

> models::Import migrations_slash_get_import_status(owner, repo)
Get an import status

View the progress of an import.  > [!WARNING] > **Endpoint closing down notice:** Due to very low levels of usage and available alternatives, this endpoint is closing down and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).  **Import status**  This section includes details about the possible values of the `status` field of the Import Progress response.  An import that does not have errors will progress through these steps:  *   `detecting` - the \"detection\" step of the import is in progress because the request did not include a `vcs` parameter. The import is identifying the type of source control present at the URL. *   `importing` - the \"raw\" step of the import is in progress. This is where commit data is fetched from the original repository. The import progress response will include `commit_count` (the total number of raw commits that will be imported) and `percent` (0 - 100, the current progress through the import). *   `mapping` - the \"rewrite\" step of the import is in progress. This is where SVN branches are converted to Git branches, and where author updates are applied. The import progress response does not include progress information. *   `pushing` - the \"push\" step of the import is in progress. This is where the importer updates the repository on GitHub. The import progress response will include `push_percent`, which is the percent value reported by `git push` when it is \"Writing objects\". *   `complete` - the import is complete, and the repository is ready on GitHub.  If there are problems, you will see one of these in the `status` field:  *   `auth_failed` - the import requires authentication in order to connect to the original repository. To update authentication for the import, please see the [Update an import](https://docs.github.com/rest/migrations/source-imports#update-an-import) section. *   `error` - the import encountered an error. The import progress response will include the `failed_step` and an error message. Contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api) for more information. *   `detection_needs_auth` - the importer requires authentication for the originating repository to continue detection. To update authentication for the import, please see the [Update an import](https://docs.github.com/rest/migrations/source-imports#update-an-import) section. *   `detection_found_nothing` - the importer didn't recognize any source control at the URL. To resolve, [Cancel the import](https://docs.github.com/rest/migrations/source-imports#cancel-an-import) and [retry](https://docs.github.com/rest/migrations/source-imports#start-an-import) with the correct URL. *   `detection_found_multiple` - the importer found several projects or repositories at the provided URL. When this is the case, the Import Progress response will also include a `project_choices` field with the possible project choices as values. To update project choice, please see the [Update an import](https://docs.github.com/rest/migrations/source-imports#update-an-import) section.  **The project_choices field**  When multiple projects are found at the provided URL, the response hash will include a `project_choices` field, the value of which is an array of hashes each representing a project choice. The exact key/value pairs of the project hashes will differ depending on the version control type.  **Git LFS related fields**  This section includes details about Git LFS related fields that may be present in the Import Progress response.  *   `use_lfs` - describes whether the import has been opted in or out of using Git LFS. The value can be `opt_in`, `opt_out`, or `undecided` if no action has been taken. *   `has_large_files` - the boolean value describing whether files larger than 100MB were found during the `importing` step. *   `large_files_size` - the total size in gigabytes of files larger than 100MB found in the originating repository. *   `large_files_count` - the total number of files larger than 100MB found in the originating repository. To see a list of these files, make a \"Get Large Files\" request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**models::Import**](import.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_get_large_files

> Vec<models::PorterLargeFile> migrations_slash_get_large_files(owner, repo)
Get large files

List files larger than 100MB found during the import  > [!WARNING] > **Endpoint closing down notice:** Due to very low levels of usage and available alternatives, this endpoint is closing down and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |

### Return type

[**Vec<models::PorterLargeFile>**](porter-large-file.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_get_status_for_authenticated_user

> models::Migration migrations_slash_get_status_for_authenticated_user(migration_id, exclude)
Get a user migration status

Fetches a single user migration. The response includes the `state` of the migration, which can be one of the following values:  *   `pending` - the migration hasn't started yet. *   `exporting` - the migration is in progress. *   `exported` - the migration finished successfully. *   `failed` - the migration failed.  Once the migration has been `exported` you can [download the migration archive](https://docs.github.com/rest/migrations/users#download-a-user-migration-archive).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**migration_id** | **i32** | The unique identifier of the migration. | [required] |
**exclude** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::Migration**](migration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_get_status_for_org

> models::Migration migrations_slash_get_status_for_org(org, migration_id, exclude)
Get an organization migration status

Fetches the status of a migration.  The `state` of a migration can be one of the following values:  *   `pending`, which means the migration hasn't started yet. *   `exporting`, which means the migration is in progress. *   `exported`, which means the migration finished successfully. *   `failed`, which means the migration failed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**migration_id** | **i32** | The unique identifier of the migration. | [required] |
**exclude** | Option<[**Vec<String>**](String.md)> | Exclude attributes from the API response to improve performance |  |

### Return type

[**models::Migration**](migration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_list_for_authenticated_user

> Vec<models::Migration> migrations_slash_list_for_authenticated_user(per_page, page)
List user migrations

Lists all migrations a user has started.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]

### Return type

[**Vec<models::Migration>**](migration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_list_for_org

> Vec<models::Migration> migrations_slash_list_for_org(org, per_page, page, exclude)
List organization migrations

Lists the most recent migrations, including both exports (which can be started through the REST API) and imports (which cannot be started using the REST API).  A list of `repositories` is only returned for export migrations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**exclude** | Option<[**Vec<String>**](String.md)> | Exclude attributes from the API response to improve performance |  |

### Return type

[**Vec<models::Migration>**](migration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_list_repos_for_authenticated_user

> Vec<models::MinimalRepository> migrations_slash_list_repos_for_authenticated_user(migration_id, per_page, page)
List repositories for a user migration

Lists all the repositories for this user migration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**migration_id** | **i32** | The unique identifier of the migration. | [required] |
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


## migrations_slash_list_repos_for_org

> Vec<models::MinimalRepository> migrations_slash_list_repos_for_org(org, migration_id, per_page, page)
List repositories in an organization migration

List all the repositories for this organization migration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**migration_id** | **i32** | The unique identifier of the migration. | [required] |
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


## migrations_slash_map_commit_author

> models::PorterAuthor migrations_slash_map_commit_author(owner, repo, author_id, migrations_map_commit_author_request)
Map a commit author

Update an author's identity for the import. Your application can continue updating authors any time before you push new commits to the repository.  > [!WARNING] > **Endpoint closing down notice:** Due to very low levels of usage and available alternatives, this endpoint is closing down and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**author_id** | **i32** |  | [required] |
**migrations_map_commit_author_request** | Option<[**MigrationsMapCommitAuthorRequest**](MigrationsMapCommitAuthorRequest.md)> |  |  |

### Return type

[**models::PorterAuthor**](porter-author.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_set_lfs_preference

> models::Import migrations_slash_set_lfs_preference(owner, repo, migrations_set_lfs_preference_request)
Update Git LFS preference

You can import repositories from Subversion, Mercurial, and TFS that include files larger than 100MB. This ability is powered by [Git LFS](https://git-lfs.com).  You can learn more about our LFS feature and working with large files [on our help site](https://docs.github.com/repositories/working-with-files/managing-large-files).  > [!WARNING] > **Endpoint closing down notice:** Due to very low levels of usage and available alternatives, this endpoint is closing down and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**migrations_set_lfs_preference_request** | [**MigrationsSetLfsPreferenceRequest**](MigrationsSetLfsPreferenceRequest.md) |  | [required] |

### Return type

[**models::Import**](import.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_start_for_authenticated_user

> models::Migration migrations_slash_start_for_authenticated_user(migrations_start_for_authenticated_user_request)
Start a user migration

Initiates the generation of a user migration archive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**migrations_start_for_authenticated_user_request** | [**MigrationsStartForAuthenticatedUserRequest**](MigrationsStartForAuthenticatedUserRequest.md) |  | [required] |

### Return type

[**models::Migration**](migration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_start_for_org

> models::Migration migrations_slash_start_for_org(org, migrations_start_for_org_request)
Start an organization migration

Initiates the generation of a migration archive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**migrations_start_for_org_request** | [**MigrationsStartForOrgRequest**](MigrationsStartForOrgRequest.md) |  | [required] |

### Return type

[**models::Migration**](migration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_start_import

> models::Import migrations_slash_start_import(owner, repo, migrations_start_import_request)
Start an import

Start a source import to a GitHub repository using GitHub Importer. Importing into a GitHub repository with GitHub Actions enabled is not supported and will return a status `422 Unprocessable Entity` response.  > [!WARNING] > **Endpoint closing down notice:** Due to very low levels of usage and available alternatives, this endpoint is closing down and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**migrations_start_import_request** | [**MigrationsStartImportRequest**](MigrationsStartImportRequest.md) |  | [required] |

### Return type

[**models::Import**](import.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_unlock_repo_for_authenticated_user

> migrations_slash_unlock_repo_for_authenticated_user(migration_id, repo_name)
Unlock a user repository

Unlocks a repository. You can lock repositories when you [start a user migration](https://docs.github.com/rest/migrations/users#start-a-user-migration). Once the migration is complete you can unlock each repository to begin using it again or [delete the repository](https://docs.github.com/rest/repos/repos#delete-a-repository) if you no longer need the source data. Returns a status of `404 Not Found` if the repository is not locked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**migration_id** | **i32** | The unique identifier of the migration. | [required] |
**repo_name** | **String** | repo_name parameter | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_unlock_repo_for_org

> migrations_slash_unlock_repo_for_org(org, migration_id, repo_name)
Unlock an organization repository

Unlocks a repository that was locked for migration. You should unlock each migrated repository and [delete them](https://docs.github.com/rest/repos/repos#delete-a-repository) when the migration is complete and you no longer need the source data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** | The organization name. The name is not case sensitive. | [required] |
**migration_id** | **i32** | The unique identifier of the migration. | [required] |
**repo_name** | **String** | repo_name parameter | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migrations_slash_update_import

> models::Import migrations_slash_update_import(owner, repo, migrations_update_import_request)
Update an import

An import can be updated with credentials or a project choice by passing in the appropriate parameters in this API request. If no parameters are provided, the import will be restarted.  Some servers (e.g. TFS servers) can have several projects at a single URL. In those cases the import progress will have the status `detection_found_multiple` and the Import Progress response will include a `project_choices` array. You can select the project to import by providing one of the objects in the `project_choices` array in the update request.  > [!WARNING] > **Endpoint closing down notice:** Due to very low levels of usage and available alternatives, this endpoint is closing down and will no longer be available from 00:00 UTC on April 12, 2024. For more details and alternatives, see the [changelog](https://gh.io/source-imports-api-deprecation).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | **String** | The account owner of the repository. The name is not case sensitive. | [required] |
**repo** | **String** | The name of the repository without the `.git` extension. The name is not case sensitive. | [required] |
**migrations_update_import_request** | Option<[**MigrationsUpdateImportRequest**](MigrationsUpdateImportRequest.md)> |  |  |

### Return type

[**models::Import**](import.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

