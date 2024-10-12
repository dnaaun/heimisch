# MigrationsStartForOrgRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**repositories** | **Vec<String>** | A list of arrays indicating which repositories should be migrated. | 
**lock_repositories** | Option<**bool**> | Indicates whether repositories should be locked (to prevent manipulation) while migrating data. | [optional][default to false]
**exclude_metadata** | Option<**bool**> | Indicates whether metadata should be excluded and only git source should be included for the migration. | [optional][default to false]
**exclude_git_data** | Option<**bool**> | Indicates whether the repository git data should be excluded from the migration. | [optional][default to false]
**exclude_attachments** | Option<**bool**> | Indicates whether attachments should be excluded from the migration (to reduce migration archive file size). | [optional][default to false]
**exclude_releases** | Option<**bool**> | Indicates whether releases should be excluded from the migration (to reduce migration archive file size). | [optional][default to false]
**exclude_owner_projects** | Option<**bool**> | Indicates whether projects owned by the organization or users should be excluded. from the migration. | [optional][default to false]
**org_metadata_only** | Option<**bool**> | Indicates whether this should only include organization metadata (repositories array should be empty and will ignore other flags). | [optional][default to false]
**exclude** | Option<**Vec<String>**> | Exclude related items from being returned in the response in order to improve performance of the request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


