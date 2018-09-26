#![allow(proc_macro_derive_resolution_fallback)]

use super::schema::pastes;

#[derive(Debug, Queryable, Identifiable)]
pub struct Paste {
    pub id: i32,
    pub hash: Option<String>,
    pub paste: String,
}

#[derive(Insertable, Debug)]
#[table_name = "pastes"]
pub struct NewPaste<'a> {
    pub paste: &'a str,
}
