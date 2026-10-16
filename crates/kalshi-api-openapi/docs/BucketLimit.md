# BucketLimit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**refill_rate** | **i64** | Tokens added to the bucket per second. | 
**bucket_capacity** | **i64** | Maximum tokens the bucket can hold. When equal to refill_rate the bucket holds one second of budget; larger values represent burst headroom that idle clients accumulate and can spend in a single pulse (e.g. write buckets at non-Basic tiers hold two seconds of budget).  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


