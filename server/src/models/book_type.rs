use uuid::Uuid;
use chrono::prelude::*;
use rusqlite::{params};
use crate::errors::DbError;
use crate::dbpool;

pub struct BookType {
    pub book_type_id: i32,
    pub book_type_name: String,
}

impl BookType {
    pub fn new (name: String) -> Result<BookType, DbError> {
        let conn = &dbpool::connection().get()?;
        conn.execute("INSERT INTO book_types (book_type_name) VALUES (?)", params![&name])?;
        conn.query_row_and_then(
            "SELECT (book_type_id, book_type_name) FROM book_types WHERE book_type_name='?'",
            &[&name],
            |row| Ok(BookType {
                book_type_id: row.get(0)?,
                book_type_name: row.get(1)?,
            }),
        )
    }
}

pub struct UpdateBookType<'a> {
    pub book_type_id: &'a i32,
    pub book_type_name: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_book() {
        let result = BookType::new("test_type".to_string()).unwrap();
        assert_eq!(result.book_type_id, 1);
        assert_eq!(result.book_type_name, "test_type");
        // execute_migration(path).unwrap;
    }
}