# \OrdersApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**amend_order_v2**](OrdersApi.md#amend_order_v2) | **POST** /portfolio/events/orders/{order_id}/amend | Amend Order (V2)
[**batch_cancel_orders_v2**](OrdersApi.md#batch_cancel_orders_v2) | **DELETE** /portfolio/events/orders/batched | Batch Cancel Orders (V2)
[**batch_create_orders_v2**](OrdersApi.md#batch_create_orders_v2) | **POST** /portfolio/events/orders/batched | Batch Create Orders (V2)
[**cancel_all_orders**](OrdersApi.md#cancel_all_orders) | **DELETE** /portfolio/events/orders | Cancel All Orders
[**cancel_order_v2**](OrdersApi.md#cancel_order_v2) | **DELETE** /portfolio/events/orders/{order_id} | Cancel Order (V2)
[**create_order_v2**](OrdersApi.md#create_order_v2) | **POST** /portfolio/events/orders | Create Order (V2)
[**decrease_order_v2**](OrdersApi.md#decrease_order_v2) | **POST** /portfolio/events/orders/{order_id}/decrease | Decrease Order (V2)
[**get_order**](OrdersApi.md#get_order) | **GET** /portfolio/orders/{order_id} | Get Order
[**get_order_queue_position**](OrdersApi.md#get_order_queue_position) | **GET** /portfolio/orders/{order_id}/queue_position | Get Order Queue Position
[**get_order_queue_positions**](OrdersApi.md#get_order_queue_positions) | **GET** /portfolio/orders/queue_positions | Get Queue Positions for Orders
[**get_orders**](OrdersApi.md#get_orders) | **GET** /portfolio/orders | Get Orders



## amend_order_v2

> models::AmendOrderV2Response amend_order_v2(order_id, amend_order_v2_request, subaccount)
Amend Order (V2)

Endpoint for amending the price, max fillable count, and/or expiration time of an existing event-market order. The request `count` is the updated total/max fillable count, equal to already filled count plus desired resting remaining count. This behavior matches the v1 amend endpoints; only the request/response shape differs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | Order ID | [required] |
**amend_order_v2_request** | [**AmendOrderV2Request**](AmendOrderV2Request.md) |  | [required] |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. |  |

### Return type

[**models::AmendOrderV2Response**](AmendOrderV2Response.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_cancel_orders_v2

> models::BatchCancelOrdersV2Response batch_cancel_orders_v2(batch_cancel_orders_v2_request)
Batch Cancel Orders (V2)

Endpoint for cancelling a batch of event-market orders using the V2 response shape. To auto-route a cancellation, provide its `market_ticker` and omit `exchange_index` or set it to `-1`. The maximum batch size scales with your tier's write budget — see [Rate Limits and Tiers](/getting_started/rate_limits).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_cancel_orders_v2_request** | [**BatchCancelOrdersV2Request**](BatchCancelOrdersV2Request.md) |  | [required] |

### Return type

[**models::BatchCancelOrdersV2Response**](BatchCancelOrdersV2Response.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## batch_create_orders_v2

> models::BatchCreateOrdersV2Response batch_create_orders_v2(batch_create_orders_v2_request)
Batch Create Orders (V2)

Endpoint for submitting a batch of event-market orders using the V2 request/response shape. The maximum batch size scales with your tier's write budget — see [Rate Limits and Tiers](/getting_started/rate_limits).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**batch_create_orders_v2_request** | [**BatchCreateOrdersV2Request**](BatchCreateOrdersV2Request.md) |  | [required] |

### Return type

[**models::BatchCreateOrdersV2Response**](BatchCreateOrdersV2Response.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_all_orders

> cancel_all_orders(subaccount)
Cancel All Orders

Cancels all resting event-market orders for the authenticated Direct member across every exchange shard. If `subaccount` is omitted, matching orders may come from any subaccount. If it is provided, only orders for that subaccount are eligible. Newly placed orders may also be cancelled during the minute after the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts. |  |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_order_v2

> models::CancelOrderV2Response cancel_order_v2(order_id, subaccount, exchange_index, market_ticker)
Cancel Order (V2)

Endpoint for cancelling event-market orders using the V2 response shape. To auto-route the cancellation, provide `market_ticker` and omit `exchange_index` or set it to `-1`. Returns `{order_id, client_order_id, reduced_by}` rather than a full order object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | Order ID | [required] |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. |  |
**exchange_index** | Option<**i64**> | Exchange shard index. If omitted, auto-routes when market_ticker is provided; otherwise defaults to 0. Use -1 to require auto-routing. The market_ticker parameter is required for auto-routing. |  |
**market_ticker** | Option<**String**> | Market ticker. Required for auto-routing when exchange_index is omitted or -1. |  |

### Return type

[**models::CancelOrderV2Response**](CancelOrderV2Response.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order_v2

> models::CreateOrderV2Response create_order_v2(create_order_v2_request)
Create Order (V2)

Endpoint for submitting event-market orders using the V2 request/response shape (single-book `bid`/`ask` side and fixed-point dollar prices). The legacy `/portfolio/orders` endpoint will be deprecated no earlier than May 6, 2026 — clients should migrate to this path.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_order_v2_request** | [**CreateOrderV2Request**](CreateOrderV2Request.md) |  | [required] |

### Return type

[**models::CreateOrderV2Response**](CreateOrderV2Response.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decrease_order_v2

> models::DecreaseOrderV2Response decrease_order_v2(order_id, decrease_order_v2_request, subaccount)
Decrease Order (V2)

Endpoint for decreasing the remaining count of an existing event-market order using the V2 request/response shape. Exactly one of `reduce_by` or `reduce_to` must be provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | Order ID | [required] |
**decrease_order_v2_request** | [**DecreaseOrderV2Request**](DecreaseOrderV2Request.md) |  | [required] |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. |  |

### Return type

[**models::DecreaseOrderV2Response**](DecreaseOrderV2Response.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order

> models::GetOrderResponse get_order(order_id)
Get Order

 Endpoint for getting a single order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | Order ID | [required] |

### Return type

[**models::GetOrderResponse**](GetOrderResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_queue_position

> models::GetOrderQueuePositionResponse get_order_queue_position(order_id)
Get Order Queue Position

 Endpoint for getting an order's queue position in the order book. This represents the amount of orders that need to be matched before this order receives a partial or full match. Queue position is determined using a price-time priority.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** | Order ID | [required] |

### Return type

[**models::GetOrderQueuePositionResponse**](GetOrderQueuePositionResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_queue_positions

> models::GetOrderQueuePositionsResponse get_order_queue_positions(market_tickers, event_ticker, subaccount)
Get Queue Positions for Orders

 Endpoint for getting queue positions for all resting orders. Queue position represents the number of contracts that need to be matched before an order receives a partial or full match, determined using price-time priority.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**market_tickers** | Option<**String**> | Comma-separated list of market tickers to filter by |  |
**event_ticker** | Option<**String**> | Event ticker to filter by |  |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. |  |

### Return type

[**models::GetOrderQueuePositionsResponse**](GetOrderQueuePositionsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orders

> models::GetOrdersResponse get_orders(ticker, event_ticker, min_ts, max_ts, status, limit, cursor, subaccount, exchange_index)
Get Orders

Restricts the response to orders that have a certain status: resting, canceled, or executed. Orders that have been canceled or fully executed before the historical cutoff are only available via `GET /historical/orders`. Resting orders will always be available through this endpoint. See [Historical Data](https://docs.kalshi.com/getting_started/historical_data) for details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Filter by market ticker |  |
**event_ticker** | Option<**String**> | Event tickers to filter by, as a comma-separated list (maximum 10). |  |
**min_ts** | Option<**i64**> | Filter items after this Unix timestamp |  |
**max_ts** | Option<**i64**> | Filter items before this Unix timestamp |  |
**status** | Option<**String**> | Filter by status. Possible values depend on the endpoint. |  |
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts. |  |
**exchange_index** | Option<**i32**> | Filter results by exchange shard. Omit to return results from all exchange shards. |  |

### Return type

[**models::GetOrdersResponse**](GetOrdersResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

