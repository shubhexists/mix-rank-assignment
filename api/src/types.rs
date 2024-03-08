use diesel::Queryable;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct App {
    id: i32,
    name: String,
    company_url: String,
    release_date: String,
    genre_id: i32,
    artwork_large_url: String,
    seller_name: String,
    five_star_ratings: i32,
    four_star_ratings: i32,
    three_star_ratings: i32,
    two_star_ratings: i32,
    one_star_ratings: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Sdk {
    id: i32,
    name: String,
    slug: String,
    url: String,
    description: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct AppSdk {
    app_id: i32,
    sdk_id: i32,
    installed: bool,
}
