use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct App {
    id: i32,
    name: String,
    company_url: String,
    release_date: Option<String>,
    genre_id: i32,
    artwork_large_url: String,
    seller_name: String,
    five_star_ratings: i32,
    four_star_ratings: i32,
    three_star_ratings: i32,
    two_star_ratings: i32,
    one_star_ratings: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Sdk {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct AppSdk {
    app_id: i32,
    sdk_id: i32,
    installed: bool,
}
