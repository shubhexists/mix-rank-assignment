use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::db_queries::{self, Response};

#[derive(Deserialize)]
pub struct Churns {
    slugs: Vec<String>,
}

pub async fn churns(churns: web::Json<Churns>) -> impl Responder {
    let slugs: &Vec<String> = &churns.slugs;
    let slug_a_to_slug_b: Vec<Response> = db_queries::slug_a_to_slug_b(slugs.to_vec()).unwrap();
    let from_none_to_slug: Vec<Response> = db_queries::from_none_to_slug(slugs.to_vec()).unwrap();
    let from_slug_to_none: Vec<Response> = db_queries::from_slug_to_none(slugs.to_vec()).unwrap();
    let from_none_to_none: Vec<Response> = db_queries::from_none_to_none(slugs.to_vec()).unwrap();
    let from_slug_to_itself: Vec<Response> =
        db_queries::from_slug_to_itself(slugs.to_vec()).unwrap();

    HttpResponse::Ok().body("Hello world")
}

pub async fn examples() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}
