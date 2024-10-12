# PullsUpdateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | The title of the pull request. | [optional]
**body** | Option<**String**> | The contents of the pull request. | [optional]
**state** | Option<**String**> | State of this Pull Request. Either `open` or `closed`. | [optional]
**base** | Option<**String**> | The name of the branch you want your changes pulled into. This should be an existing branch on the current repository. You cannot update the base branch on a pull request to point to another repository. | [optional]
**maintainer_can_modify** | Option<**bool**> | Indicates whether [maintainers can modify](https://docs.github.com/articles/allowing-changes-to-a-pull-request-branch-created-from-a-fork/) the pull request. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


