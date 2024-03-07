# Mix Rank Assignment
This README Section would contain a log of my thoughts of my approach of the problem. 
The requirements can be found in the REQUIREMENTS.md.

## UNDERSTANDING THE PROJECT 
- I went through the data in the `data.db` SQLite file. It had 3 tables - 
    1) `app` - It consists of the applications we need to crawl 
    2) `sdk` - List of SDKs we need to check for each Application
    3) `app_sdk` - It has a list of `sdk_id` and `app_id` that is actually used in the application.

## STRUCTURE

`app` -
```py
(0, 'id', 'int', 0, None, 1)
(1, 'name', 'text', 0, None, 0)
(2, 'company_url', 'text', 0, None, 0)
(3, 'release_date', 'date', 0, None, 0)
(4, 'genre_id', 'int', 0, None, 0)
(5, 'artwork_large_url', 'text', 0, None, 0)
(6, 'seller_name', 'text', 0, None, 0)
(7, 'five_star_ratings', 'int', 0, None, 0)
(8, 'four_star_ratings', 'int', 0, None, 0)
(9, 'three_star_ratings', 'int', 0, None, 0)
(10, 'two_star_ratings', 'int', 0, None, 0)
(11, 'one_star_ratings', 'int', 0, None, 0)
```

`sdk` - 
```py
(0, 'id', 'int', 0, None, 1)
(1, 'name', 'text', 0, None, 0)
(2, 'slug', 'text', 0, None, 0)
(3, 'url', 'text', 0, None, 0)
(4, 'description', 'text', 0, None, 0)
```

`app_sdk` - 
```py
(0, 'app_id', 'int', 0, None, 1)
(1, 'sdk_id', 'int', 0, None, 2)
(2, 'installed', 'boolean', 0, None, 0)
```

## API
- Initial interation will be in NodeJS as for me it is the 
fastest to iterate with rather than other faster languages.
Will try to switch CPU intensive parts to efficient languages later.

