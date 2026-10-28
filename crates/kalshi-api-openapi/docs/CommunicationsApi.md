# \CommunicationsApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_block_trade_proposal**](CommunicationsApi.md#accept_block_trade_proposal) | **POST** /communications/block-trade-proposals/{block_trade_proposal_id}/accept | Accept Block Trade Proposal
[**accept_quote**](CommunicationsApi.md#accept_quote) | **PUT** /communications/quotes/{quote_id}/accept | Accept Quote
[**accept_rfq_batch**](CommunicationsApi.md#accept_rfq_batch) | **PUT** /communications/rfq_batches/{rfq_batch_id}/accept | Accept RFQ Batch
[**accept_rfq_quote**](CommunicationsApi.md#accept_rfq_quote) | **PUT** /communications/rfqs/{rfq_id}/quotes/{quote_id}/accept | Accept RFQ Quote
[**confirm_quote**](CommunicationsApi.md#confirm_quote) | **PUT** /communications/quotes/{quote_id}/confirm | Confirm Quote
[**confirm_rfq_batch_quote**](CommunicationsApi.md#confirm_rfq_batch_quote) | **PUT** /communications/rfq_batches/{rfq_batch_id}/quotes/{batch_quote_id}/confirm | Confirm RFQ Batch Quote
[**confirm_rfq_quote**](CommunicationsApi.md#confirm_rfq_quote) | **PUT** /communications/rfqs/{rfq_id}/quotes/{quote_id}/confirm | Confirm RFQ Quote
[**create_quote**](CommunicationsApi.md#create_quote) | **POST** /communications/quotes | Create Quote
[**create_rfq**](CommunicationsApi.md#create_rfq) | **POST** /communications/rfqs | Create RFQ
[**create_rfq_batch**](CommunicationsApi.md#create_rfq_batch) | **POST** /communications/rfq_batches | Create RFQ Batch
[**create_rfq_batch_quote**](CommunicationsApi.md#create_rfq_batch_quote) | **POST** /communications/rfq_batches/{rfq_batch_id}/quotes | Create RFQ Batch Quote
[**delete_quote**](CommunicationsApi.md#delete_quote) | **DELETE** /communications/quotes/{quote_id} | Delete Quote
[**delete_rfq**](CommunicationsApi.md#delete_rfq) | **DELETE** /communications/rfqs/{rfq_id} | Delete RFQ
[**delete_rfq_batch**](CommunicationsApi.md#delete_rfq_batch) | **DELETE** /communications/rfq_batches/{rfq_batch_id} | Delete RFQ Batch
[**delete_rfq_batch_quote**](CommunicationsApi.md#delete_rfq_batch_quote) | **DELETE** /communications/rfq_batches/{rfq_batch_id}/quotes/{batch_quote_id} | Delete RFQ Batch Quote
[**delete_rfq_quote**](CommunicationsApi.md#delete_rfq_quote) | **DELETE** /communications/rfqs/{rfq_id}/quotes/{quote_id} | Delete RFQ Quote
[**get_block_trade_proposals**](CommunicationsApi.md#get_block_trade_proposals) | **GET** /communications/block-trade-proposals | Get Block Trade Proposals
[**get_communications_id**](CommunicationsApi.md#get_communications_id) | **GET** /communications/id | Get Communications ID
[**get_quote**](CommunicationsApi.md#get_quote) | **GET** /communications/quotes/{quote_id} | Get Quote
[**get_quotes**](CommunicationsApi.md#get_quotes) | **GET** /communications/quotes | Get Quotes
[**get_rfq**](CommunicationsApi.md#get_rfq) | **GET** /communications/rfqs/{rfq_id} | Get RFQ
[**get_rfq_batch**](CommunicationsApi.md#get_rfq_batch) | **GET** /communications/rfq_batches/{rfq_batch_id} | Get RFQ Batch
[**get_rfq_batch_best_quotes**](CommunicationsApi.md#get_rfq_batch_best_quotes) | **GET** /communications/rfq_batches/{rfq_batch_id}/best_quotes | Get RFQ Batch Best Quotes
[**get_rfq_batch_quotes**](CommunicationsApi.md#get_rfq_batch_quotes) | **GET** /communications/rfq_batches/{rfq_batch_id}/quotes | Get RFQ Batch Quotes
[**get_rfq_batches**](CommunicationsApi.md#get_rfq_batches) | **GET** /communications/rfq_batches | Get RFQ Batches
[**get_rfq_quote**](CommunicationsApi.md#get_rfq_quote) | **GET** /communications/rfqs/{rfq_id}/quotes/{quote_id} | Get RFQ Quote
[**get_rfqs**](CommunicationsApi.md#get_rfqs) | **GET** /communications/rfqs | Get RFQs
[**propose_block_trade**](CommunicationsApi.md#propose_block_trade) | **POST** /communications/block-trade-proposals | Propose Block Trade



## accept_block_trade_proposal

> accept_block_trade_proposal(block_trade_proposal_id, accept_block_trade_proposal_request)
Accept Block Trade Proposal

 Endpoint for accepting a block trade proposal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_trade_proposal_id** | **String** | Block trade proposal ID | [required] |
**accept_block_trade_proposal_request** | Option<[**AcceptBlockTradeProposalRequest**](AcceptBlockTradeProposalRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accept_quote

> accept_quote(quote_id, accept_quote_request)
Accept Quote

DEPRECATED: Use PUT /communications/rfqs/{rfq_id}/quotes/{quote_id}/accept instead. Endpoint for accepting a quote. This will require the quoter to confirm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quote_id** | **String** | Quote UUID. Pass the ID exactly as received when the quote was created; malformed IDs return HTTP 400. | [required] |
**accept_quote_request** | [**AcceptQuoteRequest**](AcceptQuoteRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accept_rfq_batch

> models::AcceptRfqBatchResponse accept_rfq_batch(rfq_batch_id, accept_rfq_batch_request)
Accept RFQ Batch

Accept every child RFQ of your RFQ batch on one side, each from the maker quoting it best on that side. Either every child is accepted or none is, and your balance must cover all of them. min_bids protects your price: give every child the best bid you were shown. If a child's best bid is now worse, the accept fails with rfq_batch_quotes_changed and lists that child; a better bid fills at the better price. Each winning maker then confirms its children. They execute once every winner has confirmed, and a missed confirmation closes the whole batch. An accepted batch cannot be cancelled.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_batch_id** | **String** | RFQ batch UUID returned when the batch was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |
**accept_rfq_batch_request** | [**AcceptRfqBatchRequest**](AcceptRfqBatchRequest.md) |  | [required] |

### Return type

[**models::AcceptRfqBatchResponse**](AcceptRFQBatchResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accept_rfq_quote

> accept_rfq_quote(rfq_id, quote_id, accept_quote_request)
Accept RFQ Quote

 Endpoint for accepting a quote scoped to its RFQ. This will require the quoter to confirm.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_id** | **String** | RFQ UUID returned when the RFQ was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |
**quote_id** | **String** | Quote UUID. Pass the ID exactly as received when the quote was created; malformed IDs return HTTP 400. | [required] |
**accept_quote_request** | [**AcceptQuoteRequest**](AcceptQuoteRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## confirm_quote

> confirm_quote(quote_id, body)
Confirm Quote

DEPRECATED: Use PUT /communications/rfqs/{rfq_id}/quotes/{quote_id}/confirm instead. Endpoint for confirming a quote. This will start a timer for order execution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quote_id** | **String** | Quote UUID. Pass the ID exactly as received when the quote was created; malformed IDs return HTTP 400. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## confirm_rfq_batch_quote

> confirm_rfq_batch_quote(rfq_batch_id, batch_quote_id, body)
Confirm RFQ Batch Quote

Confirm the children your batch quote won, all at once. Nothing executes until every winning maker of the batch has confirmed; the last confirmation starts the execution timer for every child, and the children then execute together. If any winner misses the confirmation deadline, the whole batch closes without trading.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_batch_id** | **String** | RFQ batch UUID returned when the batch was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |
**batch_quote_id** | **String** | Batch quote UUID returned when the batch quote was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## confirm_rfq_quote

> confirm_rfq_quote(rfq_id, quote_id, body)
Confirm RFQ Quote

 Endpoint for confirming a quote scoped to its RFQ. This will start a timer for order execution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_id** | **String** | RFQ UUID returned when the RFQ was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |
**quote_id** | **String** | Quote UUID. Pass the ID exactly as received when the quote was created; malformed IDs return HTTP 400. | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_quote

> models::CreateQuoteResponse create_quote(create_quote_request)
Create Quote

 Endpoint for creating a quote in response to an RFQ

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_quote_request** | [**CreateQuoteRequest**](CreateQuoteRequest.md) |  | [required] |

### Return type

[**models::CreateQuoteResponse**](CreateQuoteResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_rfq

> models::CreateRfqResponse create_rfq(create_rfq_request)
Create RFQ

 Endpoint for creating a new RFQ. You can have a maximum of 100 open RFQs at a time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_rfq_request** | [**CreateRfqRequest**](CreateRfqRequest.md) |  | [required] |

### Return type

[**models::CreateRfqResponse**](CreateRFQResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_rfq_batch

> models::CreateRfqBatchResponse create_rfq_batch(create_rfq_batch_request)
Create RFQ Batch

Request quotes on several combos at once, for example every parlay of a round robin. Each child is an ordinary RFQ on one combo market. Makers quote the whole batch, but each child is accepted from whichever maker quotes it best on your side, so the batch may fill from several makers. Accepting takes every child or none, and the children then execute together as one fill-or-kill: nothing trades unless every child can fill in full. In the rare case that filling one child cancels a maker order another child needed, the batch closes partially executed. Name each child's combo by its market ticker, or by its legs to look up or create the combo in mve_collection_ticker. Size each child with its own contracts_fp or target_cost_dollars, or set one on the batch for every child that has none; every child must be sized the same way, by contracts or by target cost. rest_remainder must be false, and FCM members cannot create RFQ batches yet (403 rfq_batch_unavailable). Makers also see each child as an ordinary RFQ, with rfq_batch_id set, and can quote it as a single RFQ; its best quote counts toward the batch's.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_rfq_batch_request** | [**CreateRfqBatchRequest**](CreateRfqBatchRequest.md) |  | [required] |

### Return type

[**models::CreateRfqBatchResponse**](CreateRFQBatchResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_rfq_batch_quote

> models::CreateRfqBatchQuoteResponse create_rfq_batch_quote(rfq_batch_id, create_rfq_batch_quote_request)
Create RFQ Batch Quote

Quote every child RFQ of an RFQ batch, each exactly once. Each child is accepted from the maker bidding best on the requester's side, with ties going to the earliest bid, so you may win some children and not others; you then confirm the children you won. A new batch quote replaces your previous one on the same batch, and a child resent at an unchanged price keeps its time priority. A batch quote can win children only within 30 seconds of when you last sent it, so resend it to stay eligible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_batch_id** | **String** | RFQ batch UUID returned when the batch was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |
**create_rfq_batch_quote_request** | [**CreateRfqBatchQuoteRequest**](CreateRfqBatchQuoteRequest.md) |  | [required] |

### Return type

[**models::CreateRfqBatchQuoteResponse**](CreateRFQBatchQuoteResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_quote

> delete_quote(quote_id)
Delete Quote

DEPRECATED: Use DELETE /communications/rfqs/{rfq_id}/quotes/{quote_id} instead. Endpoint for deleting a quote, which means it can no longer be accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quote_id** | **String** | Quote UUID. Pass the ID exactly as received when the quote was created; malformed IDs return HTTP 400. | [required] |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_rfq

> delete_rfq(rfq_id)
Delete RFQ

 Endpoint for deleting an RFQ by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_id** | **String** | RFQ UUID returned when the RFQ was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_rfq_batch

> delete_rfq_batch(rfq_batch_id)
Delete RFQ Batch

Close an RFQ batch and all of its child RFQs. Open batch quotes on it are cancelled. An accepted batch can no longer be closed: it executes or expires as a unit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_batch_id** | **String** | RFQ batch UUID returned when the batch was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_rfq_batch_quote

> delete_rfq_batch_quote(rfq_batch_id, batch_quote_id)
Delete RFQ Batch Quote

 Endpoint for withdrawing one of your open batch quotes, which means it can no longer be accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_batch_id** | **String** | RFQ batch UUID returned when the batch was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |
**batch_quote_id** | **String** | Batch quote UUID returned when the batch quote was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_rfq_quote

> delete_rfq_quote(rfq_id, quote_id)
Delete RFQ Quote

 Endpoint for deleting a quote scoped to its RFQ, which means it can no longer be accepted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_id** | **String** | RFQ UUID returned when the RFQ was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |
**quote_id** | **String** | Quote UUID. Pass the ID exactly as received when the quote was created; malformed IDs return HTTP 400. | [required] |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_trade_proposals

> models::GetBlockTradeProposalsResponse get_block_trade_proposals(cursor, market_ticker, limit, status)
Get Block Trade Proposals

 Endpoint for getting block trade proposals visible to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**market_ticker** | Option<**String**> | Filter by market ticker |  |
**limit** | Option<**i32**> | Parameter to specify the number of results per page. Defaults to 100. |  |[default to 100]
**status** | Option<**String**> | Filter block trade proposals by status |  |

### Return type

[**models::GetBlockTradeProposalsResponse**](GetBlockTradeProposalsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_communications_id

> models::GetCommunicationsIdResponse get_communications_id()
Get Communications ID

 Endpoint for getting the communications ID of the logged-in user.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetCommunicationsIdResponse**](GetCommunicationsIDResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quote

> models::GetQuoteResponse get_quote(quote_id)
Get Quote

DEPRECATED: Use GET /communications/rfqs/{rfq_id}/quotes/{quote_id} instead. Endpoint for getting a particular quote.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**quote_id** | **String** | Quote UUID. Pass the ID exactly as received when the quote was created; malformed IDs return HTTP 400. | [required] |

### Return type

[**models::GetQuoteResponse**](GetQuoteResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_quotes

> models::GetQuotesResponse get_quotes(cursor, min_ts, max_ts, limit, status, quote_creator_user_id, user_filter, rfq_user_filter, rfq_creator_user_id, rfq_creator_subtrader_id, rfq_id)
Get Quotes

List quotes. The rfq_id filter requires an RFQ UUID. Pass pagination cursors back unchanged; malformed RFQ IDs or cursors return HTTP 400.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**min_ts** | Option<**i64**> | Restricts the response to quotes last updated after a timestamp, formatted as a Unix Timestamp |  |
**max_ts** | Option<**i64**> | Restricts the response to quotes last updated before a timestamp, formatted as a Unix Timestamp |  |
**limit** | Option<**i32**> | Parameter to specify the number of results per page. Defaults to 500. |  |[default to 500]
**status** | Option<**String**> | Filter quotes by status |  |
**quote_creator_user_id** | Option<**String**> | Filter quotes by quote creator user ID |  |
**user_filter** | Option<[**UserFilter**](.md)> | Filter for quotes created by the authenticated user. |  |
**rfq_user_filter** | Option<[**UserFilter**](.md)> | Filter for quotes responding to RFQs created by the authenticated user. |  |
**rfq_creator_user_id** | Option<**String**> | Filter quotes by RFQ creator user ID |  |
**rfq_creator_subtrader_id** | Option<**String**> | Filter quotes by RFQ creator subtrader ID (FCM members only) |  |
**rfq_id** | Option<**String**> | Filter quotes by RFQ UUID. Pass the RFQ ID unchanged; malformed IDs return HTTP 400. |  |

### Return type

[**models::GetQuotesResponse**](GetQuotesResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rfq

> models::GetRfqResponse get_rfq(rfq_id)
Get RFQ

 Endpoint for getting a single RFQ by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_id** | **String** | RFQ UUID returned when the RFQ was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |

### Return type

[**models::GetRfqResponse**](GetRFQResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rfq_batch

> models::GetRfqBatchResponse get_rfq_batch(rfq_batch_id)
Get RFQ Batch

 Endpoint for getting a single RFQ batch, with its child RFQs in request order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_batch_id** | **String** | RFQ batch UUID returned when the batch was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |

### Return type

[**models::GetRfqBatchResponse**](GetRFQBatchResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rfq_batch_best_quotes

> models::GetRfqBatchBestQuotesResponse get_rfq_batch_best_quotes(rfq_batch_id)
Get RFQ Batch Best Quotes

Get the best open bid on each side of every child of your RFQ batch, across makers. Accepting the batch fills at these prices. Makers are not identified. Only the batch creator can read the best quotes, and a batch that is no longer open has none.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_batch_id** | **String** | RFQ batch UUID returned when the batch was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |

### Return type

[**models::GetRfqBatchBestQuotesResponse**](GetRFQBatchBestQuotesResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rfq_batch_quotes

> models::GetRfqBatchQuotesResponse get_rfq_batch_quotes(rfq_batch_id)
Get RFQ Batch Quotes

List the batch quotes on an RFQ batch that are visible to you. A maker sees its own batch quote; once the batch is accepted, it lists only the children that maker won. Once the batch is accepted, the batch creator sees each winning maker's batch quote on the children it won. While the batch is open, the creator reads the best quotes across makers with Get RFQ Batch Best Quotes instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_batch_id** | **String** | RFQ batch UUID returned when the batch was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |

### Return type

[**models::GetRfqBatchQuotesResponse**](GetRFQBatchQuotesResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rfq_batches

> models::GetRfqBatchesResponse get_rfq_batches(cursor, subaccount, limit, status, user_filter)
Get RFQ Batches

List RFQ batches, most recently updated first, each with its child RFQs in request order. A page holds at most limit batches and stops before its child RFQs would exceed 100; a batch is never split across pages. Pass pagination cursors back unchanged; a malformed cursor returns HTTP 400. Use user_filter=self to list only your own batches.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts. |  |
**limit** | Option<**i32**> | Maximum number of batches per page. Defaults to 20. A page also stops before its child RFQs would exceed 100. |  |[default to 20]
**status** | Option<[**RfqBatchStatus**](.md)> | Filter RFQ batches by status |  |
**user_filter** | Option<[**UserFilter**](.md)> |  |  |

### Return type

[**models::GetRfqBatchesResponse**](GetRFQBatchesResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rfq_quote

> models::GetQuoteResponse get_rfq_quote(rfq_id, quote_id)
Get RFQ Quote

 Endpoint for getting a particular quote scoped to its RFQ.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rfq_id** | **String** | RFQ UUID returned when the RFQ was created. Pass it unchanged; malformed IDs return HTTP 400. | [required] |
**quote_id** | **String** | Quote UUID. Pass the ID exactly as received when the quote was created; malformed IDs return HTTP 400. | [required] |

### Return type

[**models::GetQuoteResponse**](GetQuoteResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rfqs

> models::GetRfqsResponse get_rfqs(cursor, event_ticker, market_ticker, subaccount, limit, status, creator_user_id, user_filter)
Get RFQs

List RFQs. Pass pagination cursors back unchanged. A malformed cursor or invalid creator_user_id UUID returns HTTP 400. Use user_filter=self to filter by the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**event_ticker** | Option<**String**> | Event ticker to filter by. Only a single event ticker is supported. |  |
**market_ticker** | Option<**String**> | Filter by market ticker |  |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts. |  |
**limit** | Option<**i32**> | Parameter to specify the number of results per page. Defaults to 100. |  |[default to 100]
**status** | Option<**String**> | Filter RFQs by status |  |
**creator_user_id** | Option<**String**> | Filter RFQs by creator user UUID. Use user_filter=self for the authenticated user. |  |
**user_filter** | Option<[**UserFilter**](.md)> |  |  |

### Return type

[**models::GetRfqsResponse**](GetRFQsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## propose_block_trade

> models::ProposeBlockTradeResponse propose_block_trade(propose_block_trade_request)
Propose Block Trade

 Endpoint for creating a block trade proposal.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**propose_block_trade_request** | [**ProposeBlockTradeRequest**](ProposeBlockTradeRequest.md) |  | [required] |

### Return type

[**models::ProposeBlockTradeResponse**](ProposeBlockTradeResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

