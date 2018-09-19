extern crate hashids;

use hashids::HashIds;
use rocket::Data;
use std::io::{Read, Result};
use crate::database;

#[get("/")]
fn index() -> &'static str {
    "\
    USAGE

        POST /
            accepts raw data in teh body of the request and responds with a URL of
            a page containing the body's content

        GET /<id>
            retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String> {
    let mut buffer = String::new();
    paste.open().read_to_string(&mut buffer)?;
    let connection = database::establish_connection();
    let post = database::create_paste(&connection, &buffer);
    let url = format!(
        "{host}/{hash}",
        host = "http://localhost:8000",
        hash = post.hash
    );

    Ok(url)
}

#[get("/<hash_string>")]
fn retrieve(hash_string: String) -> Option<String> {
    use crate::database::models::*;
    use crate::database::schema::pastes::dsl::*;
    use diesel::prelude::*;

    let ids_some = HashIds::new_with_salt_and_min_length("the answer is 42".to_string(), 10);
    let ids = match ids_some {
        Ok(v) => v,
        Err(_e) => {
            println!("error");
            return None;
        }
    };

    let mut longs = ids.decode(hash_string);
    let request_id = match longs.pop() {
        Some(v) => v as i32,
        None => return None,
    };

    let connection = database::establish_connection();
    let mut results = pastes
        .filter(id.eq(request_id))
        .load::<Paste>(&connection)
        .expect("Error loading pastes");

    match results.pop() {
        Some(v) => Some(v.paste),
        None => None,
    }
}
