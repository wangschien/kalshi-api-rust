# AmendOrderRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subaccount** | Option<**i64**> | Optional subaccount number to use for this amendment (0 for primary, 1-63 for subaccounts) | [optional][default to 0]
**ticker** | **String** | Market ticker | 
**side** | **String** | Side of the order | 
**action** | **String** | Action of the order | 
**client_order_id** | Option<**String**> | The original client-specified order ID to be amended | [optional]
**updated_client_order_id** | Option<**String**> | The new client-specified order ID after amendment | [optional]
**yes_price** | Option<**i64**> | Updated yes price for the order in cents | [optional]
**no_price** | Option<**i64**> | Updated no price for the order in cents | [optional]
**yes_price_dollars** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**no_price_dollars** | Option<**String**> | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | [optional]
**count** | Option<**i64**> | Updated quantity for the order (whole contracts only). If updating quantity, provide count or count_fp; if both provided they must match. | [optional]
**count_fp** | Option<**String**> | Fixed-point contract count string (2 decimals, e.g., \"10.00\"; referred to as \"fp\" in field names). Requests accept 0-2 decimal places (e.g., \"10\", \"10.0\", \"10.00\"); responses always emit 2 decimals. Fractional contract values (e.g., \"2.50\") are supported; the minimum granularity is 0.01 contracts. | [optional]
**exchange_index** | Option<**i64**> | Exchange shard index. Defaults to 0. Use -1 to auto-route by market ticker. | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


