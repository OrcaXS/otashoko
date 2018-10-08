use chrono::prelude::*;
use diesel::prelude::*;

use diesel::r2d2;
use diesel::dsl::exists;
use diesel::query_builder::AsQuery;
use diesel::select;

use errors::DataError;
use dbpool::connection;
use models::*;
use schema;
use uuid::Uuid;

type Pool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

fn validate_rows(rows: usize, affected: usize) -> Result<usize, DataError> {
    if rows == affected { return Ok(rows); }
    Err(DataError::Bail(String::from("affected rows mismatched")))
}

// fn flatten<T, U>(intervals: &[(T, U)]) -> Vec<V> {
//     use std::iter::once;
//
//     intervals.iter()
//         .flat_map(|tup| once(tup.0).chain(once(tup.1)))
//         .collect()
// }

pub struct Db {
    pool: Pool,
}

impl Db {
    pub fn new() -> Db {
        let pool = connection();
        Db { pool }
    }

    pub fn get_book(&self, id: Uuid) -> Result<Book, DataError> {
        let db = &self.pool;
        let conn = db.get()?;
        let result = schema::books::table.find(id.to_string()).first::<Book>(&conn)?;
        Ok(result)
    }

    pub fn get_books(&self) -> Result<Vec<Book>, DataError> {
        use schema::books::dsl::*;
        use schema::book_types::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let results: Vec<Book> = books
            .load(&conn)?;
        Ok(results)
    }

    pub fn get_book_types(&self) -> Result<Vec<BookType>, DataError> {
        use schema::book_types::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let results = book_types
            .load::<BookType>(&conn)?;
        Ok(results)

    }

    pub fn add_book(&self, new_book: NewBook, book_uuid: Uuid) -> Result<Book, DataError> {
        // use schema::books::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::insert_into(schema::books::table)
        .values(&new_book)
        .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::books::table.find(book_uuid.to_string()).first::<Book>(&conn)?;
        Ok(result)
    }

    pub fn add_book_type(&self, new_book_type: NewBookType) -> Result<BookType, DataError> {
        use schema::book_types::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::insert_into(schema::book_types::table)
        .values(&new_book_type)
        .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::book_types::table.order(book_type_id.desc()).first::<BookType>(&conn)?;
        Ok(result)
    }

}
