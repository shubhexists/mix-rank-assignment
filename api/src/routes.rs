use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::db_queries::{self, get_all_sdks, ExampleResponse, Response};

#[derive(Deserialize, Serialize)]
pub struct Churns {
    pub slugs: Vec<String>,
}

pub async fn churns(churns: web::Json<Churns>) -> impl Responder {
    let slugs: &Vec<String> = &churns.slugs;
    let slug_a_to_slug_b: Vec<Response> =
        db_queries::from_slug_a_to_slug_b(slugs.to_vec()).unwrap();
    let from_none_to_slug: Vec<Response> = db_queries::from_none_to_slug(slugs.to_vec()).unwrap();
    let from_slug_to_none: Vec<Response> = db_queries::from_slug_to_none(slugs.to_vec()).unwrap();
    let from_none_to_none: Vec<Response> = db_queries::from_none_to_none(slugs.to_vec()).unwrap();
    let from_slug_to_itself: Vec<Response> =
        db_queries::from_slug_to_itself(slugs.to_vec()).unwrap();

    let res: Vec<Response> = [
        slug_a_to_slug_b,
        from_none_to_none,
        from_none_to_slug,
        from_slug_to_itself,
        from_slug_to_none,
    ]
    .concat();

    HttpResponse::Ok().json(res)
}

#[derive(Deserialize, Serialize)]
pub struct Examples {
    pub from_sdk_slug: String,
    pub to_sdk_slug: String,
    pub slugs: Vec<String>,
}

pub async fn examples(examples: web::Json<Examples>) -> impl Responder {
    let from_sdk_slug: &str = &examples.from_sdk_slug;
    let to_sdk_slug: &str = &examples.to_sdk_slug;
    let slugs: &Vec<String> = &examples.slugs;
    let res: ExampleResponse;
    match (from_sdk_slug, to_sdk_slug) {
        ("None", "None") => {
            res = db_queries::examples_from_none_to_none(slugs.to_vec()).unwrap();
        }
        ("None", _) => {
            res = db_queries::examples_from_none_to_slug(to_sdk_slug.to_string(), slugs.to_vec())
                .unwrap();
        }
        (_, "None") => {
            res = db_queries::examples_from_slug_to_none(from_sdk_slug.to_string(), slugs.to_vec())
                .unwrap();
        }
        (a, b) if a == b => {
            res = db_queries::examples_from_slug_to_itself(from_sdk_slug.to_string()).unwrap();
        }
        _ => {
            res = db_queries::examples_from_slug_a_to_slug_b(
                from_sdk_slug.to_string(),
                to_sdk_slug.to_string(),
            )
            .unwrap();
        }
    }
    HttpResponse::Ok().json(res)
}

pub async fn get_sdks() -> impl Responder {
    let response: Vec<db_queries::Sdk> = get_all_sdks().unwrap();
    HttpResponse::Ok().json(response)
}
