extern crate dotenv;

pub mod models;
pub mod schema;

use self::models::{NewPaste, Paste};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenv::dotenv;
use std::env;
use std::ops::Deref;
use rocket::http::Status;
use rocket::Outcome;
use rocket::request::{self, FromRequest};
use rocket::Request;
use rocket::State;

type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> PostgresPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("db_pool")
}

pub struct DbConn(pub PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PostgresPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn create_paste(conn: &PgConnection, paste: &str) -> Paste {
    let new_paste = NewPaste { paste };

    diesel::insert_into(schema::pastes::table)
        .values(&new_paste)
        .get_result(conn)
        .expect("Error saving new paste")
}
