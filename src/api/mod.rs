extern crate hashids;

use self::hashids::HashIds;
use crate::database::actions::pastes;
use crate::database::DbConn;
use rocket::Data;
use std::io::Read;

#[get("/docs")]
fn index() -> &'static str {
    "\
    USAGE

        POST api/v1/
            accepts raw data in teh body of the request and responds with a URL of
            a page containing the body's content

        GET api/v1/<id>
            retrieves the content for the paste with id `<id>`
    "
}

#[post("/", data = "<paste_data>")]
fn upload(paste_data: Data, conn: DbConn) -> Option<String> {
    let mut buffer = String::new();
    paste_data.open().read_to_string(&mut buffer).unwrap();
    match pastes::create_paste(&*conn, &buffer) {
        Some(v) => {
            let url = format!(
                "{host}/{hash}",
                host = "http://localhost:8000/api",
                hash = v.hash?
            );

            Some(url)
        }
        None => None,
    }
}

#[get("/<hash_string>")]
fn retrieve(hash_string: String, conn: DbConn) -> String {
    match pastes::retrieve_paste(&*conn, hash_string) {
        Some(v) => v.paste,
        None => String::from("Error: Paste Not Found"),
    }
}
