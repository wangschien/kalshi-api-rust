# Deposit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier for the deposit. | 
**status** | **String** | Current status of the deposit. 'applied' means funds are reflected in balance. | 
**r#type** | **String** | Payment method used for the deposit. | 
**amount_cents** | **i64** | Deposit amount in cents. | 
**fee_cents** | **i64** | Fee charged for the deposit in cents. | 
**created_ts** | **i64** | Unix timestamp of when the deposit was created. | 
**finalized_ts** | Option<**i64**> | Unix timestamp of when the deposit was finalized (applied, failed, or returned). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


