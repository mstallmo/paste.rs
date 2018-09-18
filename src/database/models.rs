use super::schema::pastes;

#[derive(Debug, Queryable)]
pub struct Paste {
    pub id: i32,
    pub hash: String,
    pub paste: String,
}

#[derive(Insertable)]
#[table_name="pastes"]
pub struct NewPaste<'a> {
    pub paste: &'a str,
}

