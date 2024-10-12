# ReposCreateReleaseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tag_name** | **String** | The name of the tag. | 
**target_commitish** | Option<**String**> | Specifies the commitish value that determines where the Git tag is created from. Can be any branch or commit SHA. Unused if the Git tag already exists. Default: the repository's default branch. | [optional]
**name** | Option<**String**> | The name of the release. | [optional]
**body** | Option<**String**> | Text describing the contents of the tag. | [optional]
**draft** | Option<**bool**> | `true` to create a draft (unpublished) release, `false` to create a published one. | [optional][default to false]
**prerelease** | Option<**bool**> | `true` to identify the release as a prerelease. `false` to identify the release as a full release. | [optional][default to false]
**discussion_category_name** | Option<**String**> | If specified, a discussion of the specified category is created and linked to the release. The value must be a category that already exists in the repository. For more information, see \"[Managing categories for discussions in your repository](https://docs.github.com/discussions/managing-discussions-for-your-community/managing-categories-for-discussions-in-your-repository).\" | [optional]
**generate_release_notes** | Option<**bool**> | Whether to automatically generate the name and body for this release. If `name` is specified, the specified name will be used; otherwise, a name will be automatically generated. If `body` is specified, the body will be pre-pended to the automatically generated notes. | [optional][default to false]
**make_latest** | Option<**String**> | Specifies whether this release should be set as the latest release for the repository. Drafts and prereleases cannot be set as latest. Defaults to `true` for newly published releases. `legacy` specifies that the latest release should be determined based on the release creation date and higher semantic version. | [optional][default to True]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


