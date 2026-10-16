# MarketCandlestick

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**end_period_ts** | **i64** | Unix timestamp for the inclusive end of the candlestick period. | 
**yes_bid** | [**models::BidAskDistribution**](BidAskDistribution.md) |  | 
**yes_ask** | [**models::BidAskDistribution**](BidAskDistribution.md) |  | 
**price** | [**models::PriceDistribution**](PriceDistribution.md) |  | 
**volume_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**open_interest_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


