# \PortfolioApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apply_subaccount_transfer**](PortfolioApi.md#apply_subaccount_transfer) | **POST** /portfolio/subaccounts/transfer | Transfer Between Subaccounts
[**create_subaccount**](PortfolioApi.md#create_subaccount) | **POST** /portfolio/subaccounts | Create Subaccount
[**delete_event_contract_exit_trigger**](PortfolioApi.md#delete_event_contract_exit_trigger) | **DELETE** /portfolio/positions/{ticker}/exit_trigger | Cancel Position Exit Trigger
[**get_balance**](PortfolioApi.md#get_balance) | **GET** /portfolio/balance | Get Balance
[**get_credit_history**](PortfolioApi.md#get_credit_history) | **GET** /portfolio/credit_history | Get Credit History
[**get_deposits**](PortfolioApi.md#get_deposits) | **GET** /portfolio/deposits | Get Deposits
[**get_event_contract_exit_triggers**](PortfolioApi.md#get_event_contract_exit_triggers) | **GET** /portfolio/positions/{ticker}/exit_trigger | Get Position Exit Triggers
[**get_fills**](PortfolioApi.md#get_fills) | **GET** /portfolio/fills | Get Fills
[**get_intra_exchange_instance_transfer**](PortfolioApi.md#get_intra_exchange_instance_transfer) | **GET** /portfolio/intra_exchange_instance_transfers/{transfer_id} | Get Intra Account Transfer
[**get_intra_exchange_instance_transfers**](PortfolioApi.md#get_intra_exchange_instance_transfers) | **GET** /portfolio/intra_exchange_instance_transfers | Get Intra Account Transfers
[**get_portfolio_resting_order_total_value**](PortfolioApi.md#get_portfolio_resting_order_total_value) | **GET** /portfolio/summary/total_resting_order_value | Get Total Resting Order Value
[**get_positions**](PortfolioApi.md#get_positions) | **GET** /portfolio/positions | Get Positions
[**get_settlements**](PortfolioApi.md#get_settlements) | **GET** /portfolio/settlements | Get Settlements
[**get_subaccount_balances**](PortfolioApi.md#get_subaccount_balances) | **GET** /portfolio/subaccounts/balances | Get All Subaccount Balances
[**get_subaccount_netting**](PortfolioApi.md#get_subaccount_netting) | **GET** /portfolio/subaccounts/netting | Get Subaccount Netting
[**get_subaccount_transfers**](PortfolioApi.md#get_subaccount_transfers) | **GET** /portfolio/subaccounts/transfers | Get Subaccount Transfers
[**get_target_balance_allocation**](PortfolioApi.md#get_target_balance_allocation) | **GET** /portfolio/target_balance_allocation | Get Target Balance Allocation
[**get_withdrawals**](PortfolioApi.md#get_withdrawals) | **GET** /portfolio/withdrawals | Get Withdrawals
[**intra_exchange_instance_transfer**](PortfolioApi.md#intra_exchange_instance_transfer) | **POST** /portfolio/intra_exchange_instance_transfer | Intra Account Transfer
[**set_event_contract_exit_trigger**](PortfolioApi.md#set_event_contract_exit_trigger) | **PUT** /portfolio/positions/{ticker}/exit_trigger | Set Position Exit Trigger
[**set_target_balance_allocation**](PortfolioApi.md#set_target_balance_allocation) | **POST** /portfolio/target_balance_allocation | Set Target Balance Allocation
[**update_subaccount_netting**](PortfolioApi.md#update_subaccount_netting) | **PUT** /portfolio/subaccounts/netting | Update Subaccount Netting



## apply_subaccount_transfer

> serde_json::Value apply_subaccount_transfer(apply_subaccount_transfer_request)
Transfer Between Subaccounts

Transfers funds between the authenticated user's subaccounts. Use 0 for the primary account, or 1-63 for numbered subaccounts. Set exchange_index to apply the transfer on a specific exchange shard (defaults to 0).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apply_subaccount_transfer_request** | [**ApplySubaccountTransferRequest**](ApplySubaccountTransferRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_subaccount

> models::CreateSubaccountResponse create_subaccount(create_subaccount_request)
Create Subaccount

Creates a new subaccount for the authenticated user. This endpoint is available to all users on the Advanced API tier and above. Subaccounts are numbered sequentially starting from 1. Maximum 63 numbered subaccounts per user (64 including the primary account).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_subaccount_request** | Option<[**CreateSubaccountRequest**](CreateSubaccountRequest.md)> |  |  |

### Return type

[**models::CreateSubaccountResponse**](CreateSubaccountResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_event_contract_exit_trigger

> delete_event_contract_exit_trigger(ticker, subaccount, kind)
Cancel Position Exit Trigger

Endpoint for cancelling the take-profit / stop-loss bracket on a position. Succeeds even when nothing is live.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** |  | [required] |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount. |  |
**kind** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_balance

> models::GetBalanceResponse get_balance(subaccount, exchange_index)
Get Balance

Returns the balance and portfolio value for a member. Both values include all exchange indexes unless `exchange_index` is provided. Pass `subaccount` to use a subaccount instead of the primary account. This endpoint also accepts API keys with the 'read::portfolio_balance' scope.  **Rate limit:** this endpoint has its own bucket, separate from the Read budget. Basic and Advanced tiers may call it **1 time per second**; all higher tiers may call it **5 times per second**. The bucket holds one second of budget, so there is no burst. Calls to this endpoint do not draw down the Read budget, and other reads do not draw down this bucket. Balance only changes on fills, settlements and transfers, so react to those (for example the user fills websocket channel) instead of polling this endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount. |  |
**exchange_index** | Option<**i64**> | Exchange index used to scope the balance and portfolio value. If omitted, both include all exchange indexes. |  |

### Return type

[**models::GetBalanceResponse**](GetBalanceResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_credit_history

> models::GetCreditHistoryResponse get_credit_history(limit, cursor, min_ts, max_ts)
Get Credit History

Returns the authenticated member's credit history, newest first. Filters apply to credit creation time: `min_ts` is inclusive and `max_ts` is exclusive. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**min_ts** | Option<**i64**> | Filter items after this Unix timestamp |  |
**max_ts** | Option<**i64**> | Filter items before this Unix timestamp |  |

### Return type

[**models::GetCreditHistoryResponse**](GetCreditHistoryResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deposits

> models::GetDepositsResponse get_deposits(limit, cursor)
Get Deposits

Endpoint for getting the member's deposit history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. Maximum value is 500. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |

### Return type

[**models::GetDepositsResponse**](GetDepositsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_event_contract_exit_triggers

> models::GetEventContractExitTriggers200Response get_event_contract_exit_triggers(ticker, subaccount, kind)
Get Position Exit Triggers

Endpoint for retrieving the take-profit / stop-loss bracket on a position. `exit_triggers` lists live brackets, and `exit_trigger` is the most recently updated bracket, including a terminal one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** |  | [required] |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount. |  |
**kind** | Option<**String**> |  |  |

### Return type

[**models::GetEventContractExitTriggers200Response**](GetEventContractExitTriggers_200_response.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fills

> models::GetFillsResponse get_fills(ticker, order_id, min_ts, max_ts, limit, cursor, subaccount, exchange_index)
Get Fills

Endpoint for getting all fills for the member. A fill is when a trade you have is matched. Registered partners may also use a user OAuth access token with the explicitly granted read::compliance_partner scope. Fills that occurred before the historical cutoff are only available via `GET /historical/fills`. See [Historical Data](https://docs.kalshi.com/getting_started/historical_data) for details. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | Option<**String**> | Filter by market ticker. Accepts a comma-separated list of up to 100 market tickers. |  |
**order_id** | Option<**String**> | Filter by order ID |  |
**min_ts** | Option<**i64**> | Filter items after this Unix timestamp |  |
**max_ts** | Option<**i64**> | Filter items before this Unix timestamp |  |
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts. |  |
**exchange_index** | Option<**i32**> | Filter results by exchange shard. Omit to return results from all exchange shards. |  |

### Return type

[**models::GetFillsResponse**](GetFillsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp), [kalshiOauthAccessToken](../README.md#kalshiOauthAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_intra_exchange_instance_transfer

> models::GetIntraExchangeInstanceTransferResponse get_intra_exchange_instance_transfer(transfer_id)
Get Intra Account Transfer

Endpoint for getting a single intra-account transfer by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_id** | **String** | Transfer id returned by creation endpoint | [required] |

### Return type

[**models::GetIntraExchangeInstanceTransferResponse**](GetIntraExchangeInstanceTransferResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_intra_exchange_instance_transfers

> models::GetIntraExchangeInstanceTransfersResponse get_intra_exchange_instance_transfers(limit, cursor)
Get Intra Account Transfers

Endpoint for fetching intra-exchange account transfer history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. Maximum value is 500. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |

### Return type

[**models::GetIntraExchangeInstanceTransfersResponse**](GetIntraExchangeInstanceTransfersResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_portfolio_resting_order_total_value

> models::GetPortfolioRestingOrderTotalValueResponse get_portfolio_resting_order_total_value()
Get Total Resting Order Value

 Endpoint for getting the total value, in cents, of resting orders. This endpoint is only intended for use by FCM members (rare). Note: If you're uncertain about this endpoint, it likely does not apply to you.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetPortfolioRestingOrderTotalValueResponse**](GetPortfolioRestingOrderTotalValueResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_positions

> models::GetPositionsResponse get_positions(cursor, limit, count_filter, settlement_status, ticker, event_ticker, subaccount, exchange_index)
Get Positions

Returns unsettled positions by default. Set `settlement_status=settled` to page through settled positions that are still in the live data set, or `settlement_status=all` for both live states. Positions already archived are available via `GET /historical/positions`. Settled and all-state positions are paginated by event ticker to keep large histories bounded. Cursors are specific to the selected settlement status. `count_filter` restricts positions to those with a non-zero position or total traded count. Registered partners may also use a user OAuth access token with the explicitly granted read::compliance_partner scope. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | The Cursor represents a pointer to the next page of records in the pagination. Use the value returned from the previous response to get the next page. |  |
**limit** | Option<**i32**> | Parameter to specify the number of results per page. Defaults to 100. |  |[default to 100]
**count_filter** | Option<**String**> | Restricts the positions to those with any of following fields with non-zero values, as a comma separated list. The following values are accepted - position, total_traded |  |
**settlement_status** | Option<**String**> | Return unsettled positions by default, settled positions that have not yet been archived, or all positions in the live data set. |  |[default to unsettled]
**ticker** | Option<**String**> | Filter by market ticker |  |
**event_ticker** | Option<**String**> | Event ticker to filter by. Only a single event ticker is supported. |  |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount. |  |
**exchange_index** | Option<**i32**> | Filter results by exchange shard. Omit to return results from all exchange shards. |  |

### Return type

[**models::GetPositionsResponse**](GetPositionsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp), [kalshiOauthAccessToken](../README.md#kalshiOauthAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_settlements

> models::GetSettlementsResponse get_settlements(limit, cursor, ticker, event_ticker, min_ts, max_ts, subaccount)
Get Settlements

Returns settlement records still in the live data set. Archived positions are available via `GET /historical/positions`, which has different fields; this endpoint does not return archived settlement records.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |
**ticker** | Option<**String**> | Filter by market ticker |  |
**event_ticker** | Option<**String**> | Event ticker to filter by. Only a single event ticker is supported. |  |
**min_ts** | Option<**i64**> | Filter items after this Unix timestamp |  |
**max_ts** | Option<**i64**> | Filter items before this Unix timestamp |  |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts. |  |

### Return type

[**models::GetSettlementsResponse**](GetSettlementsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subaccount_balances

> models::GetSubaccountBalancesResponse get_subaccount_balances()
Get All Subaccount Balances

Gets balances for all subaccounts including the primary account. For an API key restricted to a subaccount, returns only that subaccount's balances across exchange indices.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetSubaccountBalancesResponse**](GetSubaccountBalancesResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subaccount_netting

> models::GetSubaccountNettingResponse get_subaccount_netting()
Get Subaccount Netting

Gets the netting enabled settings for all subaccounts.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetSubaccountNettingResponse**](GetSubaccountNettingResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subaccount_transfers

> models::GetSubaccountTransfersResponse get_subaccount_transfers(limit, cursor)
Get Subaccount Transfers

Gets a paginated list of all transfers between subaccounts for the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |

### Return type

[**models::GetSubaccountTransfersResponse**](GetSubaccountTransfersResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_target_balance_allocation

> models::GetTargetBalanceAllocationResponse get_target_balance_allocation()
Get Target Balance Allocation

Retrieves the caller's target balance allocation across exchange indexes. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetTargetBalanceAllocationResponse**](GetTargetBalanceAllocationResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_withdrawals

> models::GetWithdrawalsResponse get_withdrawals(limit, cursor)
Get Withdrawals

Endpoint for getting the member's withdrawal history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i64**> | Number of results per page. Defaults to 100. Maximum value is 500. |  |[default to 100]
**cursor** | Option<**String**> | Pagination cursor. Use the cursor value returned from the previous response to get the next page of results. Leave empty for the first page. |  |

### Return type

[**models::GetWithdrawalsResponse**](GetWithdrawalsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## intra_exchange_instance_transfer

> models::IntraExchangeInstanceTransferResponse intra_exchange_instance_transfer(intra_exchange_instance_transfer_request)
Intra Account Transfer

Transfers funds within the same account.  Subaccount-restricted API keys can only transfer between `event_contract` exchange indices, with `source_subaccount` and `destination_subaccount` both set to the key's subaccount.  When `source_exchange_shard` and `destination_exchange_shard` are the same, Kalshi treats the request as a subaccount transfer. The returned transfer ID appears in the subaccount transfer history.  Cross-exchange-index subaccount transfers run in up to three non-atomic steps. If a later step fails, completed steps are not undone, so funds may remain in the primary account on the source or destination exchange index. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**intra_exchange_instance_transfer_request** | [**IntraExchangeInstanceTransferRequest**](IntraExchangeInstanceTransferRequest.md) |  | [required] |

### Return type

[**models::IntraExchangeInstanceTransferResponse**](IntraExchangeInstanceTransferResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_event_contract_exit_trigger

> models::EventContractExitTrigger set_event_contract_exit_trigger(ticker, set_event_contract_exit_trigger_request, subaccount)
Set Position Exit Trigger

Endpoint for setting a take-profit / stop-loss bracket on a position. Requires an open position and fires a sell capped at the position. Both legs are set from the request, so a leg you omit is cleared; use DELETE to cancel the bracket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticker** | **String** |  | [required] |
**set_event_contract_exit_trigger_request** | [**SetEventContractExitTriggerRequest**](SetEventContractExitTriggerRequest.md) |  | [required] |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount. |  |

### Return type

[**models::EventContractExitTrigger**](EventContractExitTrigger.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_target_balance_allocation

> serde_json::Value set_target_balance_allocation(set_target_balance_allocation_request)
Set Target Balance Allocation

Replaces the caller's target balance allocation across exchange indexes. Percentages must total 100. Passing an empty allocations array disables automatic rebalancing. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_target_balance_allocation_request** | [**SetTargetBalanceAllocationRequest**](SetTargetBalanceAllocationRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subaccount_netting

> update_subaccount_netting(update_subaccount_netting_request)
Update Subaccount Netting

Updates the netting enabled setting for a specific subaccount. Use 0 for the primary account, or 1-63 for numbered subaccounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_subaccount_netting_request** | [**UpdateSubaccountNettingRequest**](UpdateSubaccountNettingRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

