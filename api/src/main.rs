use actix_web::{web, App, HttpServer};
use routes::{churns, examples};
use std::io;
mod db_queries;
mod routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/churns", web::post().to(churns))
            .route("/examples", web::post().to(examples))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
