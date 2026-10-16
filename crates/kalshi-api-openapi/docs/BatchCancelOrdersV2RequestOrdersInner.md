# BatchCancelOrdersV2RequestOrdersInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | **String** | Order ID to cancel. | 
**subaccount** | Option<**i64**> | Optional subaccount number to use for this cancellation (0 for primary, 1-63 for subaccounts). Subaccount-restricted API keys must omit this field or pass their locked subaccount. | [optional]
**exchange_index** | Option<**i64**> | Exchange shard index. If omitted, auto-routes when market_ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing. The market_ticker field is required for auto-routing. | [optional]
**market_ticker** | Option<**String**> | Market ticker. Required for auto-routing when exchange_index is omitted or -1. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


