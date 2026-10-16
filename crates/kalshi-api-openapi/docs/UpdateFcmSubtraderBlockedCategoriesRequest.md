# UpdateFcmSubtraderBlockedCategoriesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subtrader_id** | **String** | The subtrader whose blocked categories should be updated. Must belong to the requesting FCM. | 
**category** | **String** | A single event category to add to or remove from the blocked set, 1-100 characters (e.g. \"Politics\"). | 
**blocked** | **bool** | True adds the category to the blocked set; false removes it. Removing a category that is not blocked is a no-op. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


