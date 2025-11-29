# \CopyTradingApi

All URIs are relative to *https://api.binance.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sapi_v1_copy_trading_futures_lead_symbol_get**](CopyTradingApi.md#sapi_v1_copy_trading_futures_lead_symbol_get) | **GET** /sapi/v1/copyTrading/futures/leadSymbol | Get Futures Lead Trading Symbol Whitelist(USER_DATA)
[**sapi_v1_copy_trading_futures_user_status_get**](CopyTradingApi.md#sapi_v1_copy_trading_futures_user_status_get) | **GET** /sapi/v1/copyTrading/futures/userStatus | Get Futures Lead Trader Status(TRADE)



## sapi_v1_copy_trading_futures_lead_symbol_get

> models::SapiV1CopyTradingFuturesLeadSymbolGet200Response sapi_v1_copy_trading_futures_lead_symbol_get(timestamp, signature, recv_window)
Get Futures Lead Trading Symbol Whitelist(USER_DATA)

Get Futures Lead Trading Symbol Whitelist  Weight(IP): 20

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1CopyTradingFuturesLeadSymbolGet200Response**](_sapi_v1_copyTrading_futures_leadSymbol_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sapi_v1_copy_trading_futures_user_status_get

> models::SapiV1CopyTradingFuturesUserStatusGet200Response sapi_v1_copy_trading_futures_user_status_get(timestamp, signature, recv_window)
Get Futures Lead Trader Status(TRADE)

Get Futures Lead Trader Status  Weight(UID): 20

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**timestamp** | **i64** | UTC timestamp in ms | [required] |
**signature** | **String** | Signature | [required] |
**recv_window** | Option<**i64**> | The value cannot be greater than 60000 |  |

### Return type

[**models::SapiV1CopyTradingFuturesUserStatusGet200Response**](_sapi_v1_copyTrading_futures_userStatus_get_200_response.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

