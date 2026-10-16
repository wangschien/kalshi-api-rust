# GetApiKeysResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_keys** | [**Vec<models::ApiKey>**](ApiKey.md) | List of all API keys associated with the user | 
**api_key_region_expiration_ts** | Option<**i64**> | Unix timestamp (seconds) when the account's location attestation for API key requests expires; a past value means the attestation has lapsed. Absent when the account has never attested. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


