# GitCreateCommitRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**message** | **String** | The commit message | 
**tree** | **String** | The SHA of the tree object this commit points to | 
**parents** | Option<**Vec<String>**> | The full SHAs of the commits that were the parents of this commit. If omitted or empty, the commit will be written as a root commit. For a single parent, an array of one SHA should be provided; for a merge commit, an array of more than one should be provided. | [optional]
**author** | Option<[**models::GitCreateCommitRequestAuthor**](git_create_commit_request_author.md)> |  | [optional]
**committer** | Option<[**models::GitCreateCommitRequestCommitter**](git_create_commit_request_committer.md)> |  | [optional]
**signature** | Option<**String**> | The [PGP signature](https://en.wikipedia.org/wiki/Pretty_Good_Privacy) of the commit. GitHub adds the signature to the `gpgsig` header of the created commit. For a commit signature to be verifiable by Git or GitHub, it must be an ASCII-armored detached PGP signature over the string commit as it would be written to the object database. To pass a `signature` parameter, you need to first manually create a valid PGP signature, which can be complicated. You may find it easier to [use the command line](https://git-scm.com/book/id/v2/Git-Tools-Signing-Your-Work) to create signed commits. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


