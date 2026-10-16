# ExchangeIndexStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exchange_index** | **i64** | Identifier for an exchange shard. | 
**description** | **String** | Description of this exchange shard. | 
**exchange_active** | **bool** | False if this exchange index is no longer taking any state changes at all. True unless under maintenance. | 
**trading_active** | **bool** | True if trading is currently permitted on this exchange index. False outside exchange hours or during pauses. | 
**intra_exchange_transfers_active** | **bool** | True if intra-exchange transfers are currently permitted on this exchange index. False when transfers are temporarily blocked. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


