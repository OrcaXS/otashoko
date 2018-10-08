use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::types::{Nullable, Text};
use juniper::{FieldResult, RootNode};
use models::{Book, NewBook as mNewBook, BookType, NewBookType as mNewBookType};
use chrono::prelude::*;
use uuid::Uuid;

use super::Database;

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewBook {
    name: String,
    book_type_id: i32,
    file_id: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewBookType {
    book_type_name: String,
}

// type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

graphql_object!(Book: Database |&self| {
    field id() -> &str { self.book_id.as_str() }
    field name() -> &str { self.name.as_str() }
    field book_type_id() -> i32 { self.book_type_id }
    field add_date() -> String { self.add_date.to_string() }
    field last_open_date() -> String {
        self.last_open_date.as_ref().map_or(String::from(""), |d| d.to_string())
    }
    field file_id() -> &str { self.file_id.as_str() }
    field book_meta() -> &str { self.book_meta.as_ref().map_or("", |i| i.as_str()) }
});

graphql_object!(BookType: Database |&self| {
    field book_type_id() -> i32 { self.book_type_id }
    field book_type_name() -> &str { self.book_type_name.as_str() }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Database |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field bookList(&executor) -> FieldResult<Vec<Book>> {
        let context = executor.context();
        Ok(context.db.get_books()?)
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Database |&self| {
    field addBook(&executor, new_book: NewBook) -> FieldResult<Book> {
        let book_uuid = Uuid::new_v4();
        let new_book_db = mNewBook {
            book_id: &book_uuid.to_string(),
            name: &new_book.name,
            add_date: &diesel::dsl::now,
            book_type_id: &new_book.book_type_id,
            file_id: &new_book.file_id,
        };
        let context = executor.context();
        Ok(context.db.add_book(new_book_db, book_uuid)?)
    }

    field addBookType(&executor, new_book_type: NewBookType) -> FieldResult<BookType> {
        let new_book_type_db = mNewBookType {
            book_type_name: &new_book_type.book_type_name,
        };
        let context = executor.context();
        Ok(context.db.add_book_type(new_book_type_db)?)
    }
});

// field addBookType(&executor, book_type: NewBookType) -> FieldResult<Book> {
//     use ::schema::books;
//     // use ::schema::books::dsl::*;
//
//     let book_uuid = uuid::Uuid::new_v4();
//     let new_book_d = dNewBook {
//         book_id: &book_uuid.to_string(),
//         name: &new_book.name,
//         add_date: &diesel::dsl::now,
//         book_type_id: &new_book.book_type_id,
//         file_id: &new_book.file_id,
//     };
//     let context = executor.context();
//     let conn = context.connection.get()?;
//     diesel::insert_into(books::table)
//         .values(&new_book_d)
//         .execute(&conn)?;
//
//     let result = books::table.find(book_uuid.to_string()).first::<dBook>(&conn)?;
//     Ok(Book{
//         id: result.book_id,
//         name: result.name,
//         add_date: result.add_date,
//         book_type_id: result.book_type_id,
//         file_id: result.file_id,
//     })


pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
