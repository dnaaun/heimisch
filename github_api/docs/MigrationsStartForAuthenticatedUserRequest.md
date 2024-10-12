# MigrationsStartForAuthenticatedUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lock_repositories** | Option<**bool**> | Lock the repositories being migrated at the start of the migration | [optional]
**exclude_metadata** | Option<**bool**> | Indicates whether metadata should be excluded and only git source should be included for the migration. | [optional]
**exclude_git_data** | Option<**bool**> | Indicates whether the repository git data should be excluded from the migration. | [optional]
**exclude_attachments** | Option<**bool**> | Do not include attachments in the migration | [optional]
**exclude_releases** | Option<**bool**> | Do not include releases in the migration | [optional]
**exclude_owner_projects** | Option<**bool**> | Indicates whether projects owned by the organization or users should be excluded. | [optional]
**org_metadata_only** | Option<**bool**> | Indicates whether this should only include organization metadata (repositories array should be empty and will ignore other flags). | [optional][default to false]
**exclude** | Option<**Vec<String>**> | Exclude attributes from the API response to improve performance | [optional]
**repositories** | **Vec<String>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


