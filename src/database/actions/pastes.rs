extern crate hashids;

use self::hashids::HashIds;
use super::models::{NewPaste, Paste};
use super::schema::pastes;
use super::schema::pastes::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;

pub fn create_paste(conn: &PgConnection, paste_content: &str) -> Option<Paste> {
    let ids_some = HashIds::new_with_salt_and_min_length("the answer is 42".to_string(), 10);
    let ids = match ids_some {
        Ok(v) => v,
        Err(_e) => return None,
    };

    let new_paste = NewPaste {
        paste: paste_content,
    };

    match conn.transaction::<_, Error, _>(|| {
        let inserted_paste: Paste = diesel::insert_into(pastes::table)
            .values(&new_paste)
            .get_result(conn)
            .expect("Error saving new paste");

        let hash_id = ids.encode(&vec![inserted_paste.id as i64]);

        diesel::update(&inserted_paste)
            .set(hash.eq(hash_id))
            .get_result(conn)
    }) {
        Ok(v) => Some(v),
        Err(_e) => None,
    }
}

pub fn retrieve_paste(conn: &PgConnection, hash_string: String) -> Option<Paste> {
    let ids_some = HashIds::new_with_salt_and_min_length("the answer is 42".to_string(), 10);
    let ids = match ids_some {
        Ok(v) => v,
        Err(_e) => return None,
    };

    let mut longs = ids.decode(hash_string);
    let request_id = match longs.pop() {
        Some(v) => v as i32,
        None => return None,
    };

    let mut results = pastes
        .filter(id.eq(request_id))
        .load::<Paste>(conn)
        .expect("Error loading pastes");

    results.pop()
}
