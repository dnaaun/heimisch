# CodespaceMachine

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the machine. | 
**display_name** | **String** | The display name of the machine includes cores, memory, and storage. | 
**operating_system** | **String** | The operating system of the machine. | 
**storage_in_bytes** | **i32** | How much storage is available to the codespace. | 
**memory_in_bytes** | **i32** | How much memory is available to the codespace. | 
**cpus** | **i32** | How many cores are available to the codespace. | 
**prebuild_availability** | Option<**String**> | Whether a prebuild is currently available when creating a codespace for this machine and repository. If a branch was not specified as a ref, the default branch will be assumed. Value will be \"null\" if prebuilds are not supported or prebuild availability could not be determined. Value will be \"none\" if no prebuild is available. Latest values \"ready\" and \"in_progress\" indicate the prebuild availability status. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


