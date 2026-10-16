# \ApiKeysApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_key**](ApiKeysApi.md#create_api_key) | **POST** /api_keys | Create API Key
[**delete_api_key**](ApiKeysApi.md#delete_api_key) | **DELETE** /api_keys/{api_key} | Delete API Key
[**generate_api_key**](ApiKeysApi.md#generate_api_key) | **POST** /api_keys/generate | Generate API Key
[**get_api_keys**](ApiKeysApi.md#get_api_keys) | **GET** /api_keys | Get API Keys



## create_api_key

> models::CreateApiKeyResponse create_api_key(create_api_key_request)
Create API Key

 Endpoint for creating a new API key with a user-provided public key.  This endpoint allows users with Premier or Market Maker API usage levels to create API keys by providing their own RSA or Ed25519 public key in PEM format. The platform will use this public key to verify signatures on API requests: RSA-PSS with SHA-256 for RSA keys, Ed25519 for Ed25519 keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_api_key_request** | [**CreateApiKeyRequest**](CreateApiKeyRequest.md) |  | [required] |

### Return type

[**models::CreateApiKeyResponse**](CreateApiKeyResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_key

> delete_api_key(api_key)
Delete API Key

 Endpoint for deleting an existing API key.  This endpoint permanently deletes an API key. Once deleted, the key can no longer be used for authentication. This action cannot be undone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key** | **String** | API key ID to delete | [required] |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_api_key

> models::GenerateApiKeyResponse generate_api_key(generate_api_key_request)
Generate API Key

 Endpoint for generating a new API key with an automatically created key pair.  This endpoint generates a key pair of the requested key_type (RSA when omitted). The public key is stored on the platform, while the private key is returned to the user and must be stored securely. The private key cannot be retrieved again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**generate_api_key_request** | [**GenerateApiKeyRequest**](GenerateApiKeyRequest.md) |  | [required] |

### Return type

[**models::GenerateApiKeyResponse**](GenerateApiKeyResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_keys

> models::GetApiKeysResponse get_api_keys(fcm_subtrader_id)
Get API Keys

 Endpoint for retrieving all API keys associated with the authenticated user.  API keys allow programmatic access to the platform without requiring username/password authentication. Each key has a unique identifier and name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fcm_subtrader_id** | Option<**String**> | Return only API keys bound to this FCM subtrader. Spelled {your_user_id}_{suffix}; only FCM members hold bound keys. Omit to return every key. |  |

### Return type

[**models::GetApiKeysResponse**](GetApiKeysResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

