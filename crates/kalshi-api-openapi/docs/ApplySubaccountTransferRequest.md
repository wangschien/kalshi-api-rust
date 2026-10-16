# ApplySubaccountTransferRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_transfer_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique client-provided transfer ID for idempotency. | 
**from_subaccount** | **i64** | Source subaccount number (0 for primary, 1-63 for numbered subaccounts). | 
**to_subaccount** | **i64** | Destination subaccount number (0 for primary, 1-63 for numbered subaccounts). | 
**amount_cents** | **i64** | Amount to transfer in cents. | 
**exchange_index** | Option<**i64**> | Identifier for an exchange shard. Defaults to 0 if unspecified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


