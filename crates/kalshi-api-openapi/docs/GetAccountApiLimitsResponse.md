# GetAccountApiLimitsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**usage_tier** | **String** | User's effective Predictions API usage tier for these limits (for example, basic, advanced, expert, premier, paragon, prime, or prestige). | 
**read** | [**models::BucketLimit**](BucketLimit.md) |  | 
**write** | [**models::BucketLimit**](BucketLimit.md) |  | 
**grants** | [**Vec<models::ApiUsageLevelGrant>**](ApiUsageLevelGrant.md) | The caller's active API usage level grants across exchange lanes, where each grant applies to its exchange_instance and usage_tier reflects the effective tier for the lane reported by this endpoint. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


