# CodespacesCreateForAuthenticatedUserRequestOneOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**repository_id** | **i32** | Repository id for this codespace | 
**r#ref** | Option<**String**> | Git ref (typically a branch name) for this codespace | [optional]
**location** | Option<**String**> | The requested location for a new codespace. Best efforts are made to respect this upon creation. Assigned by IP if not provided. | [optional]
**geo** | Option<**String**> | The geographic area for this codespace. If not specified, the value is assigned by IP. This property replaces `location`, which is closing down. | [optional]
**client_ip** | Option<**String**> | IP for location auto-detection when proxying a request | [optional]
**machine** | Option<**String**> | Machine type to use for this codespace | [optional]
**devcontainer_path** | Option<**String**> | Path to devcontainer.json config to use for this codespace | [optional]
**multi_repo_permissions_opt_out** | Option<**bool**> | Whether to authorize requested permissions from devcontainer.json | [optional]
**working_directory** | Option<**String**> | Working directory for this codespace | [optional]
**idle_timeout_minutes** | Option<**i32**> | Time in minutes before codespace stops from inactivity | [optional]
**display_name** | Option<**String**> | Display name for this codespace | [optional]
**retention_period_minutes** | Option<**i32**> | Duration in minutes after codespace has gone idle in which it will be deleted. Must be integer minutes between 0 and 43200 (30 days). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


