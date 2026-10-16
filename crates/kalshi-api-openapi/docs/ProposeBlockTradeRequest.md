# ProposeBlockTradeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**buyer_user_id** | **String** | User ID of the buyer | 
**buyer_subtrader_id** | Option<**String**> | Subtrader ID of the buyer. Provide either this or buyer_subaccount, not both. | [optional]
**buyer_subaccount** | Option<**i64**> | User-managed subaccount number of the buyer (0 for primary, 1-63 for numbered subaccounts). Provide either this or buyer_subtrader_id, not both. | [optional]
**seller_user_id** | **String** | User ID of the seller | 
**seller_subtrader_id** | Option<**String**> | Subtrader ID of the seller. Provide either this or seller_subaccount, not both. | [optional]
**seller_subaccount** | Option<**i64**> | User-managed subaccount number of the seller (0 for primary, 1-63 for numbered subaccounts). Provide either this or seller_subtrader_id, not both. | [optional]
**market_ticker** | **String** | The ticker of the market for this block trade | 
**price_centi_cents** | **i64** | Price in centi-cents | 
**centicount** | **i64** | Number of contracts in centicounts | 
**maker_side** | **String** | The maker side of the trade | 
**expiration_ts** | **String** | Expiration time of the proposal | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


