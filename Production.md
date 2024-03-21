# How to update data without a page refresh 

We will have a redis datastore running with a value {"new_data" : 0}. 
Whenever the database is updated with the newer values, make "new_data" -> 1 .
The client will maintain a web socket connection with this value and whenever this value is 1, 
the data on the client will refresh and "new_data" will be set to 0 again.

# Performance upgrades 

1) Have a query caching layer - 
Currently, every time a user clicks on an example, it will send a seperate request.
This is not an appropriate approach as the examples are not something that needs to be queried for every click. 
We will have a caching redis layer, which handles caching thereby preventing repeatitive querying.

2) Concurrency - 
```rs
 let slug_a_to_slug_b: Vec<Response> =
        db_queries::from_slug_a_to_slug_b(slugs.to_vec()).unwrap();
    let from_none_to_slug: Vec<Response> = db_queries::from_none_to_slug(slugs.to_vec()).unwrap();
    let from_slug_to_none: Vec<Response> = db_queries::from_slug_to_none(slugs.to_vec()).unwrap();
    let from_none_to_none: Vec<Response> = db_queries::from_none_to_none(slugs.to_vec()).unwrap();
    let from_slug_to_itself: Vec<Response> =
        db_queries::from_slug_to_itself(slugs.to_vec()).unwrap();
```

These are independent queries made on the database and have no relation to one another. 
Hence, these can be made concurrent for performant results.
(I am not very confident in concurrent Rust)

# Scaling upgrades

1) Database when it becomes too large for the queries to run efficienty(fast), can be further splitted into smaller sets 
with a specified `app_id` and `sdk_id`, to make the query time shorter.
