# FRED® API

All URIs are relative to *https://api.stlouisfed.org*

| Method                                                                                | HTTP request                             | Description                     |
| ------------------------------------------------------------------------------------- | ---------------------------------------- | ------------------------------- |
| [**fred_category_children_get**](API.md#fred_category_children_get)                   | **GET** /fred/category/children          | Get child categories            |
| [**fred_category_get**](API.md#fred_category_get)                                     | **GET** /fred/category                   | Get a category                  |
| [**fred_category_related_get**](API.md#fred_category_related_get)                     | **GET** /fred/category/related           | Get related categories          |
| [**fred_category_related_tags_get**](API.md#fred_category_related_tags_get)           | **GET** /fred/category/related_tags      | Get related category tags       |
| [**fred_category_series_get**](API.md#fred_category_series_get)                       | **GET** /fred/category/series            | Get series in a category        |
| [**fred_category_tags_get**](API.md#fred_category_tags_get)                           | **GET** /fred/category/tags              | Get category tags               |
| [**fred_related_tags_get**](API.md#fred_related_tags_get)                             | **GET** /fred/related_tags               | Get related tags                |
| [**fred_release_dates_get**](API.md#fred_release_dates_get)                           | **GET** /fred/release/dates              | Get release dates for a release |
| [**fred_release_get**](API.md#fred_release_get)                                       | **GET** /fred/release                    | Get a release                   |
| [**fred_release_related_tags_get**](API.md#fred_release_related_tags_get)             | **GET** /fred/release/related_tags       | Get related release tags        |
| [**fred_release_series_get**](API.md#fred_release_series_get)                         | **GET** /fred/release/series             | Get release series              |
| [**fred_release_sources_get**](API.md#fred_release_sources_get)                       | **GET** /fred/release/sources            | Get release sources             |
| [**fred_release_tables_get**](API.md#fred_release_tables_get)                         | **GET** /fred/release/tables             | Get release tables              |
| [**fred_release_tags_get**](API.md#fred_release_tags_get)                             | **GET** /fred/release/tags               | Get release tags                |
| [**fred_releases_dates_get**](API.md#fred_releases_dates_get)                         | **GET** /fred/releases/dates             | Get release dates               |
| [**fred_releases_get**](API.md#fred_releases_get)                                     | **GET** /fred/releases                   | Get all releases                |
| [**fred_series_categories_get**](API.md#fred_series_categories_get)                   | **GET** /fred/series/categories          | Get series categories           |
| [**fred_series_get**](API.md#fred_series_get)                                         | **GET** /fred/series                     | Get a series                    |
| [**fred_series_observations_get**](API.md#fred_series_observations_get)               | **GET** /fred/series/observations        | Get series observations         |
| [**fred_series_release_get**](API.md#fred_series_release_get)                         | **GET** /fred/series/release             | Get series release              |
| [**fred_series_search_get**](API.md#fred_series_search_get)                           | **GET** /fred/series/search              | Search series                   |
| [**fred_series_search_related_tags_get**](API.md#fred_series_search_related_tags_get) | **GET** /fred/series/search/related_tags | Search series related tags      |
| [**fred_series_search_tags_get**](API.md#fred_series_search_tags_get)                 | **GET** /fred/series/search/tags         | Search series tags              |
| [**fred_series_tags_get**](API.md#fred_series_tags_get)                               | **GET** /fred/series/tags                | Get series tags                 |
| [**fred_series_updates_get**](API.md#fred_series_updates_get)                         | **GET** /fred/series/updates             | Get series updates              |
| [**fred_series_vintagedates_get**](API.md#fred_series_vintagedates_get)               | **GET** /fred/series/vintagedates        | Get series vintage dates        |
| [**fred_source_get**](API.md#fred_source_get)                                         | **GET** /fred/source                     | Get a source                    |
| [**fred_source_releases_get**](API.md#fred_source_releases_get)                       | **GET** /fred/source/releases            | Get source releases             |
| [**fred_sources_get**](API.md#fred_sources_get)                                       | **GET** /fred/sources                    | Get all sources                 |
| [**fred_tags_get**](API.md#fred_tags_get)                                             | **GET** /fred/tags                       | Get FRED tags                   |
| [**fred_tags_series_get**](API.md#fred_tags_series_get)                               | **GET** /fred/tags/series                | Get tagged series               |
| [**geofred_regional_data_get**](API.md#geofred_regional_data_get)                     | **GET** /geofred/regional/data           | Get regional data               |
| [**geofred_series_data_get**](API.md#geofred_series_data_get)                         | **GET** /geofred/series/data             | Get series data info            |
| [**geofred_series_group_get**](API.md#geofred_series_group_get)                       | **GET** /geofred/series/group            | Get series group info           |
| [**geofred_shape_files_get**](API.md#geofred_shape_files_get)                         | **GET** /geofred/shape/files             | Get shape files                 |

## fred_category_children_get

> fred_category_children_get(api_key, file_type, realtime_start, realtime_end, category_id)
> Get child categories

Get the child categories for a specified parent category

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |
| **category_id**    | Option<**i32**>    | The ID for a category                       |            | [default to 0]   |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_category_get

> String fred_category_get(api_key, file_type, realtime_start, realtime_end, category_id)
> Get a category

Get information about a specific category

### Parameters

| Name               | Type               | Description                                      | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------------ | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string      | [required] |
| **file_type**      | Option<**String**> | Response format                                  |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)       |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)         |            |
| **category_id**    | Option<**i32**>    | The ID for a category (default is root category) |            | [default to 0]   |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/xml, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_category_related_get

> fred_category_related_get(api_key, category_id, file_type, realtime_start, realtime_end)
> Get related categories

Get the related categories for a category

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **category_id**    | **i32**            | The ID for a category                       | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_category_related_tags_get

> fred_category_related_tags_get(api_key, category_id, tag_names, file_type, realtime_start, realtime_end, limit, offset, sort_order, exclude_tag_names, tag_group_id, search_text, order_by)
> Get related category tags

Get the related FRED tags for one or more FRED tags within a category

### Parameters

| Name                  | Type               | Description                                      | Required   | Notes             |
| --------------------- | ------------------ | ------------------------------------------------ | ---------- | ----------------- |
| **api_key**           | **String**         | 32 character alpha-numeric lowercase string      | [required] |
| **category_id**       | **i32**            | The ID for a category                            | [required] |
| **tag_names**         | **String**         | Semicolon-delimited list of tag names            | [required] |
| **file_type**         | Option<**String**> | Response format                                  |            | [default to xml]  |
| **realtime_start**    | Option<**String**> | Start of the real-time period (YYYY-MM-DD)       |            |
| **realtime_end**      | Option<**String**> | End of the real-time period (YYYY-MM-DD)         |            |
| **limit**             | Option<**i32**>    | Maximum number of results to return              |            | [default to 1000] |
| **offset**            | Option<**i32**>    | Number of results to skip                        |            | [default to 0]    |
| **sort_order**        | Option<**String**> | Sort order                                       |            | [default to asc]  |
| **exclude_tag_names** | Option<**String**> | Semicolon-delimited list of tag names to exclude |            |
| **tag_group_id**      | Option<**String**> | Tag group ID                                     |            |
| **search_text**       | Option<**String**> | Text to search for                               |            |
| **order_by**          | Option<**String**> | Variable to order results by                     |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_category_series_get

> fred_category_series_get(api_key, category_id, file_type, realtime_start, realtime_end, limit, offset, sort_order, order_by, filter_variable, filter_value, tag_names, exclude_tag_names)
> Get series in a category

Get the series in a category

### Parameters

| Name                  | Type               | Description                                      | Required   | Notes             |
| --------------------- | ------------------ | ------------------------------------------------ | ---------- | ----------------- |
| **api_key**           | **String**         | 32 character alpha-numeric lowercase string      | [required] |
| **category_id**       | **i32**            | The ID for a category                            | [required] |
| **file_type**         | Option<**String**> | Response format                                  |            | [default to xml]  |
| **realtime_start**    | Option<**String**> | Start of the real-time period (YYYY-MM-DD)       |            |
| **realtime_end**      | Option<**String**> | End of the real-time period (YYYY-MM-DD)         |            |
| **limit**             | Option<**i32**>    | Maximum number of results to return              |            | [default to 1000] |
| **offset**            | Option<**i32**>    | Number of results to skip                        |            | [default to 0]    |
| **sort_order**        | Option<**String**> | Sort order                                       |            | [default to asc]  |
| **order_by**          | Option<**String**> | Variable to order results by                     |            |
| **filter_variable**   | Option<**String**> | Variable to filter results by                    |            |
| **filter_value**      | Option<**String**> | Value to filter by                               |            |
| **tag_names**         | Option<**String**> | Semicolon-delimited list of tag names            |            |
| **exclude_tag_names** | Option<**String**> | Semicolon-delimited list of tag names to exclude |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_category_tags_get

> fred_category_tags_get(api_key, category_id, file_type, realtime_start, realtime_end, limit, offset, sort_order, tag_names, tag_group_id, search_text, order_by)
> Get category tags

Get the FRED tags for a category

### Parameters

| Name               | Type               | Description                                 | Required   | Notes             |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ----------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **category_id**    | **i32**            | The ID for a category                       | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml]  |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |
| **limit**          | Option<**i32**>    | Maximum number of results to return         |            | [default to 1000] |
| **offset**         | Option<**i32**>    | Number of results to skip                   |            | [default to 0]    |
| **sort_order**     | Option<**String**> | Sort order                                  |            | [default to asc]  |
| **tag_names**      | Option<**String**> | Semicolon-delimited list of tag names       |            |
| **tag_group_id**   | Option<**String**> | Tag group ID                                |            |
| **search_text**    | Option<**String**> | Text to search for                          |            |
| **order_by**       | Option<**String**> | Variable to order results by                |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_related_tags_get

> fred_related_tags_get(api_key, tag_names, file_type, realtime_start, realtime_end)
> Get related tags

Get related FRED tags for one or more FRED tags

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **tag_names**      | **String**         | Semicolon-delimited list of tag names       | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_release_dates_get

> fred_release_dates_get(api_key, release_id, file_type, realtime_start, realtime_end, sort_order, limit, offset, include_release_dates_with_no_data)
> Get release dates for a release

Get release dates for a release of economic data

### Parameters

| Name                                   | Type               | Description                                 | Required   | Notes              |
| -------------------------------------- | ------------------ | ------------------------------------------- | ---------- | ------------------ |
| **api_key**                            | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **release_id**                         | **i32**            | The ID for a release                        | [required] |
| **file_type**                          | Option<**String**> | Response format                             |            | [default to xml]   |
| **realtime_start**                     | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**                       | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |
| **sort_order**                         | Option<**String**> | Sort order                                  |            | [default to asc]   |
| **limit**                              | Option<**i32**>    | Maximum number of results                   |            | [default to 10000] |
| **offset**                             | Option<**i32**>    | Number of results to skip                   |            | [default to 0]     |
| **include_release_dates_with_no_data** | Option<**String**> | Include release dates with no data          |            | [default to false] |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_release_get

> fred_release_get(api_key, release_id, file_type, realtime_start, realtime_end)
> Get a release

Get a release of economic data

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **release_id**     | **i32**            | The ID for a release                        | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_release_related_tags_get

> fred_release_related_tags_get(api_key, release_id, tag_names, file_type, realtime_start, realtime_end, limit, offset, sort_order, exclude_tag_names, tag_group_id, search_text, order_by)
> Get related release tags

Get the related FRED tags for one or more FRED tags within a release

### Parameters

| Name                  | Type               | Description                                      | Required   | Notes             |
| --------------------- | ------------------ | ------------------------------------------------ | ---------- | ----------------- |
| **api_key**           | **String**         | 32 character alpha-numeric lowercase string      | [required] |
| **release_id**        | **i32**            | The ID for a release                             | [required] |
| **tag_names**         | **String**         | Semicolon-delimited list of tag names            | [required] |
| **file_type**         | Option<**String**> | Response format                                  |            | [default to xml]  |
| **realtime_start**    | Option<**String**> | Start of the real-time period (YYYY-MM-DD)       |            |
| **realtime_end**      | Option<**String**> | End of the real-time period (YYYY-MM-DD)         |            |
| **limit**             | Option<**i32**>    | Maximum number of results to return              |            | [default to 1000] |
| **offset**            | Option<**i32**>    | Number of results to skip                        |            | [default to 0]    |
| **sort_order**        | Option<**String**> | Sort order                                       |            | [default to asc]  |
| **exclude_tag_names** | Option<**String**> | Semicolon-delimited list of tag names to exclude |            |
| **tag_group_id**      | Option<**String**> | Tag group ID                                     |            |
| **search_text**       | Option<**String**> | Text to search for                               |            |
| **order_by**          | Option<**String**> | Variable to order results by                     |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_release_series_get

> fred_release_series_get(api_key, release_id, file_type, realtime_start, realtime_end, limit, offset, sort_order, order_by, filter_variable, filter_value, tag_names, exclude_tag_names)
> Get release series

Get the series on a release of economic data

### Parameters

| Name                  | Type               | Description                                      | Required   | Notes             |
| --------------------- | ------------------ | ------------------------------------------------ | ---------- | ----------------- |
| **api_key**           | **String**         | 32 character alpha-numeric lowercase string      | [required] |
| **release_id**        | **i32**            | The ID for a release                             | [required] |
| **file_type**         | Option<**String**> | Response format                                  |            | [default to xml]  |
| **realtime_start**    | Option<**String**> | Start of the real-time period (YYYY-MM-DD)       |            |
| **realtime_end**      | Option<**String**> | End of the real-time period (YYYY-MM-DD)         |            |
| **limit**             | Option<**i32**>    | Maximum number of results to return              |            | [default to 1000] |
| **offset**            | Option<**i32**>    | Number of results to skip                        |            | [default to 0]    |
| **sort_order**        | Option<**String**> | Sort order                                       |            | [default to asc]  |
| **order_by**          | Option<**String**> | Variable to order results by                     |            |
| **filter_variable**   | Option<**String**> | Variable to filter results by                    |            |
| **filter_value**      | Option<**String**> | Value to filter by                               |            |
| **tag_names**         | Option<**String**> | Semicolon-delimited list of tag names            |            |
| **exclude_tag_names** | Option<**String**> | Semicolon-delimited list of tag names to exclude |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_release_sources_get

> fred_release_sources_get(api_key, release_id, file_type, realtime_start, realtime_end)
> Get release sources

Get the sources for a release of economic data

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **release_id**     | **i32**            | The ID for a release                        | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_release_tables_get

> fred_release_tables_get(api_key, release_id, file_type, realtime_start, realtime_end, element_id, include_observation_values, observation_date)
> Get release tables

Get release table trees for a given release

### Parameters

| Name                           | Type               | Description                                 | Required   | Notes                                     |
| ------------------------------ | ------------------ | ------------------------------------------- | ---------- | ----------------------------------------- |
| **api_key**                    | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **release_id**                 | **i32**            | The ID for a release                        | [required] |
| **file_type**                  | Option<**String**> | Response format                             |            | [default to xml]                          |
| **realtime_start**             | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**               | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |
| **element_id**                 | Option<**i32**>    | The release table element ID                |            |
| **include_observation_values** | Option<**String**> | Include observation values                  |            | [default to false]                        |
| **observation_date**           | Option<**String**> | Date of observation                         |            | [default to Fri Dec 31 00:00:00 EET 9999] |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_release_tags_get

> fred_release_tags_get(api_key, release_id, file_type, realtime_start, realtime_end, limit, offset, sort_order, tag_names, tag_group_id, search_text, order_by)
> Get release tags

Get the FRED tags for a release

### Parameters

| Name               | Type               | Description                                 | Required   | Notes             |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ----------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **release_id**     | **i32**            | The ID for a release                        | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml]  |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |
| **limit**          | Option<**i32**>    | Maximum number of results to return         |            | [default to 1000] |
| **offset**         | Option<**i32**>    | Number of results to skip                   |            | [default to 0]    |
| **sort_order**     | Option<**String**> | Sort order                                  |            | [default to asc]  |
| **tag_names**      | Option<**String**> | Semicolon-delimited list of tag names       |            |
| **tag_group_id**   | Option<**String**> | Tag group ID                                |            |
| **search_text**    | Option<**String**> | Text to search for                          |            |
| **order_by**       | Option<**String**> | Variable to order results by                |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_releases_dates_get

> fred_releases_dates_get(api_key, file_type, realtime_start, realtime_end, limit, offset, sort_order, order_by, include_release_dates_with_no_data)
> Get release dates

Get release dates for all releases of economic data

### Parameters

| Name                                   | Type               | Description                                 | Required   | Notes              |
| -------------------------------------- | ------------------ | ------------------------------------------- | ---------- | ------------------ |
| **api_key**                            | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **file_type**                          | Option<**String**> | Response format                             |            | [default to xml]   |
| **realtime_start**                     | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**                       | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |
| **limit**                              | Option<**i32**>    | Maximum number of results to return         |            | [default to 1000]  |
| **offset**                             | Option<**i32**>    | Number of results to skip                   |            | [default to 0]     |
| **sort_order**                         | Option<**String**> | Sort order                                  |            | [default to desc]  |
| **order_by**                           | Option<**String**> | Variable to order results by                |            |
| **include_release_dates_with_no_data** | Option<**String**> | Include release dates with no data          |            | [default to false] |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_releases_get

> fred_releases_get(api_key, file_type, realtime_start, realtime_end, limit, offset, sort_order, order_by)
> Get all releases

Get all releases of economic data

### Parameters

| Name               | Type               | Description                                 | Required   | Notes             |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ----------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml]  |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |
| **limit**          | Option<**i32**>    | Maximum number of results to return         |            | [default to 1000] |
| **offset**         | Option<**i32**>    | Number of results to skip                   |            | [default to 0]    |
| **sort_order**     | Option<**String**> | Sort order                                  |            | [default to asc]  |
| **order_by**       | Option<**String**> | Variable to order results by                |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_series_categories_get

> fred_series_categories_get(api_key, series_id, file_type, realtime_start, realtime_end)
> Get series categories

Get the categories for an economic data series

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **series_id**      | **String**         | The ID for a series                         | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_series_get

> fred_series_get(api_key, series_id, file_type, realtime_start, realtime_end)
> Get a series

Get an economic data series

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **series_id**      | **String**         | The ID for a series                         | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_series_observations_get

> fred_series_observations_get(api_key, series_id, file_type, realtime_start, realtime_end, sort_order, limit, offset, observation_start, observation_end, units, frequency, aggregation_method, output_type, vintage_dates)
> Get series observations

Get the observations or data values for an economic data series

### Parameters

| Name                   | Type               | Description                                                     | Required   | Notes                                     |
| ---------------------- | ------------------ | --------------------------------------------------------------- | ---------- | ----------------------------------------- |
| **api_key**            | **String**         | 32 character alpha-numeric lowercase string                     | [required] |
| **series_id**          | **String**         | The ID for a series                                             | [required] |
| **file_type**          | Option<**String**> | Response format (supports additional formats for this endpoint) |            | [default to xml]                          |
| **realtime_start**     | Option<**String**> | Start of the real-time period (YYYY-MM-DD)                      |            |
| **realtime_end**       | Option<**String**> | End of the real-time period (YYYY-MM-DD)                        |            |
| **sort_order**         | Option<**String**> | Sort order                                                      |            | [default to asc]                          |
| **limit**              | Option<**i32**>    | Maximum number of results                                       |            | [default to 100000]                       |
| **offset**             | Option<**i32**>    | Number of results to skip                                       |            | [default to 0]                            |
| **observation_start**  | Option<**String**> | Start date for observations                                     |            | [default to Thu Jul 04 00:00:00 EET 1776] |
| **observation_end**    | Option<**String**> | End date for observations                                       |            | [default to Fri Dec 31 00:00:00 EET 9999] |
| **units**              | Option<**String**> | Data transformation                                             |            | [default to lin]                          |
| **frequency**          | Option<**String**> | Data frequency                                                  |            |
| **aggregation_method** | Option<**String**> | Aggregation method                                              |            | [default to avg]                          |
| **output_type**        | Option<**i32**>    | Output type                                                     |            | [default to 1]                            |
| **vintage_dates**      | Option<**String**> | Comma-separated list of vintage dates                           |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_series_release_get

> fred_series_release_get(api_key, series_id, file_type, realtime_start, realtime_end)
> Get series release

Get the release for an economic data series

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **series_id**      | **String**         | The ID for a series                         | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_series_search_get

> fred_series_search_get(api_key, search_text, file_type, realtime_start, realtime_end, limit, offset, search_type, order_by, sort_order, filter_variable, filter_value, tag_names, exclude_tag_names)
> Search series

Get economic data series that match search text

### Parameters

| Name                  | Type               | Description                                      | Required   | Notes                  |
| --------------------- | ------------------ | ------------------------------------------------ | ---------- | ---------------------- |
| **api_key**           | **String**         | 32 character alpha-numeric lowercase string      | [required] |
| **search_text**       | **String**         | Text to search for                               | [required] |
| **file_type**         | Option<**String**> | Response format                                  |            | [default to xml]       |
| **realtime_start**    | Option<**String**> | Start of the real-time period (YYYY-MM-DD)       |            |
| **realtime_end**      | Option<**String**> | End of the real-time period (YYYY-MM-DD)         |            |
| **limit**             | Option<**i32**>    | Maximum number of results to return              |            | [default to 1000]      |
| **offset**            | Option<**i32**>    | Number of results to skip                        |            | [default to 0]         |
| **search_type**       | Option<**String**> | Type of search                                   |            | [default to full_text] |
| **order_by**          | Option<**String**> | Variable to order results by                     |            |
| **sort_order**        | Option<**String**> | Sort order                                       |            |
| **filter_variable**   | Option<**String**> | Variable to filter results by                    |            |
| **filter_value**      | Option<**String**> | Value to filter by                               |            |
| **tag_names**         | Option<**String**> | Semicolon-delimited list of tag names            |            |
| **exclude_tag_names** | Option<**String**> | Semicolon-delimited list of tag names to exclude |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_series_search_related_tags_get

> fred_series_search_related_tags_get(api_key, series_search_text, tag_names, file_type, realtime_start, realtime_end)
> Search series related tags

Get the related FRED tags for one or more FRED tags within a series search

### Parameters

| Name                   | Type               | Description                                 | Required   | Notes            |
| ---------------------- | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**            | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **series_search_text** | **String**         | Text to search for                          | [required] |
| **tag_names**          | **String**         | Semicolon-delimited list of tag names       | [required] |
| **file_type**          | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start**     | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**       | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_series_search_tags_get

> fred_series_search_tags_get(api_key, series_search_text, file_type, realtime_start, realtime_end)
> Search series tags

Get the FRED tags for a series search

### Parameters

| Name                   | Type               | Description                                 | Required   | Notes            |
| ---------------------- | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**            | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **series_search_text** | **String**         | Text to search for                          | [required] |
| **file_type**          | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start**     | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**       | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_series_tags_get

> fred_series_tags_get(api_key, series_id, file_type, realtime_start, realtime_end)
> Get series tags

Get the FRED tags for a series

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **series_id**      | **String**         | The ID for a series                         | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_series_updates_get

> fred_series_updates_get(api_key, file_type, realtime_start, realtime_end, limit, offset)
> Get series updates

Get economic data series sorted by when observations were updated

### Parameters

| Name               | Type               | Description                                 | Required   | Notes             |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ----------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml]  |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |
| **limit**          | Option<**i32**>    | Maximum number of results to return         |            | [default to 1000] |
| **offset**         | Option<**i32**>    | Number of results to skip                   |            | [default to 0]    |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_series_vintagedates_get

> fred_series_vintagedates_get(api_key, series_id, file_type, realtime_start, realtime_end)
> Get series vintage dates

Get the dates in history when a series' data values were revised or new data values were released

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **series_id**      | **String**         | The ID for a series                         | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_source_get

> fred_source_get(api_key, source_id, file_type, realtime_start, realtime_end)
> Get a source

Get a source of economic data

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **source_id**      | **i32**            | The ID for a source                         | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_source_releases_get

> fred_source_releases_get(api_key, source_id, file_type, realtime_start, realtime_end)
> Get source releases

Get the releases for a source

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **source_id**      | **i32**            | The ID for a source                         | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_sources_get

> fred_sources_get(api_key, file_type, realtime_start, realtime_end)
> Get all sources

Get all sources of economic data

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_tags_get

> fred_tags_get(api_key, file_type, realtime_start, realtime_end, limit, offset, sort_order)
> Get FRED tags

Get FRED tags

### Parameters

| Name               | Type               | Description                                 | Required   | Notes             |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ----------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml]  |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |
| **limit**          | Option<**i32**>    | Maximum number of results to return         |            | [default to 1000] |
| **offset**         | Option<**i32**>    | Number of results to skip                   |            | [default to 0]    |
| **sort_order**     | Option<**String**> | Sort order                                  |            | [default to asc]  |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## fred_tags_series_get

> fred_tags_series_get(api_key, tag_names, file_type, realtime_start, realtime_end)
> Get tagged series

Get the series matching tags

### Parameters

| Name               | Type               | Description                                 | Required   | Notes            |
| ------------------ | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**        | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **tag_names**      | **String**         | Semicolon-delimited list of tag names       | [required] |
| **file_type**      | Option<**String**> | Response format                             |            | [default to xml] |
| **realtime_start** | Option<**String**> | Start of the real-time period (YYYY-MM-DD)  |            |
| **realtime_end**   | Option<**String**> | End of the real-time period (YYYY-MM-DD)    |            |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## geofred_regional_data_get

> geofred_regional_data_get(api_key, file_type)
> Get regional data

Regional Data - Maps API

### Parameters

| Name          | Type               | Description                                 | Required   | Notes            |
| ------------- | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**   | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **file_type** | Option<**String**> | Response format                             |            | [default to xml] |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## geofred_series_data_get

> geofred_series_data_get(api_key, series_id, file_type)
> Get series data info

Series Data Info - Maps API

### Parameters

| Name          | Type               | Description                                 | Required   | Notes            |
| ------------- | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**   | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **series_id** | **String**         | The ID for a series                         | [required] |
| **file_type** | Option<**String**> | Response format                             |            | [default to xml] |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## geofred_series_group_get

> geofred_series_group_get(api_key, series_id, file_type)
> Get series group info

Series Group Info - Maps API

### Parameters

| Name          | Type               | Description                                 | Required   | Notes            |
| ------------- | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**   | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **series_id** | **String**         | The ID for a series                         | [required] |
| **file_type** | Option<**String**> | Response format                             |            | [default to xml] |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

## geofred_shape_files_get

> geofred_shape_files_get(api_key, file_type)
> Get shape files

Shape Files - Maps API

### Parameters

| Name          | Type               | Description                                 | Required   | Notes            |
| ------------- | ------------------ | ------------------------------------------- | ---------- | ---------------- |
| **api_key**   | **String**         | 32 character alpha-numeric lowercase string | [required] |
| **file_type** | Option<**String**> | Response format                             |            | [default to xml] |

### Return type

(empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
