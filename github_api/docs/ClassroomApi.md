# \ClassroomApi

All URIs are relative to *https://api.github.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**classroom_slash_get_a_classroom**](ClassroomApi.md#classroom_slash_get_a_classroom) | **GET** /classrooms/{classroom_id} | Get a classroom
[**classroom_slash_get_an_assignment**](ClassroomApi.md#classroom_slash_get_an_assignment) | **GET** /assignments/{assignment_id} | Get an assignment
[**classroom_slash_get_assignment_grades**](ClassroomApi.md#classroom_slash_get_assignment_grades) | **GET** /assignments/{assignment_id}/grades | Get assignment grades
[**classroom_slash_list_accepted_assignments_for_an_assignment**](ClassroomApi.md#classroom_slash_list_accepted_assignments_for_an_assignment) | **GET** /assignments/{assignment_id}/accepted_assignments | List accepted assignments for an assignment
[**classroom_slash_list_assignments_for_a_classroom**](ClassroomApi.md#classroom_slash_list_assignments_for_a_classroom) | **GET** /classrooms/{classroom_id}/assignments | List assignments for a classroom
[**classroom_slash_list_classrooms**](ClassroomApi.md#classroom_slash_list_classrooms) | **GET** /classrooms | List classrooms



## classroom_slash_get_a_classroom

> models::Classroom classroom_slash_get_a_classroom(classroom_id)
Get a classroom

Gets a GitHub Classroom classroom for the current user. Classroom will only be returned if the current user is an administrator of the GitHub Classroom.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**classroom_id** | **i32** | The unique identifier of the classroom. | [required] |

### Return type

[**models::Classroom**](classroom.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## classroom_slash_get_an_assignment

> models::ClassroomAssignment classroom_slash_get_an_assignment(assignment_id)
Get an assignment

Gets a GitHub Classroom assignment. Assignment will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **i32** | The unique identifier of the classroom assignment. | [required] |

### Return type

[**models::ClassroomAssignment**](classroom-assignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## classroom_slash_get_assignment_grades

> Vec<models::ClassroomAssignmentGrade> classroom_slash_get_assignment_grades(assignment_id)
Get assignment grades

Gets grades for a GitHub Classroom assignment. Grades will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **i32** | The unique identifier of the classroom assignment. | [required] |

### Return type

[**Vec<models::ClassroomAssignmentGrade>**](classroom-assignment-grade.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## classroom_slash_list_accepted_assignments_for_an_assignment

> Vec<models::ClassroomAcceptedAssignment> classroom_slash_list_accepted_assignments_for_an_assignment(assignment_id, page, per_page)
List accepted assignments for an assignment

Lists any assignment repositories that have been created by students accepting a GitHub Classroom assignment. Accepted assignments will only be returned if the current user is an administrator of the GitHub Classroom for the assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **i32** | The unique identifier of the classroom assignment. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**Vec<models::ClassroomAcceptedAssignment>**](classroom-accepted-assignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## classroom_slash_list_assignments_for_a_classroom

> Vec<models::SimpleClassroomAssignment> classroom_slash_list_assignments_for_a_classroom(classroom_id, page, per_page)
List assignments for a classroom

Lists GitHub Classroom assignments for a classroom. Assignments will only be returned if the current user is an administrator of the GitHub Classroom.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**classroom_id** | **i32** | The unique identifier of the classroom. | [required] |
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**Vec<models::SimpleClassroomAssignment>**](simple-classroom-assignment.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## classroom_slash_list_classrooms

> Vec<models::SimpleClassroom> classroom_slash_list_classrooms(page, per_page)
List classrooms

Lists GitHub Classroom classrooms for the current user. Classrooms will only be returned if the current user is an administrator of one or more GitHub Classrooms.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 1]
**per_page** | Option<**i32**> | The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\" |  |[default to 30]

### Return type

[**Vec<models::SimpleClassroom>**](simple-classroom.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

