# \CommunicationsApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_block_trade_proposal**](CommunicationsApi.md#accept_block_trade_proposal) | **POST** /communications/block-trade-proposals/{block_trade_proposal_id}/accept | Accept Block Trade Proposal
[**accept_quote**](CommunicationsApi.md#accept_quote) | **PUT** /communications/quotes/{quote_id}/accept | Accept Quote
[**accept_rfq_quote**](CommunicationsApi.md#accept_rfq_quote) | **PUT** /communications/rfqs/{rfq_id}/quotes/{quote_id}/accept | Accept RFQ Quote
[**confirm_quote**](CommunicationsApi.md#confirm_quote) | **PUT** /communications/quotes/{quote_id}/confirm | Confirm Quote
[**confirm_rfq_quote**](CommunicationsApi.md#confirm_rfq_quote) | **PUT** /communications/rfqs/{rfq_id}/quotes/{quote_id}/confirm | Confirm RFQ Quote
[**create_quote**](CommunicationsApi.md#create_quote) | **POST** /communications/quotes | Create Quote
[**create_rfq**](CommunicationsApi.md#create_rfq) | **POST** /communications/rfqs | Create RFQ
[**delete_quote**](CommunicationsApi.md#delete_quote) | **DELETE** /communications/quotes/{quote_id} | Delete Quote
[**delete_rfq**](CommunicationsApi.md#delete_rfq) | **DELETE** /communications/rfqs/{rfq_id} | Delete RFQ
[**delete_rfq_quote**](CommunicationsApi.md#delete_rfq_quote) | **DELETE** /communications/rfqs/{rfq_id}/quotes/{quote_id} | Delete RFQ Quote
[**get_block_trade_proposals**](CommunicationsApi.md#get_block_trade_proposals) | **GET** /communications/block-trade-proposals | Get Block Trade Proposals
[**get_communications_id**](CommunicationsApi.md#get_communications_id) | **GET** /communications/id | Get Communications ID
[**get_quote**](CommunicationsApi.md#get_quote) | **GET** /communications/quotes/{quote_id} | Get Quote
[**get_quotes**](CommunicationsApi.md#get_quotes) | **GET** /communications/quotes | Get Quotes
[**get_rfq**](CommunicationsApi.md#get_rfq) | **GET** /communications/rfqs/{rfq_id} | Get RFQ
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

