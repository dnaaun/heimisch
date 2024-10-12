# ChecksCreateRequestOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | **String** | The title of the check run. | 
**summary** | **String** | The summary of the check run. This parameter supports Markdown. **Maximum length**: 65535 characters. | 
**text** | Option<**String**> | The details of the check run. This parameter supports Markdown. **Maximum length**: 65535 characters. | [optional]
**annotations** | Option<[**Vec<models::ChecksCreateRequestOutputAnnotationsInner>**](checks_create_request_output_annotations_inner.md)> | Adds information from your analysis to specific lines of code. Annotations are visible on GitHub in the **Checks** and **Files changed** tab of the pull request. The Checks API limits the number of annotations to a maximum of 50 per API request. To create more than 50 annotations, you have to make multiple requests to the [Update a check run](https://docs.github.com/rest/checks/runs#update-a-check-run) endpoint. Each time you update the check run, annotations are appended to the list of annotations that already exist for the check run. GitHub Actions are limited to 10 warning annotations and 10 error annotations per step. For details about how you can view annotations on GitHub, see \"[About status checks](https://docs.github.com/articles/about-status-checks#checks)\". | [optional]
**images** | Option<[**Vec<models::ChecksCreateRequestOutputImagesInner>**](checks_create_request_output_images_inner.md)> | Adds images to the output displayed in the GitHub pull request UI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


