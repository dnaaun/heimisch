# SelectedActions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**github_owned_allowed** | Option<**bool**> | Whether GitHub-owned actions are allowed. For example, this includes the actions in the `actions` organization. | [optional]
**verified_allowed** | Option<**bool**> | Whether actions from GitHub Marketplace verified creators are allowed. Set to `true` to allow all actions by GitHub Marketplace verified creators. | [optional]
**patterns_allowed** | Option<**Vec<String>**> | Specifies a list of string-matching patterns to allow specific action(s) and reusable workflow(s). Wildcards, tags, and SHAs are allowed. For example, `monalisa/octocat@*`, `monalisa/octocat@v2`, `monalisa/_*`.  > [!NOTE] > The `patterns_allowed` setting only applies to public repositories. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


