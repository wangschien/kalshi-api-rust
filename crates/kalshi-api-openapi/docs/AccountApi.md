# \AccountApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account_api_limits**](AccountApi.md#get_account_api_limits) | **GET** /account/limits | Get Account API Limits
[**get_account_api_usage_level_volume_progress**](AccountApi.md#get_account_api_usage_level_volume_progress) | **GET** /account/api_usage_level/volume_progress | Get Account API Usage Level Volume Progress
[**get_account_endpoint_costs**](AccountApi.md#get_account_endpoint_costs) | **GET** /account/endpoint_costs | List Non-Default Endpoint Costs
[**upgrade_account_api_usage_level**](AccountApi.md#upgrade_account_api_usage_level) | **POST** /account/api_usage_level/upgrade | Upgrade Account API Usage Level



## get_account_api_limits

> models::GetAccountApiLimitsResponse get_account_api_limits()
Get Account API Limits

Endpoint to retrieve the authenticated user's Predictions API usage tier and token-bucket limits. Public Predictions tiers include Basic, Advanced, Expert, Premier, Paragon, Prime, and Prestige.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAccountApiLimitsResponse**](GetAccountApiLimitsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_api_usage_level_volume_progress

> models::GetAccountApiUsageLevelVolumeProgressResponse get_account_api_usage_level_volume_progress()
Get Account API Usage Level Volume Progress

Returns the authenticated user's latest cron-computed trading volume progress toward volume-based API usage tiers for the predictions (event_contract) lane. Volume figures are reported as fixed-point contract counts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAccountApiUsageLevelVolumeProgressResponse**](GetAccountApiUsageLevelVolumeProgressResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_endpoint_costs

> models::GetAccountEndpointCostsResponse get_account_endpoint_costs()
List Non-Default Endpoint Costs

Lists API v2 endpoints whose configured token cost differs from the default cost. Endpoints that use the default cost are omitted.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAccountEndpointCostsResponse**](GetAccountEndpointCostsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_account_api_usage_level

> upgrade_account_api_usage_level()
Upgrade Account API Usage Level

Grants a permanent Advanced API usage-level grant. Currently only the Predictions exchange instance is supported. Criteria: at least 1 of the user's last 100 Predictions orders was created via API. Use Get Account API Limits to inspect the resulting usage tier and grants.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

