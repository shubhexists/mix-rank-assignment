use actix_web::{HttpResponse, Responder};
pub async fn churns() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

pub async fn examples() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}
