# BlockTradeProposal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the block trade proposal | 
**proposer_user_id** | **String** | User ID of the proposal creator | 
**buyer_user_id** | **String** | User ID of the buyer. Empty when the authenticated user is not the buyer. | 
**buyer_subtrader_id** | Option<**String**> | Subtrader ID of the buyer. Empty when the authenticated user is not the buyer. | [optional]
**seller_user_id** | **String** | User ID of the seller. Empty when the authenticated user is not the seller. | 
**seller_subtrader_id** | Option<**String**> | Subtrader ID of the seller. Empty when the authenticated user is not the seller. | [optional]
**market_ticker** | **String** | The ticker of the market for this block trade | 
**price_centi_cents** | **i64** | Price in centi-cents | 
**centicount** | **i64** | Number of contracts in centicounts | 
**maker_side** | **String** | The maker side of the trade | 
**expiration_ts** | **String** | Expiration time of the proposal | 
**status** | **String** | Current status of the proposal | 
**created_ts** | **String** | Timestamp when the proposal was created | 
**updated_ts** | **String** | Timestamp when the proposal was last updated | 
**buyer_accepted** | **bool** | Whether the buyer has accepted the proposal | 
**seller_accepted** | **bool** | Whether the seller has accepted the proposal | 
**buyer_accepted_ts** | Option<**String**> | Timestamp when the buyer accepted | [optional]
**seller_accepted_ts** | Option<**String**> | Timestamp when the seller accepted | [optional]
**executed_ts** | Option<**String**> | Timestamp when the proposal was executed | [optional]
**buyer_order_id** | Option<**String**> | Order ID for the buyer after the proposal is executed | [optional]
**seller_order_id** | Option<**String**> | Order ID for the seller after the proposal is executed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


