use super::schema::books;
use chrono::prelude::*;
use chrono;
use diesel::dsl;

#[derive(Queryable)]
pub struct Book {
    pub book_id: String,
    pub name: String,
    pub book_type_id: i32,
    pub add_date: chrono::NaiveDateTime,
    pub last_open_date: Option<chrono::NaiveDateTime>,
    pub file_id: String,
    pub book_meta: Option<String>,
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook<'a> {
    pub name: &'a str,
    pub book_type_id: &'a i32,
    pub add_date: &'a dsl::now,
    pub file_id: &'a str,
}