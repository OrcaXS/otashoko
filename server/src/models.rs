use super::schema::*;
use chrono::prelude::*;
use chrono;
use diesel::dsl;

#[derive(Queryable, Identifiable, PartialEq, Associations)]
#[belongs_to(BookType, foreign_key = "book_type_id")]
#[belongs_to(File, foreign_key="file_id")]
#[primary_key(book_id)]
#[table_name = "books"]
pub struct Book {
    pub book_id: String,
    pub name: String,
    pub book_type_id: i32,
    // pub book_type_name: String,
    pub add_date: chrono::NaiveDateTime,
    pub last_open_date: Option<chrono::NaiveDateTime>,
    pub file_id: String,
    pub book_meta: Option<String>,
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook<'a> {
    pub book_id: &'a str,
    pub name: &'a str,
    pub book_type_id: &'a i32,
    pub add_date: &'a dsl::now,
    pub file_id: &'a str,
}

#[derive(Queryable, Identifiable, PartialEq, Associations)]
#[primary_key(book_type_id)]
#[table_name = "book_types"]
pub struct BookType {
    pub book_type_id: i32,
    pub book_type_name: String,
}

#[derive(Insertable)]
#[table_name = "book_types"]
pub struct NewBookType<'a> {
    pub book_type_name: &'a str,
}

#[derive(Queryable, Identifiable, Associations)]
#[primary_key(file_id)]
#[table_name = "files"]
pub struct File {
    pub file_id: String,
    pub file_type_id: i32,
    pub file_path: Option<String>,
    pub file_size: Option<i32>,
}

#[derive(Insertable)]
#[table_name="files"]
pub struct NewFile<'a> {
    pub file_id: &'a str,
    pub file_type_id: &'a i32,
    pub file_path: Option<&'a str>,
    pub file_size: Option<&'a i32>,
}

#[derive(Queryable, Identifiable, Associations)]
#[primary_key(file_type_id)]
pub struct FileType {
    pub file_type_id: i32,
    pub file_type_name: String,
}

#[derive(Insertable)]
#[table_name="file_types"]
pub struct NewFileType<'a> {
    pub file_type_name: &'a str,
}
