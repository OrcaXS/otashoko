use super::schema::medium;
use chrono::prelude::*;
use chrono;

#[derive(Queryable)]
pub struct Media {
    pub media_id: i32,
    pub name: String,
    pub media_type_id: i32,
    pub add_date: chrono::NaiveDateTime,
    pub last_open_date: Option<chrono::NaiveDateTime>, 
    pub file_id: i32,
    pub media_meta: Option<String>,
}

// #[derive(Insertable)]
// #[table_name = "medium"]
// pub struct NewMedia<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }