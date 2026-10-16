# \ExchangeApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_exchange_schedule**](ExchangeApi.md#get_exchange_schedule) | **GET** /exchange/schedule | Get Exchange Schedule
[**get_exchange_status**](ExchangeApi.md#get_exchange_status) | **GET** /exchange/status | Get Exchange Status
[**get_series_fee_changes**](ExchangeApi.md#get_series_fee_changes) | **GET** /series/fee_changes | Get Series Fee Changes
[**get_user_data_timestamp**](ExchangeApi.md#get_user_data_timestamp) | **GET** /exchange/user_data_timestamp | Get User Data Timestamp



## get_exchange_schedule

> models::GetExchangeScheduleResponse get_exchange_schedule()
Get Exchange Schedule

 Endpoint for getting the exchange schedule.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetExchangeScheduleResponse**](GetExchangeScheduleResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchange_status

> models::ExchangeStatus get_exchange_status()
Get Exchange Status

 Endpoint for getting the exchange status.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ExchangeStatus**](ExchangeStatus.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_series_fee_changes

> models::GetSeriesFeeChangesResponse get_series_fee_changes(series_ticker, show_historical)
Get Series Fee Changes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**series_ticker** | Option<**String**> |  |  |
**show_historical** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::GetSeriesFeeChangesResponse**](GetSeriesFeeChangesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_data_timestamp

> models::GetUserDataTimestampResponse get_user_data_timestamp()
Get User Data Timestamp

 There is typically a short delay before exchange events are reflected in the API endpoints. Whenever possible, combine API responses to PUT/POST/DELETE requests with WebSocket data to obtain the most accurate view of the exchange state. This endpoint provides an approximate indication of when the data from the following endpoints was last validated: GetBalance, GetOrder(s), GetFills, GetPositions

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetUserDataTimestampResponse**](GetUserDataTimestampResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

