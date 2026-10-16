# GetBalanceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**balance** | **i64** | Member's available balance in cents for the requested account and exchange index. It includes all exchange indexes when `exchange_index` is omitted. | 
**balance_dollars** | **String** | Fixed-point US dollar string. Most request fields accept 2-4 decimal places (e.g., \"0.56\", \"0.5600\"); responses emit up to 6. Valid quote intervals for a given market are constrained by that market's price level structure. | 
**portfolio_value** | **i64** | Member's portfolio value in cents for the requested account and exchange index. It includes all exchange indexes when `exchange_index` is omitted. | 
**updated_ts** | **i64** | Unix timestamp of the last update to the balance. | 
**balance_breakdown** | Option<[**Vec<models::IndexedBalance>**](IndexedBalance.md)> | User balance breakdown per exchange instance, omitted only when using a subaccount-restricted API key. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


