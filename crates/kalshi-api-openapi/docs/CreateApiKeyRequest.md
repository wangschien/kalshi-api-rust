# CreateApiKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name for the API key. This helps identify the key's purpose | 
**public_key** | **String** | RSA or Ed25519 public key in PEM format (`-----BEGIN PUBLIC KEY-----`). This will be used to verify signatures on API requests - RSA-PSS with SHA-256 for RSA keys, Ed25519 for Ed25519 keys | 
**scopes** | Option<[**Vec<models::ApiKeyScope>**](ApiKeyScope.md)> | List of scopes to grant to the API key. If the broad `write` parent scope is included, `read` must also be included. Child scopes may be granted without the broad parent scope. Defaults to full access (`read`, `write`) if not provided. | [optional]
**subaccount** | Option<**i64**> | If set, restricts the API key to a single sub-account (0-63) that you own. A restricted key may only read and trade on that sub-account; it cannot act on other sub-accounts, transfer funds between sub-accounts, or create sub-accounts. Omit to leave the key unrestricted. Mutually exclusive with fcm_subtrader_id. | [optional]
**fcm_subtrader_id** | Option<**String**> | FCM members only. If set, binds the API key to a single FCM subtrader that you own, spelled {your_user_id}_{suffix} with a suffix of 1-16 case-sensitive ASCII alphanumeric characters. The subtrader must already exist. A bound key is the institution's trading credential for that subtrader - FIX order-entry and market-data sessions, plus margin WebSocket sessions scoped to the subtrader's own data - and is denied on every REST endpoint, including key management. Mutually exclusive with subaccount. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


