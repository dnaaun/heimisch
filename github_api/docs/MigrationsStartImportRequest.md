# MigrationsStartImportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**vcs_url** | **String** | The URL of the originating repository. | 
**vcs** | Option<**String**> | The originating VCS type. Without this parameter, the import job will take additional time to detect the VCS type before beginning the import. This detection step will be reflected in the response. | [optional]
**vcs_username** | Option<**String**> | If authentication is required, the username to provide to `vcs_url`. | [optional]
**vcs_password** | Option<**String**> | If authentication is required, the password to provide to `vcs_url`. | [optional]
**tfvc_project** | Option<**String**> | For a tfvc import, the name of the project that is being imported. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


