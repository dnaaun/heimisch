# Page

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | The API address for accessing this Page resource. | 
**status** | Option<**String**> | The status of the most recent build of the Page. | 
**cname** | Option<**String**> | The Pages site's custom domain | 
**protected_domain_state** | Option<**String**> | The state if the domain is verified | [optional]
**pending_domain_unverified_at** | Option<**String**> | The timestamp when a pending domain becomes unverified. | [optional]
**custom_404** | **bool** | Whether the Page has a custom 404 page. | [default to false]
**html_url** | Option<**String**> | The web address the Page can be accessed from. | [optional]
**build_type** | Option<**String**> | The process in which the Page will be built. | [optional]
**source** | Option<[**models::PagesSourceHash**](pages-source-hash.md)> |  | [optional]
**public** | **bool** | Whether the GitHub Pages site is publicly visible. If set to `true`, the site is accessible to anyone on the internet. If set to `false`, the site will only be accessible to users who have at least `read` access to the repository that published the site. | 
**https_certificate** | Option<[**models::PagesHttpsCertificate**](pages-https-certificate.md)> |  | [optional]
**https_enforced** | Option<**bool**> | Whether https is enabled on the domain | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


