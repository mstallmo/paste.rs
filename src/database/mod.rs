extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::{NewPaste, Paste};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_paste<'a>(conn: &PgConnection, paste: &'a str) -> Paste {
    let new_paste = NewPaste { paste: paste };

    diesel::insert_into(schema::pastes::table)
        .values(&new_paste)
        .get_result(conn)
        .expect("Error saving new paste")
}
