# GetAccountEndpointCostsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**default_cost** | **i64** | Default token cost applied to endpoints that are not listed in `endpoint_costs`. This is currently 10. | 
**endpoint_costs** | [**Vec<models::EndpointTokenCost>**](EndpointTokenCost.md) | API v2 endpoints whose configured token cost differs from `default_cost`. Endpoints that use the default cost are omitted. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


