# GenerateApiKeyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_key_id** | **String** | Unique identifier for the newly generated API key | 
**key_type** | Option<[**models::ApiKeyType**](ApiKeyType.md)> |  | [optional]
**private_key** | **String** | Private key in PEM format - PKCS#1 (`-----BEGIN RSA PRIVATE KEY-----`) for `rsa`, PKCS#8 (`-----BEGIN PRIVATE KEY-----`) for `ed25519`. This must be stored securely and cannot be retrieved again after this response | 
**warning** | Option<**String**> | Present only when the minted key is bound to an FCM subtrader that is missing a per-subtrader risk control - the initial-margin cap (margin lane) or the event-contract daily cap. The mint still succeeds; the warning names each missing control - an event-contract subtrader without a daily cap has its event-contract orders rejected until one is set, while a margin subtrader without an initial-margin cap is bounded only by firm-level risk limits. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


