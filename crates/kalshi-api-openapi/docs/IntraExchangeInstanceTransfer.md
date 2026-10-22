# IntraExchangeInstanceTransfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transfer_id** | **String** | Unique transfer id | 
**client_transfer_id** | Option<**String**> | The client_transfer_id provided when the transfer was created. Omitted if none was provided. | [optional]
**source** | [**models::ExchangeInstance**](ExchangeInstance.md) |  | 
**destination** | [**models::ExchangeInstance**](ExchangeInstance.md) |  | 
**source_exchange_shard** | **i64** | Source exchange shard index | 
**destination_exchange_shard** | **i64** | Destination exchange shard index | 
**amount** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**status** | [**models::IntraExchangeInstanceTransferStatus**](IntraExchangeInstanceTransferStatus.md) |  | 
**created_ts** | **i64** | Unix timestamp when the transfer was created | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


