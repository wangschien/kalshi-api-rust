# \StructuredTargetsApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_structured_target**](StructuredTargetsApi.md#get_structured_target) | **GET** /structured_targets/{structured_target_id} | Get Structured Target
[**get_structured_targets**](StructuredTargetsApi.md#get_structured_targets) | **GET** /structured_targets | Get Structured Targets



## get_structured_target

> models::GetStructuredTargetResponse get_structured_target(structured_target_id)
Get Structured Target

 Endpoint for getting data about a specific structured target by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structured_target_id** | **String** | Structured target ID | [required] |

### Return type

[**models::GetStructuredTargetResponse**](GetStructuredTargetResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_structured_targets

> models::GetStructuredTargetsResponse get_structured_targets(ids, r#type, competition, page_size, cursor)
Get Structured Targets

Page size (min: 1, max: 2000)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Filter by specific structured target IDs. Pass multiple IDs by repeating the parameter (e.g. `?ids=uuid1&ids=uuid2`). |  |
**r#type** | Option<**String**> | Filter by structured target type |  |
**competition** | Option<**String**> | Filter by competition. Matches against the league, conference, division, or tour in the structured target details, or any entry in the details leagues array. |  |
**page_size** | Option<**i32**> | Number of items per page (min 1, max 2000, default 100) |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor |  |

### Return type

[**models::GetStructuredTargetsResponse**](GetStructuredTargetsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

