# GetEventMetadataResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image_url** | **String** | A path to an image that represents this event. | 
**featured_image_url** | Option<**String**> | A path to an image that represents the image of the featured market. | [optional]
**market_details** | [**Vec<models::MarketMetadata>**](MarketMetadata.md) | Metadata for the markets in this event. | 
**settlement_sources** | [**Vec<models::SettlementSource>**](SettlementSource.md) | A list of settlement sources for this event. | 
**competition** | Option<**String**> | Event competition. | [optional]
**competition_scope** | Option<**String**> | Event scope, based on the competition. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


