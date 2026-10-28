# MarketIndex

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Stable index ID, e.g. `SP500` or `SP500-45`. | 
**name** | **String** | Index name, e.g. `S&P 500 Information Technology`. | 
**family** | **String** | Index family, e.g. `S&P 500 GICS Sectors`. | 
**gics_code** | Option<**String**> | GICS sector code, for sector indexes only. | [optional]
**sector** | Option<**String**> | GICS sector name, for sector indexes only. | [optional]
**currency** | **String** | ISO 4217 currency of the index, e.g. `USD`. | 
**units** | **String** | Unit of index values; always `index_points`. | 
**session** | [**models::MarketIndexSession**](MarketIndexSession.md) |  | 
**available** | **bool** | False while the index has no data source; such an index serves no values. | 
**latest** | Option<[**models::MarketIndexLatest**](MarketIndexLatest.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


