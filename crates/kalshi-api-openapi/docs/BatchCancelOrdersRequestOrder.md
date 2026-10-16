# BatchCancelOrdersRequestOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**order_id** | **String** | Order ID to cancel | 
**subaccount** | Option<**i64**> | Optional subaccount number to use for this cancellation (0 for primary, 1-63 for subaccounts) | [optional][default to 0]
**exchange_index** | Option<**i64**> | Exchange shard index. Defaults to 0. Use -1 to auto-route by market ticker. | [optional][default to 0]
**market_ticker** | Option<**String**> | Market ticker. Required when exchange_index is -1 (auto). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


