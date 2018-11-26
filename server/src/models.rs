use super::schema::*;
use chrono;
use chrono::prelude::*;
use diesel::dsl;

#[derive(Queryable, Identifiable, PartialEq, Associations)]
#[belongs_to(BookType, foreign_key = "book_type_id")]
#[belongs_to(Folder, foreign_key = "folder_id")]
#[primary_key(book_id)]
pub struct Book {
    pub book_id: String,
    pub name: String,
    pub book_type_id: i32,
    pub add_date: chrono::NaiveDateTime,
    pub last_open_date: Option<chrono::NaiveDateTime>,
    pub folder_id: String,
    pub book_meta: Option<String>,
}

#[derive(Insertable)]
#[table_name = "books"]
pub struct NewBook<'a> {
    pub book_id: &'a str,
    pub name: &'a str,
    pub book_type_id: &'a i32,
    pub add_date: &'a dsl::now,
    pub folder_id: &'a str,
}

#[derive(AsChangeset, Identifiable)]
#[primary_key(book_id)]
#[table_name = "books"]
pub struct UpdateBook<'a> {
    pub book_id: &'a str,
    pub name: Option<&'a str>,
    pub book_type_id: Option<&'a i32>,
    pub folder_id: Option<&'a str>,
    pub book_meta: Option<&'a str>,
}

#[derive(Queryable, Identifiable, PartialEq, Associations)]
#[primary_key(book_type_id)]
pub struct BookType {
    pub book_type_id: i32,
    pub book_type_name: String,
}

#[derive(Insertable)]
#[table_name = "book_types"]
pub struct NewBookType<'a> {
    pub book_type_name: &'a str,
}

#[derive(AsChangeset, Identifiable)]
#[primary_key(book_type_id)]
#[table_name = "book_types"]
pub struct UpdateBookType<'a> {
    pub book_type_id: &'a i32,
    pub book_type_name: &'a str,
}

#[derive(Queryable, Identifiable, Associations, AsChangeset)]
#[primary_key(file_id)]
pub struct File {
    pub file_id: String,
    pub folder_id: String,
    pub file_name: String,
    pub file_size: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile<'a> {
    pub file_id: &'a str,
    pub folder_id: &'a str,
    pub file_name: &'a str,
    pub file_size: Option<&'a i32>,
}

#[derive(AsChangeset, Identifiable)]
#[primary_key(file_id)]
#[table_name = "files"]
pub struct UpdateFile<'a> {
    pub file_id: &'a str,
    pub folder_id: Option<&'a str>,
    pub file_name: Option<&'a str>,
    pub file_size: Option<&'a i32>,
}

#[derive(Queryable, Identifiable, Associations, AsChangeset)]
#[primary_key(folder_id)]
pub struct Folder {
    pub folder_id: String,
    pub folder_path: String,
    pub folder_size: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "folders"]
pub struct NewFolder<'a> {
    pub folder_id: &'a str,
    pub folder_path: &'a str,
    pub folder_size: Option<&'a i32>,
}

#[derive(AsChangeset, Identifiable)]
#[primary_key(folder_id)]
#[table_name = "folders"]
pub struct UpdateFolder<'a> {
    pub folder_id: &'a str,
    pub folder_path: Option<&'a str>,
    pub folder_size: Option<&'a i32>,
}

// #[derive(Debug, Queryable, Identifiable, Associations)]
// #[primary_key(file_type_id)]
// pub struct FileType {
//     pub file_type_id: i32,
//     pub file_type_name: String,
// }
//
// #[derive(Insertable)]
// #[table_name="file_types"]
// pub struct NewFileType<'a> {
//     pub file_type_name: &'a str,
// }
//
// #[derive(AsChangeset, Identifiable)]
// #[primary_key(file_type_id)]
// #[table_name="file_types"]
// pub struct UpdateFileType<'a> {
//     pub file_type_id: &'a i32,
//     pub file_type_name: &'a str,
// }

#[derive(Queryable, Identifiable, Associations)]
#[primary_key(tag_id)]
pub struct Tag {
    pub tag_id: i32,
    pub tag_name: String,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag<'a> {
    pub tag_name: &'a str,
}

#[derive(AsChangeset, Identifiable)]
#[primary_key(tag_id)]
#[table_name = "tags"]
pub struct UpdateTag<'a> {
    pub tag_id: &'a i32,
    pub tag_name: &'a str,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Book, foreign_key = "book_id")]
#[belongs_to(Tag, foreign_key = "tag_id")]
pub struct BookTag {
    pub id: i32,
    pub book_id: String,
    pub tag_id: i32,
}

#[derive(Insertable)]
#[table_name = "book_tags"]
pub struct NewBookTag<'a> {
    pub book_id: &'a str,
    pub tag_id: &'a i32,
}
