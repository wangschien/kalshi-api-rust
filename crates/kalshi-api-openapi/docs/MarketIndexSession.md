# MarketIndexSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**timezone** | **String** | IANA time zone of `open` and `close`, e.g. `America/New_York`. | 
**open** | **String** | Session open, `HH:MM` local time. | 
**close** | **String** | Session close, `HH:MM` local time. | 
**days** | **Vec<String>** | Weekdays the session runs, e.g. `mon`. | 
**calendar** | Option<**String**> | Holiday calendar applied on top of `days`: `us_equity` (US stock market holidays and 1:00 PM early closes). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


