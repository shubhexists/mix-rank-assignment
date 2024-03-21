# MATRIX QUERIES 

## SDK A to SDK B

```sql
    SELECT sdk1.slug, sdk2.slug, COUNT(DISTINCT app_sdk_1.app_id)
    FROM app_sdk AS app_sdk_1, app_sdk AS app_sdk_2, sdk AS sdk1, sdk AS sdk2 
    WHERE app_sdk_1.app_id = app_sdk_2.app_id --> Joining the table with itself, with `app_id` param
    AND app_sdk_1.sdk_id = sdk1.id AND app_sdk_2.sdk_id = sdk2.id --> Links app_sdk to corresponding sdk
    AND app_sdk_1.installed = 0 --> a to b means a should not be there anymore
    AND app_sdk_2.installed = 1 --> and b should be present
    AND sdk1.slug IN ({}) AND sdk2.slug IN ({}) --> filter from the queries
    GROUP BY sdk1.id, sdk2.id;
```

## None to Slug (Similar: Slug to None)

```sql
    SELECT sdk2.slug, COUNT(DISTINCT app_sdk_1.app_id)
    FROM app_sdk AS app_sdk_1, app_sdk AS app_sdk_2, sdk AS sdk1, sdk AS sdk2
    WHERE app_sdk_1.app_id = app_sdk_2.app_id  --> Joining the table with itself, with `app_id` param
    AND app_sdk_1.sdk_id = sdk1.id AND app_sdk_2.sdk_id = sdk2.id --> Links app_sdk to corresponding sdk
    AND app_sdk_1.installed = false AND app_sdk_2.installed = true
    AND sdk1.slug NOT IN ({}) --> none to any slug means should not be from the available slugs
    AND sdk2.slug IN ({}) --> slug
    GROUP BY sdk2.id;
```

## Slug to itself

```sql
    SELECT sdk.slug, COUNT(DISTINCT app_sdk.app_id)
    FROM app_sdk, sdk
    WHERE app_sdk.sdk_id = sdk.id --> Joins here
    AND app_sdk.installed = true --> all rows that are installed
    AND sdk.slug IN ({}) --> and are in given SDKs
    GROUP BY sdk.id;
```

## None to None

```sql
    SELECT COUNT(app.id)
    FROM app
    WHERE NOT EXISTS ( --> all the rows that do not follow the subquery
    SELECT 1 FROM app_sdk, sdk
    WHERE app_sdk.sdk_id = sdk.id --> joins here
    AND sdk.slug IN ({}) --> all the selected slugs 
    AND app_sdk.installed = 1 --> selected slug is installed
    AND app_sdk.app_id = app.id
    );
```