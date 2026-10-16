# Milestone

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the milestone. | 
**category** | **String** | Category of the milestone. E.g. Sports, Elections, Esports, Crypto. | 
**r#type** | **String** | Type of the milestone. E.g. football_game, basketball_game, soccer_tournament_multi_leg, baseball_game, hockey_match, golf_tournament, political_race. | 
**start_date** | **String** | Start date of the milestone. | 
**end_date** | Option<**String**> | End date of the milestone, if any. | [optional]
**related_event_tickers** | **Vec<String>** | List of event tickers related to this milestone. | 
**title** | **String** | Title of the milestone. | 
**notification_message** | **String** | Notification message for the milestone. | 
**source_id** | Option<**String**> | Source id of milestone if available. | [optional]
**source_ids** | Option<**std::collections::HashMap<String, String>**> | Source ids of milestone if available. | [optional]
**details** | [**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Additional details about the milestone. | 
**primary_event_tickers** | **Vec<String>** | List of event tickers directly related to the outcome of this milestone. | 
**last_updated_ts** | **String** | Last time this structured target was updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


