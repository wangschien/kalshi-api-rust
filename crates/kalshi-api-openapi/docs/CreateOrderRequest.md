# CreateOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | **String** |  | 
**client_order_id** | Option<**String**> |  | [optional]
**side** | **String** |  | 
**action** | **String** |  | 
**count** | Option<**i64**> | Order quantity in contracts (whole contracts only). Provide count or count_fp; if both provided they must match. | [optional]
**count_fp** | Option<**String**> | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | [optional]
**yes_price** | Option<**i64**> |  | [optional]
**no_price** | Option<**i64**> |  | [optional]
**yes_price_dollars** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**no_price_dollars** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**expiration_ts** | Option<**i64**> | Optional Unix timestamp in seconds for when the order expires. To place an expiring order, set `time_in_force` to `good_till_canceled` and provide this `expiration_ts`. `GTT` is an internal execution type and is not a valid API value for `time_in_force`. The `immediate_or_cancel` time-in-force value cannot be combined with `expiration_ts`.  | [optional]
**time_in_force** | Option<**String**> | Specifies how long the order remains active. Use `good_till_canceled` with `expiration_ts` for an order that should rest until a specific expiration time; without `expiration_ts`, `good_till_canceled` is a true good-till-canceled order. `GTT` is not a valid API value.  | [optional]
**buy_max_cost** | Option<**i64**> | Maximum cost in cents. When specified, the order will automatically have Fill-or-Kill (FoK) behavior. | [optional]
**post_only** | Option<**bool**> |  | [optional]
**reduce_only** | Option<**bool**> |  | [optional]
**sell_position_floor** | Option<**i64**> | Deprecated: Use reduce_only instead. Only accepts value of 0. | [optional]
**self_trade_prevention_type** | Option<[**models::SelfTradePreventionType**](SelfTradePreventionType.md)> |  | [optional]
**order_group_id** | Option<**String**> | The order group this order is part of | [optional]
**cancel_order_on_pause** | Option<**bool**> | If this flag is set to true, the order will be canceled if the order is open and trading on the exchange is paused for any reason. | [optional]
**subaccount** | Option<**i64**> | The subaccount number to use for this order. 0 is the primary subaccount. | [optional][default to 0]
**exchange_index** | Option<**i64**> | Exchange shard index. Defaults to 0. Use -1 to auto-route by market ticker. | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


