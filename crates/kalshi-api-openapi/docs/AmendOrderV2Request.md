# AmendOrderV2Request

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ticker** | **String** | Market ticker | 
**side** | [**models::BookSide**](BookSide.md) |  | 
**price** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**count** | **String** | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | 
**client_order_id** | Option<**String**> | The original client-specified order ID to be amended | [optional]
**updated_client_order_id** | Option<**String**> | The new client-specified order ID after amendment | [optional]
**expiration_time** | Option<**i64**> | New Unix expiration timestamp in seconds. Omit to preserve the current expiry, or use 0 to remove it (good-till-canceled). A nonzero expiry must be in the future. Send the current price and total quantity for an expiry-only amendment; this preserves queue position. | [optional]
**exchange_index** | Option<**i64**> | Exchange shard index. If omitted, auto-routes when ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing by ticker. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


