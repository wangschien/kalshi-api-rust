# ApiKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key_id** | **String** | Unique identifier for the API key | 
**name** | **String** | User-provided name for the API key | 
**scopes** | [**Vec<models::ApiKeyScope>**](ApiKeyScope.md) | List of scopes granted to this API key. | 
**subaccount** | Option<**i64**> | If set, the API key is restricted to this single sub-account and may only read and trade on it. Absent/null means the key is unrestricted. | [optional]
**fcm_subtrader_id** | Option<**String**> | If set, the API key is bound to this single FCM subtrader ({fcm_user_id}_{suffix}) and is usable only as that institution's trading credential - FIX sessions and subtrader-scoped margin WebSocket sessions; every REST endpoint is denied. Absent/null means the key carries no subtrader binding. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


