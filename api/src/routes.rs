use actix_web::{HttpResponse, Responder};
use diesel::prelude::SqliteConnection;

use crate::db_queries::establish_connection;

pub async fn churns() -> impl Responder {
    let connection: SqliteConnection = establish_connection();
    HttpResponse::Ok().body("Hello world")
}

pub async fn examples() -> impl Responder {
    let connection: SqliteConnection = establish_connection();
    HttpResponse::Ok().body("Hello world")
}
