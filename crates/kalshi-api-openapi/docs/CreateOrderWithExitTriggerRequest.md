# CreateOrderWithExitTriggerRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | **String** |  | 
**client_order_id** | Option<**String**> |  | [optional]
**side** | [**models::BookSide**](BookSide.md) |  | 
**count** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**price** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**expiration_time** | Option<**i64**> | Optional Unix timestamp in seconds for when the order expires. To place an expiring order, set `time_in_force` to `good_till_canceled` and provide this `expiration_time`. `GTT` is an internal execution type and is not a valid API value for `time_in_force`. The `immediate_or_cancel` time-in-force value cannot be combined with `expiration_time`.  | [optional]
**time_in_force** | **String** |  | 
**post_only** | Option<**bool**> |  | [optional]
**self_trade_prevention_type** | [**models::SelfTradePreventionType**](SelfTradePreventionType.md) |  | 
**cancel_order_on_pause** | Option<**bool**> | If this flag is set to true, the order will be canceled if the order is open and trading on the exchange is paused for any reason. | [optional]
**reduce_only** | Option<**bool**> | Specifies whether the order place count should be capped by the member's current position. Orders with reduce_only set to true will be rejected unless time_in_force is immediate_or_cancel. | [optional]
**subaccount** | Option<**i64**> | The subaccount number to use for this order. 0 is the primary subaccount. Subaccount-restricted API keys must omit this field or pass their locked subaccount. | [optional]
**order_group_id** | Option<**String**> | The order group this order is part of | [optional]
**exchange_index** | Option<**i64**> | Exchange shard index. If omitted, auto-routes when ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by ticker. | [optional]
**exit_trigger** | [**models::AttachedEventContractExitTrigger**](AttachedEventContractExitTrigger.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


