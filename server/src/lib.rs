#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

pub mod schema;
pub mod models;

use self::models::{Book, NewBook};
use diesel::prelude::*;
use diesel::dsl;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_book(
    conn: &SqliteConnection, 
    name: &str,
    book_type_id: &i32,
    file_id: &i32,
) -> usize {
    use schema::books;
    
    let new_book = NewBook {
        name: name,
        book_type_id: book_type_id,
        add_date: &dsl::now,
        file_id: file_id,
    };

    diesel::insert_into(books::table)
        .values(&new_book)
        .execute(conn)
        .expect("Error saving new book")
}