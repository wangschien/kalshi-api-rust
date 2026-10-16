# MultivariateEventCollection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_ticker** | **String** | Unique identifier for the collection. | 
**series_ticker** | **String** | Series associated with the collection. Events produced in the collection will be associated with this series. | 
**exchange_index** | Option<**i64**> | Exchange index inherited from the collection's series. | [optional]
**title** | **String** | Title of the collection. | 
**description** | **String** | Short description of the collection. | 
**open_date** | **String** | The open date of the collection. Before this time, the collection cannot be interacted with. | 
**close_date** | **String** | The close date of the collection. After this time, the collection cannot be interacted with. | 
**associated_events** | [**Vec<models::AssociatedEvent>**](AssociatedEvent.md) | List of events with their individual configuration. | 
**associated_event_tickers** | **Vec<String>** | [DEPRECATED - Use associated_events instead] A list of events associated with the collection. Markets in these events can be passed as inputs to the Lookup and Create endpoints. | 
**is_ordered** | **bool** | Whether the collection is ordered. If true, the order of markets passed into Lookup/Create affects the output. If false, the order does not matter. | 
**is_single_market_per_event** | **bool** | [DEPRECATED - Use associated_events instead] Whether the collection accepts multiple markets from the same event passed into Lookup/Create. | 
**is_all_yes** | **bool** | [DEPRECATED - Use associated_events instead] Whether the collection requires that only the market side of 'yes' may be used. | 
**size_min** | **i32** | The minimum number of markets that must be passed into Lookup/Create (inclusive). | 
**size_max** | **i32** | The maximum number of markets that must be passed into Lookup/Create (inclusive). | 
**functional_description** | **String** | A functional description of the collection describing how inputs affect the output. | 
**price_level_structure** | **String** | Price level structure used for new markets created in this collection. Existing markets may differ; use the market's price_level_structure and price_ranges for its current pricing. | 
**price_ranges** | [**Vec<models::PriceRange>**](PriceRange.md) | Price bands and tick sizes in fixed-point dollars for new markets created in this collection. Use these ranges to determine valid prices rather than hardcoding the structure name. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


