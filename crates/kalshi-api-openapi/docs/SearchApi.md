# \SearchApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_filters_for_sports**](SearchApi.md#get_filters_for_sports) | **GET** /search/filters_by_sport | Get Filters for Sports
[**get_tags_for_series_categories**](SearchApi.md#get_tags_for_series_categories) | **GET** /search/tags_by_categories | Get Tags for Series Categories



## get_filters_for_sports

> models::GetFiltersBySportsResponse get_filters_for_sports()
Get Filters for Sports

Retrieve available filters organized by sport.  This endpoint returns filtering options available for each sport, including scopes and competitions. It also provides an ordered list of sports for display purposes. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetFiltersBySportsResponse**](GetFiltersBySportsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tags_for_series_categories

> models::GetTagsForSeriesCategoriesResponse get_tags_for_series_categories()
Get Tags for Series Categories

Retrieve tags organized by series categories.  This endpoint returns a mapping of series categories to their associated tags, which can be used for filtering and search functionality. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetTagsForSeriesCategoriesResponse**](GetTagsForSeriesCategoriesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

