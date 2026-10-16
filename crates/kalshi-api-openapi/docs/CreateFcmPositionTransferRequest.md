# CreateFcmPositionTransferRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_subtrader_id** | **String** | Full id of the FCM subtrader giving up the position, as returned by `GET /fcm/subtraders`. | 
**target_subtrader_id** | **String** | Subtrader under your error account that receives the position, `{error_account_id}_{suffix}` with a 1-16 character alphanumeric suffix. Created if it does not exist. | 
**market_ticker** | **String** | Ticker of the market whose position is moved. | 
**position_centicount** | **i64** | Contracts to move, in centicounts (contracts x 100). Positive moves YES, negative moves NO. Must be a non-zero whole number of contracts, e.g. 1000 for 10 YES or -500 for 5 NO. | 
**price_centicents** | **i64** | YES price per contract in centicents (cents x 100), in whole cents, even when moving NO. For example, 4500 is 45 cents. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


