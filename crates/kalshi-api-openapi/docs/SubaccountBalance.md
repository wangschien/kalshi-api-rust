# SubaccountBalance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subaccount_number** | **i64** | Subaccount number (0 for primary, 1-63 for subaccounts). | 
**exchange_index** | **i64** | Exchange index the balance is held on. | 
**balance** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**updated_ts** | **i64** | Unix timestamp of last balance update. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


