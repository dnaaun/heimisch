# ReposUpdateInformationAboutPagesSiteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cname** | Option<**String**> | Specify a custom domain for the repository. Sending a `null` value will remove the custom domain. For more about custom domains, see \"[Using a custom domain with GitHub Pages](https://docs.github.com/pages/configuring-a-custom-domain-for-your-github-pages-site).\" | [optional]
**https_enforced** | Option<**bool**> | Specify whether HTTPS should be enforced for the repository. | [optional]
**build_type** | Option<**String**> | The process by which the GitHub Pages site will be built. `workflow` means that the site is built by a custom GitHub Actions workflow. `legacy` means that the site is built by GitHub when changes are pushed to a specific branch. | [optional]
**source** | Option<[**models::ReposUpdateInformationAboutPagesSiteRequestSource**](repos_update_information_about_pages_site_request_source.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


