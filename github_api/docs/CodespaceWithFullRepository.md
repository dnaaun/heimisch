# CodespaceWithFullRepository

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** |  | 
**name** | **String** | Automatically generated name of this codespace. | 
**display_name** | Option<**String**> | Display name for this codespace. | [optional]
**environment_id** | Option<**String**> | UUID identifying this codespace's environment. | 
**owner** | [**models::SimpleUser**](simple-user.md) |  | 
**billable_owner** | [**models::SimpleUser**](simple-user.md) |  | 
**repository** | [**models::FullRepository**](full-repository.md) |  | 
**machine** | Option<[**models::NullableCodespaceMachine**](nullable-codespace-machine.md)> |  | 
**devcontainer_path** | Option<**String**> | Path to devcontainer.json from repo root used to create Codespace. | [optional]
**prebuild** | Option<**bool**> | Whether the codespace was created from a prebuild. | 
**created_at** | **String** |  | 
**updated_at** | **String** |  | 
**last_used_at** | **String** | Last known time this codespace was started. | 
**state** | **String** | State of this codespace. | 
**url** | **String** | API URL for this codespace. | 
**git_status** | [**models::CodespaceGitStatus**](codespace_git_status.md) |  | 
**location** | **String** | The initally assigned location of a new codespace. | 
**idle_timeout_minutes** | Option<**i32**> | The number of minutes of inactivity after which this codespace will be automatically stopped. | 
**web_url** | **String** | URL to access this codespace on the web. | 
**machines_url** | **String** | API URL to access available alternate machine types for this codespace. | 
**start_url** | **String** | API URL to start this codespace. | 
**stop_url** | **String** | API URL to stop this codespace. | 
**publish_url** | Option<**String**> | API URL to publish this codespace to a new repository. | [optional]
**pulls_url** | Option<**String**> | API URL for the Pull Request associated with this codespace, if any. | 
**recent_folders** | **Vec<String>** |  | 
**runtime_constraints** | Option<[**models::CodespaceRuntimeConstraints**](codespace_runtime_constraints.md)> |  | [optional]
**pending_operation** | Option<**bool**> | Whether or not a codespace has a pending async operation. This would mean that the codespace is temporarily unavailable. The only thing that you can do with a codespace in this state is delete it. | [optional]
**pending_operation_disabled_reason** | Option<**String**> | Text to show user when codespace is disabled by a pending operation | [optional]
**idle_timeout_notice** | Option<**String**> | Text to show user when codespace idle timeout minutes has been overriden by an organization policy | [optional]
**retention_period_minutes** | Option<**i32**> | Duration in minutes after codespace has gone idle in which it will be deleted. Must be integer minutes between 0 and 43200 (30 days). | [optional]
**retention_expires_at** | Option<**String**> | When a codespace will be auto-deleted based on the \"retention_period_minutes\" and \"last_used_at\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


