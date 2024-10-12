# ChecksUpdateRequestOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> | **Required**. | [optional]
**summary** | **String** | Can contain Markdown. | 
**text** | Option<**String**> | Can contain Markdown. | [optional]
**annotations** | Option<[**Vec<models::ChecksCreateRequestOutputAnnotationsInner>**](checks_create_request_output_annotations_inner.md)> | Adds information from your analysis to specific lines of code. Annotations are visible in GitHub's pull request UI. Annotations are visible in GitHub's pull request UI. The Checks API limits the number of annotations to a maximum of 50 per API request. To create more than 50 annotations, you have to make multiple requests to the [Update a check run](https://docs.github.com/rest/checks/runs#update-a-check-run) endpoint. Each time you update the check run, annotations are appended to the list of annotations that already exist for the check run. GitHub Actions are limited to 10 warning annotations and 10 error annotations per step. For details about annotations in the UI, see \"[About status checks](https://docs.github.com/articles/about-status-checks#checks)\". | [optional]
**images** | Option<[**Vec<models::ChecksCreateRequestOutputImagesInner>**](checks_create_request_output_images_inner.md)> | Adds images to the output displayed in the GitHub pull request UI. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


