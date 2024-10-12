# ReposGenerateReleaseNotesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tag_name** | **String** | The tag name for the release. This can be an existing tag or a new one. | 
**target_commitish** | Option<**String**> | Specifies the commitish value that will be the target for the release's tag. Required if the supplied tag_name does not reference an existing tag. Ignored if the tag_name already exists. | [optional]
**previous_tag_name** | Option<**String**> | The name of the previous tag to use as the starting point for the release notes. Use to manually specify the range for the set of changes considered as part this release. | [optional]
**configuration_file_path** | Option<**String**> | Specifies a path to a file in the repository containing configuration settings used for generating the release notes. If unspecified, the configuration file located in the repository at '.github/release.yml' or '.github/release.yaml' will be used. If that is not present, the default configuration will be used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


