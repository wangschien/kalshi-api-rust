# Withdrawal

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the withdrawal. | 
**status** | **String** | Current status of the withdrawal. 'applied' means funds have been deducted from balance. | 
**r#type** | **String** | Payment type used for the withdrawal. | 
**amount_cents** | **i64** | Withdrawal amount in cents. | 
**fee_cents** | **i64** | Fee charged for the withdrawal in cents. | 
**created_ts** | **i64** | Unix timestamp of when the withdrawal was created. | 
**finalized_ts** | Option<**i64**> | Unix timestamp of when the withdrawal was finalized (applied, failed, or returned). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


