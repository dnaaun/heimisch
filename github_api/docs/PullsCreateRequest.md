# PullsCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The title of the new pull request. Required unless `issue` is specified. | [optional]
**head** | **String** | The name of the branch where your changes are implemented. For cross-repository pull requests in the same network, namespace `head` with a user like this: `username:branch`. | 
**head_repo** | Option<**String**> | The name of the repository where the changes in the pull request were made. This field is required for cross-repository pull requests if both repositories are owned by the same organization. | [optional]
**base** | **String** | The name of the branch you want the changes pulled into. This should be an existing branch on the current repository. You cannot submit a pull request to one repository that requests a merge to a base of another repository. | 
**body** | Option<**String**> | The contents of the pull request. | [optional]
**maintainer_can_modify** | Option<**bool**> | Indicates whether [maintainers can modify](https://docs.github.com/articles/allowing-changes-to-a-pull-request-branch-created-from-a-fork/) the pull request. | [optional]
**draft** | Option<**bool**> | Indicates whether the pull request is a draft. See \"[Draft Pull Requests](https://docs.github.com/articles/about-pull-requests#draft-pull-requests)\" in the GitHub Help documentation to learn more. | [optional]
**issue** | Option<**i64**> | An issue in the repository to convert to a pull request. The issue title, body, and comments will become the title, body, and comments on the new pull request. Required unless `title` is specified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


