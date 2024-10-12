# CodespacesCreateForAuthenticatedUserRequestOneOf1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pull_request** | [**models::CodespacesCreateForAuthenticatedUserRequestOneOf1PullRequest**](codespaces_create_for_authenticated_user_request_oneOf_1_pull_request.md) |  | 
**location** | Option<**String**> | The requested location for a new codespace. Best efforts are made to respect this upon creation. Assigned by IP if not provided. | [optional]
**geo** | Option<**String**> | The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is closing down. | [optional]
**machine** | Option<**String**> | Machine type to use for this codespace | [optional]
**devcontainer_path** | Option<**String**> | Path to devcontainer.json config to use for this codespace | [optional]
**working_directory** | Option<**String**> | Working directory for this codespace | [optional]
**idle_timeout_minutes** | Option<**i32**> | Time in minutes before codespace stops from inactivity | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


