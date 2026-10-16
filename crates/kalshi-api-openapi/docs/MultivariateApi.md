# \MultivariateApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_market_in_multivariate_event_collection**](MultivariateApi.md#create_market_in_multivariate_event_collection) | **POST** /multivariate_event_collections/{collection_ticker} | Create Market In Multivariate Event Collection
[**get_multivariate_event_collection**](MultivariateApi.md#get_multivariate_event_collection) | **GET** /multivariate_event_collections/{collection_ticker} | Get Multivariate Event Collection
[**get_multivariate_event_collections**](MultivariateApi.md#get_multivariate_event_collections) | **GET** /multivariate_event_collections | Get Multivariate Event Collections



## create_market_in_multivariate_event_collection

> models::CreateMarketInMultivariateEventCollectionResponse create_market_in_multivariate_event_collection(collection_ticker, create_market_in_multivariate_event_collection_request)
Create Market In Multivariate Event Collection

Endpoint for creating an individual market in a multivariate event collection. This endpoint must be hit at least once before trading or looking up a market. Users are limited to 5000 creations per week.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_ticker** | **String** | Collection ticker | [required] |
**create_market_in_multivariate_event_collection_request** | [**CreateMarketInMultivariateEventCollectionRequest**](CreateMarketInMultivariateEventCollectionRequest.md) |  | [required] |

### Return type

[**models::CreateMarketInMultivariateEventCollectionResponse**](CreateMarketInMultivariateEventCollectionResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_multivariate_event_collection

> models::GetMultivariateEventCollectionResponse get_multivariate_event_collection(collection_ticker)
Get Multivariate Event Collection

 Endpoint for getting data about a multivariate event collection by its ticker.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**collection_ticker** | **String** | Collection ticker | [required] |

### Return type

[**models::GetMultivariateEventCollectionResponse**](GetMultivariateEventCollectionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_multivariate_event_collections

> models::GetMultivariateEventCollectionsResponse get_multivariate_event_collections(status, associated_event_ticker, series_ticker, limit, cursor)
Get Multivariate Event Collections

 Endpoint for getting data about multivariate event collections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> | Only return collections of a certain status. Can be unopened, open, or closed. |  |
**associated_event_ticker** | Option<**String**> | Only return collections associated with a particular event ticker. |  |
**series_ticker** | Option<**String**> | Only return collections with a particular series ticker. |  |
**limit** | Option<**i32**> | Specify the maximum number of results. |  |
**cursor** | Option<**String**> | The Cursor represents a pointer to the next page of records in the pagination. This optional parameter, when filled, should be filled with the cursor string returned in a previous request to this end-point. |  |

### Return type

[**models::GetMultivariateEventCollectionsResponse**](GetMultivariateEventCollectionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

