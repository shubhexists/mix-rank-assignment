use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use routes::{churns, examples, get_sdks};
use std::io;
mod db_queries;
mod routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/churns", web::post().to(churns))
            .route("/examples", web::post().to(examples))
            .route("/sdks", web::get().to(get_sdks))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
