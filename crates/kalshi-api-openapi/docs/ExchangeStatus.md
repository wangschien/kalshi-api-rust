# ExchangeStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exchange_active** | **bool** | False if the core Kalshi exchange is no longer taking any state changes at all. This includes but is not limited to trading, new users, and transfers. True unless we are under maintenance. | 
**trading_active** | **bool** | True if we are currently permitting trading on the exchange. This is true during trading hours and false outside exchange hours. Kalshi reserves the right to pause at any time in case issues are detected. | 
**intra_exchange_transfers_active** | Option<**bool**> | True if intra-exchange transfers are currently permitted. False when transfers are temporarily blocked. | [optional]
**exchange_estimated_resume_time** | Option<**String**> | Estimated downtime for the current exchange maintenance window. However, this is not guaranteed and can be extended. | [optional]
**exchange_index_statuses** | Option<[**Vec<models::ExchangeIndexStatus>**](ExchangeIndexStatus.md)> | Status of each exchange index. The top-level fields above reflect the default exchange index (0). Absent when the per-index breakdown is unavailable. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


