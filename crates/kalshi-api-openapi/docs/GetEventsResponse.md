# GetEventsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**events** | [**Vec<models::EventData>**](EventData.md) | Array of events matching the query criteria. | 
**milestones** | Option<[**Vec<models::Milestone>**](Milestone.md)> | Array of milestones related to the events. | [optional]
**cursor** | **String** | Pagination cursor for the next page. Empty if there are no more results. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


