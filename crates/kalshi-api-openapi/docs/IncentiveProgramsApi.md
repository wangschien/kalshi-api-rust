# \IncentiveProgramsApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_incentive_programs**](IncentiveProgramsApi.md#get_incentive_programs) | **GET** /incentive_programs | Get Incentives



## get_incentive_programs

> models::GetIncentiveProgramsResponse get_incentive_programs(status, r#type, incentive_description, limit, cursor)
Get Incentives

 List incentives with optional filters. Incentives are rewards programs for trading activity on specific markets.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Status filter. Can be \"all\", \"active\", \"upcoming\", \"closed\", or \"paid_out\". Default is \"all\". |  |
**r#type** | Option<**String**> | Type filter. Can be \"all\", \"liquidity\", \"volume\", \"margin_maker_volume\", or \"margin_taker_volume\". Default is \"all\". |  |
**incentive_description** | Option<**String**> | Filter by exact incentive description. |  |
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. Maximum value is 10000. |  |
**cursor** | Option<**String**> | Cursor for pagination |  |

### Return type

[**models::GetIncentiveProgramsResponse**](GetIncentiveProgramsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

