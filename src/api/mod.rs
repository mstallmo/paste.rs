extern crate hashids;

use self::hashids::HashIds;
use rocket::Data;
use std::io::Read;
use crate::database;

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

//TODO: Should probably refactor this into the database crate so we're not duping code
#[post("/", data = "<paste_data>")]
fn upload(paste_data: Data, conn: database::DbConn) -> Option<String> {
    use crate::database::models::*;
    use crate::database::schema::pastes::dsl::*;
    use diesel::prelude::*;

    let ids_some = HashIds::new_with_salt_and_min_length("the answer is 42".to_string(), 10);
    let ids = match ids_some {
        Ok(v) => v,
        Err(_e) => return None
    };

    let mut buffer = String::new();
    paste_data.open().read_to_string(&mut buffer).unwrap();
    let mut post = database::create_paste(&*conn, &buffer);

    let hash_id = ids.encode(&vec![post.id as i64]);
    post = diesel::update(&post).set(hash.eq(hash_id)).get_result(&*conn).expect("Error updating paste");

    let url = format!(
        "{host}/{hash}",
        host = "http://localhost:8000/api",
        hash = post.hash?
    );

    Some(url)
}

#[get("/<hash_string>")]
fn retrieve(hash_string: String, conn: database::DbConn) -> Option<String> {
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

    let mut results = pastes
        .filter(id.eq(request_id))
        .load::<Paste>(&*conn)
        .expect("Error loading pastes");

    match results.pop() {
        Some(v) => Some(v.paste),
        None => None,
    }
}
