# Order

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | **String** |  | 
**user_id** | **String** | Unique identifier for users | 
**client_order_id** | **String** |  | 
**ticker** | **String** |  | 
**side** | Option<**String**> | Deprecated. Use `outcome_side` (or `book_side`) instead. See [Order direction](/getting_started/order_direction). This field will not be removed before May 14, 2026.  | [optional]
**action** | Option<**String**> | Deprecated. Use `outcome_side` (or `book_side`) instead. See [Order direction](/getting_started/order_direction). This field will not be removed before May 14, 2026.  | [optional]
**outcome_side** | **String** | The outcome side this order is positioned for. buy-yes and sell-no produce 'yes'; buy-no and sell-yes produce 'no'.  `outcome_side` describes directional exposure only; it does not change the order's price. An order at price `p` with `outcome_side=no` is matched by an order at the same price `p` with `outcome_side=yes` — both parties trade at the same price, just on opposite directions.  `outcome_side` and `book_side` will become the canonical way to determine order direction. The legacy `action`, `side`, and `is_yes` fields will be deprecated in a future release — please migrate to these new fields.  | 
**book_side** | [**models::BookSide**](BookSide.md) |  | 
**r#type** | **String** |  | 
**status** | [**models::OrderStatus**](OrderStatus.md) |  | 
**yes_price_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**no_price_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**fill_count_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**remaining_count_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**initial_count_fp** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**taker_fill_cost_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**maker_fill_cost_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**taker_fees_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**maker_fees_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**expiration_time** | Option<**String**> |  | [optional]
**created_time** | Option<**String**> |  | [optional]
**last_update_time** | Option<**String**> | The last update to an order (modify, cancel, fill) | [optional]
**self_trade_prevention_type** | Option<[**models::SelfTradePreventionType**](SelfTradePreventionType.md)> |  | [optional]
**order_group_id** | Option<**String**> | The order group this order is part of | [optional]
**cancel_order_on_pause** | Option<**bool**> | If this flag is set to true, the order will be canceled if the order is open and trading on the exchange is paused for any reason. | [optional]
**subaccount_number** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). | [optional]
**exchange_index** | Option<**i64**> | Identifier for an exchange shard. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


