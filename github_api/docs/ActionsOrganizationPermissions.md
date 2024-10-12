# ActionsOrganizationPermissions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enabled_repositories** | [**models::EnabledRepositories**](enabled-repositories.md) |  | 
**selected_repositories_url** | Option<**String**> | The API URL to use to get or set the selected repositories that are allowed to run GitHub Actions, when `enabled_repositories` is set to `selected`. | [optional]
**allowed_actions** | Option<[**models::AllowedActions**](allowed-actions.md)> |  | [optional]
**selected_actions_url** | Option<**String**> | The API URL to use to get or set the actions and reusable workflows that are allowed to run, when `allowed_actions` is set to `selected`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


