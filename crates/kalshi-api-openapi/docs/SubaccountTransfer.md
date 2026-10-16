# SubaccountTransfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transfer_id** | **String** | Unique identifier for this transfer. | 
**from_subaccount** | **i64** | Source subaccount number (0 for primary, 1-63 for subaccounts). | 
**to_subaccount** | **i64** | Destination subaccount number (0 for primary, 1-63 for subaccounts). | 
**amount_cents** | **i64** | Cash transfer amount in cents. | 
**created_ts** | **i64** | Unix timestamp when the transfer was created. | 
**exchange_index** | **i64** | Exchange index the transfer was applied on. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


