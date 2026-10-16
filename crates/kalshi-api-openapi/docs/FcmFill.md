# FcmFill

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fill_id** | **String** | Fill ID. | 
**exchange_index** | **i64** | Identifier for an exchange shard. | 
**ticker** | **String** | Market ticker. | 
**taker_outcome_side** | [**models::OutcomeSide**](OutcomeSide.md) |  | 
**count_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**yes_price_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**created_time** | Option<**String**> | Fill execution time. | [optional]
**maker_order_id** | Option<**String**> | Maker order ID when owned by the FCM. | [optional]
**maker_subtrader_id** | Option<**String**> | Maker subtrader ID when owned by the FCM. | [optional]
**maker_fee_cost** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**taker_order_id** | Option<**String**> | Taker order ID when owned by the FCM. | [optional]
**taker_subtrader_id** | Option<**String**> | Taker subtrader ID when owned by the FCM. | [optional]
**taker_fee_cost** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


