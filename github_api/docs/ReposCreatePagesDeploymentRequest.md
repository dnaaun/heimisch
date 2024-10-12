# ReposCreatePagesDeploymentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**artifact_id** | Option<**f64**> | The ID of an artifact that contains the .zip or .tar of static assets to deploy. The artifact belongs to the repository. Either `artifact_id` or `artifact_url` are required. | [optional]
**artifact_url** | Option<**String**> | The URL of an artifact that contains the .zip or .tar of static assets to deploy. The artifact belongs to the repository. Either `artifact_id` or `artifact_url` are required. | [optional]
**environment** | Option<**String**> | The target environment for this GitHub Pages deployment. | [optional][default to github-pages]
**pages_build_version** | **String** | A unique string that represents the version of the build for this deployment. | [default to GITHUB_SHA]
**oidc_token** | **String** | The OIDC token issued by GitHub Actions certifying the origin of the deployment. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


