# WebhookPush

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**after** | **String** | The SHA of the most recent commit on `ref` after the push. | 
**base_ref** | Option<**String**> |  | 
**before** | **String** | The SHA of the most recent commit on `ref` before the push. | 
**commits** | [**Vec<models::Commit>**](Commit.md) | An array of commit objects describing the pushed commits. (Pushed commits are all commits that are included in the `compare` between the `before` commit and the `after` commit.) The array includes a maximum of 2048 commits. If necessary, you can use the [Commits API](https://docs.github.com/rest/commits) to fetch additional commits. | 
**compare** | **String** | URL that shows the changes in this `ref` update, from the `before` commit to the `after` commit. For a newly created `ref` that is directly based on the default branch, this is the comparison between the head of the default branch and the `after` commit. Otherwise, this shows all commits until the `after` commit. | 
**created** | **bool** | Whether this push created the `ref`. | 
**deleted** | **bool** | Whether this push deleted the `ref`. | 
**enterprise** | Option<[**models::EnterpriseWebhooks**](enterprise-webhooks.md)> |  | [optional]
**forced** | **bool** | Whether this push was a force push of the `ref`. | 
**head_commit** | Option<[**models::Commit1**](Commit_1.md)> |  | 
**installation** | Option<[**models::SimpleInstallation**](simple-installation.md)> |  | [optional]
**organization** | Option<[**models::OrganizationSimpleWebhooks**](organization-simple-webhooks.md)> |  | [optional]
**pusher** | [**models::Committer1**](Committer_1.md) |  | 
**r#ref** | **String** | The full git ref that was pushed. Example: `refs/heads/main` or `refs/tags/v3.14.1`. | 
**repository** | [**models::Repository2**](Repository_2.md) |  | 
**sender** | Option<[**models::SimpleUser**](simple-user.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


