# IntraExchangeInstanceTransferRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_transfer_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Client ID for duplicate prevention. Reuse only for identical requests. Scoped to the user and both exchange instances and shards. Accepted transfers cannot be repeated with the same ID on the same route. Only supported for primary accounts. Transfers involving numbered subaccounts reject this field. If omitted, each request creates a new transfer.  | [optional]
**source** | [**models::ExchangeInstance**](ExchangeInstance.md) |  | 
**destination** | [**models::ExchangeInstance**](ExchangeInstance.md) |  | 
**amount** | **i64** | The amount to transfer in centicents | 
**source_exchange_shard** | Option<**i64**> | Source exchange shard index (default 0) | [optional][default to 0]
**destination_exchange_shard** | Option<**i64**> | Destination exchange shard index (default 0) | [optional][default to 0]
**source_subaccount** | Option<**i64**> | Source subaccount number (default 0 for the primary account). Only supported for event contract to event contract transfers. | [optional][default to 0]
**destination_subaccount** | Option<**i64**> | Destination subaccount number (default 0 for the primary account). Only supported for event contract to event contract transfers. | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


