# \OrderGroupsApi

All URIs are relative to *https://external-api.kalshi.com/trade-api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_order_group**](OrderGroupsApi.md#create_order_group) | **POST** /portfolio/order_groups/create | Create Order Group
[**delete_order_group**](OrderGroupsApi.md#delete_order_group) | **DELETE** /portfolio/order_groups/{order_group_id} | Delete Order Group
[**get_order_group**](OrderGroupsApi.md#get_order_group) | **GET** /portfolio/order_groups/{order_group_id} | Get Order Group
[**get_order_groups**](OrderGroupsApi.md#get_order_groups) | **GET** /portfolio/order_groups | Get Order Groups
[**reset_order_group**](OrderGroupsApi.md#reset_order_group) | **PUT** /portfolio/order_groups/{order_group_id}/reset | Reset Order Group
[**trigger_order_group**](OrderGroupsApi.md#trigger_order_group) | **PUT** /portfolio/order_groups/{order_group_id}/trigger | Trigger Order Group
[**update_order_group_limit**](OrderGroupsApi.md#update_order_group_limit) | **PUT** /portfolio/order_groups/{order_group_id}/limit | Update Order Group Limit



## create_order_group

> models::CreateOrderGroupResponse create_order_group(create_order_group_request)
Create Order Group

 Creates a new order group with a contracts limit measured over a rolling 15-second window. Users can have up to 100,000 order groups at a time. When the limit is hit, all orders in the group are cancelled and no new orders can be placed until reset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_order_group_request** | [**CreateOrderGroupRequest**](CreateOrderGroupRequest.md) |  | [required] |

### Return type

[**models::CreateOrderGroupResponse**](CreateOrderGroupResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_order_group

> serde_json::Value delete_order_group(order_group_id, subaccount, exchange_index)
Delete Order Group

 Deletes an order group and cancels all orders within it. This permanently removes the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_group_id** | **String** | Order group ID | [required] |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount. |  |
**exchange_index** | Option<**i64**> | Identifier for an exchange shard. Defaults to 0. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_group

> models::GetOrderGroupResponse get_order_group(order_group_id, subaccount)
Get Order Group

 Retrieves details for a single order group including all order IDs and auto-cancel status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_group_id** | **String** | Order group ID | [required] |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts. |  |

### Return type

[**models::GetOrderGroupResponse**](GetOrderGroupResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order_groups

> models::GetOrderGroupsResponse get_order_groups(subaccount)
Get Order Groups

 Retrieves all order groups for the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). If omitted, defaults to all subaccounts. |  |

### Return type

[**models::GetOrderGroupsResponse**](GetOrderGroupsResponse.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_order_group

> serde_json::Value reset_order_group(order_group_id, subaccount, exchange_index, body)
Reset Order Group

 Resets the order group's matched contracts counter to zero, allowing new orders to be placed again after the limit was hit.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_group_id** | **String** | Order group ID | [required] |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount. |  |
**exchange_index** | Option<**i64**> | Identifier for an exchange shard. Defaults to 0. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## trigger_order_group

> serde_json::Value trigger_order_group(order_group_id, subaccount, exchange_index, body)
Trigger Order Group

 Triggers the order group, canceling all orders in the group and preventing new orders until the group is reset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_group_id** | **String** | Order group ID | [required] |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount. |  |
**exchange_index** | Option<**i64**> | Identifier for an exchange shard. Defaults to 0. |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_order_group_limit

> serde_json::Value update_order_group_limit(order_group_id, update_order_group_limit_request, subaccount, exchange_index)
Update Order Group Limit

 Updates the order group contracts limit (rolling 15-second window). If the updated limit would immediately trigger the group, all orders in the group are canceled and the group is triggered.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_group_id** | **String** | Order group ID | [required] |
**update_order_group_limit_request** | [**UpdateOrderGroupLimitRequest**](UpdateOrderGroupLimitRequest.md) |  | [required] |
**subaccount** | Option<**i64**> | Subaccount number (0 for primary, 1-63 for subaccounts). Defaults to 0. Subaccount-restricted API keys default to their locked subaccount and must omit this parameter or pass their locked subaccount. |  |
**exchange_index** | Option<**i64**> | Identifier for an exchange shard. Defaults to 0. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[kalshiAccessSignature](../README.md#kalshiAccessSignature), [kalshiAccessKey](../README.md#kalshiAccessKey), [kalshiAccessTimestamp](../README.md#kalshiAccessTimestamp)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

