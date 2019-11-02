use uuid::Uuid;
use chrono::prelude::*;
use crate::errors::DbError;
use crate::utils;
use crate::dbpool;

pub struct Book {
    book_id: Uuid,
    name: String,
    book_type_id: i32,
    add_date: chrono::NaiveDateTime,
    last_open_date: Option<chrono::NaiveDateTime>,
    file_id: Uuid,
    book_meta: Option<serde_json::Value>,
}

impl Book {
    pub fn new(new_book: NewBook) -> Result<Book, DbError> {
        let book_uuid = utils::gen_uuid_v1()?;

        Ok(Book {
            book_id: book_uuid,
            name: new_book.name,
            book_type_id: new_book.book_type_id,
            add_date: new_book.add_date,
            last_open_date: None,
            file_id: new_book.file_id,
            book_meta: None,
        })
    }

    // pub fn get_book(&self, id: Uuid) -> Result<Book, DbError> {
    //     let db = &self.pool;
    //     let conn = db.get()?;
    //     let result = schema::books::table
    //         .find(id.to_string())
    //         .first::<Book>(&conn)?;
    //     Ok(result)
    // }
}

pub struct NewBook {
    pub book_id: String,
    pub name: String,
    pub book_type_id: i32,
    pub add_date: chrono::NaiveDateTime,
    pub file_id: Uuid,
}

pub struct UpdateBook {
    pub book_id: String,
    pub name: Option<String>,
    pub book_type_id: Option<i32>,
    pub file_id: Option<String>,
    pub book_meta: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_book() {
        let result = Book::new();
        assert_eq!(result, 1);
        // execute_migration(path).unwrap;
    }
}