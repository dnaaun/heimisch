# ReposUpdateReleaseRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tag_name** | Option<**String**> | The name of the tag. | [optional]
**target_commitish** | Option<**String**> | Specifies the commitish value that determines where the Git tag is created from. Can be any branch or commit SHA. Unused if the Git tag already exists. Default: the repository's default branch. | [optional]
**name** | Option<**String**> | The name of the release. | [optional]
**body** | Option<**String**> | Text describing the contents of the tag. | [optional]
**draft** | Option<**bool**> | `true` makes the release a draft, and `false` publishes the release. | [optional]
**prerelease** | Option<**bool**> | `true` to identify the release as a prerelease, `false` to identify the release as a full release. | [optional]
**make_latest** | Option<**String**> | Specifies whether this release should be set as the latest release for the repository. Drafts and prereleases cannot be set as latest. Defaults to `true` for newly published releases. `legacy` specifies that the latest release should be determined based on the release creation date and higher semantic version. | [optional][default to True]
**discussion_category_name** | Option<**String**> | If specified, a discussion of the specified category is created and linked to the release. The value must be a category that already exists in the repository. If there is already a discussion linked to the release, this parameter is ignored. For more information, see \"[Managing categories for discussions in your repository](https://docs.github.com/discussions/managing-discussions-for-your-community/managing-categories-for-discussions-in-your-repository).\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


