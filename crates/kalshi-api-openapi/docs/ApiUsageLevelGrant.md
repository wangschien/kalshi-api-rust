# ApiUsageLevelGrant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exchange_instance** | [**models::ExchangeInstance**](ExchangeInstance.md) |  | 
**level** | **String** | API usage level this grant confers (for example, expert, premier, paragon, prime, or prestige). | 
**expires_ts** | Option<**i64**> | Unix timestamp (seconds) when the grant expires. Absent for permanent grants. | [optional]
**source** | **String** | How the grant was created: \"volume\" (earned from trading volume) or \"manual\" (assigned by Kalshi). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


