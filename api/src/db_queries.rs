use diesel::prelude::*;

pub fn establish_connection() -> SqliteConnection {
    let database_url = "../database/data.db";
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
