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

```sql
    SELECT sdk1.slug, COUNT(DISTINCT app_sdk_1.app_id)
    FROM app_sdk AS app_sdk_1, app_sdk AS app_sdk_2, sdk AS sdk1, sdk AS sdk2
    WHERE app_sdk_1.app_id = app_sdk_2.app_id
    AND app_sdk_1.sdk_id = sdk1.id
    AND app_sdk_2.sdk_id = sdk2.id
    AND app_sdk_1.installed = True
    AND app_sdk_2.installed = False
    AND sdk1.slug IN ({})
    AND sdk2.slug NOT IN ({})
    GROUP BY
    sdk1.id;
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

# EXAMPLE QUERIES

## SDK A to SDK B

```sql
    SELECT App.id, App.name FROM app_sdk AS app_sdk_1
    JOIN app_sdk AS app_sdk_2 ON app_sdk_1.app_id = app_sdk_2.app_id --> join
    JOIN sdk AS Sdk1 ON app_sdk_1.sdk_id = Sdk1.id --> join
    JOIN sdk AS Sdk2 ON app_sdk_2.sdk_id = Sdk2.id --> join
    JOIN app AS App ON App.id = app_sdk_1.app_id --> join
    WHERE app_sdk_1.installed = False --> sdk a should not be installed now
    AND app_sdk_2.installed = True --> b should be installed
    AND Sdk1.slug = '{}'
    AND Sdk2.slug = '{}'
    LIMIT 10;
```

## None to Slug (Similar: Slug to None)

```sql
    SELECT App.id, App.name FROM app_sdk AS app_sdk_1
    JOIN app_sdk AS app_sdk_2 ON app_sdk_1.app_id = app_sdk_2.app_id  --> join
    JOIN sdk AS Sdk1 ON app_sdk_1.sdk_id = Sdk1.id --> join
    JOIN sdk AS Sdk2 ON app_sdk_2.sdk_id = Sdk2.id --> join
    JOIN app AS App ON App.id = app_sdk_1.app_id --> join
    WHERE app_sdk_1.installed = False 
    AND app_sdk_2.installed = True
    AND Sdk1.slug NOT IN ({}) --> none is the sdk that is not in selected sdks
    AND Sdk2.slug = '{}' --> sdk b
    LIMIT 10;
```

```sql
    SELECT App.id, App.name
    FROM app_sdk AS app_sdk_1
    JOIN app_sdk AS app_sdk_2 ON app_sdk_1.app_id = app_sdk_2.app_id
    JOIN sdk AS Sdk1 ON app_sdk_1.sdk_id = Sdk1.id
    JOIN sdk AS Sdk2 ON app_sdk_2.sdk_id = Sdk2.id
    JOIN app AS App ON App.id = app_sdk_1.app_id
    WHERE app_sdk_1.installed = True
    AND app_sdk_2.installed = False
    AND Sdk1.slug = '{}'
    AND Sdk2.slug NOT IN ({})
    LIMIT 10;
```

## Slug to itself

```sql
    SELECT App.id, App.name FROM app_sdk
    JOIN sdk ON app_sdk.sdk_id = sdk.id --> join
    JOIN app ON app_sdk.app_id = app.id --> join
    WHERE app_sdk.installed = 1 --> installed = true
    AND sdk.slug = '{}' --> sdk 
    LIMIT 10;
```

## None to None

```sql
    SELECT App.id, App.name FROM app
    WHERE NOT EXISTS ( --> all the records that do not satisfy below subquery
    SELECT 1 FROM app_sdk
    JOIN sdk ON app_sdk.sdk_id = sdk.id --> join
    WHERE app_sdk.installed = 1
    AND sdk.slug IN ({}) --> all the selected slugs (not satisfy will give out the `None` ones)
    AND app_sdk.app_id = app.id
    )
    LIMIT 10;
```
