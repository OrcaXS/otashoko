use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::types::{Nullable, Text};
use juniper::{Context as JuniperContext, FieldResult, RootNode};
use models::{Book as dBook, NewBook as dNewBook};
use chrono::prelude::*;

use super::Database;
use database;

#[derive(GraphQLObject)]
#[graphql(description = "Query of Book object")]
struct Book {
    id: String,
    name: String,
    book_type_id: i32,
    add_date: NaiveDateTime,
    file_id: String,
}

// #[derive(GraphQLEnum)]
// enum BookType {
//     Manga,
//     Novel,
//     Others,
// }

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewBook {
    id: String,
    name: String,
    book_type_id: i32,
    file_id: String,
}

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub struct QueryRoot;

// graphql_object!(QueryRoot: Context |&self| {
//     // Arguments to resolvers can either be simple types or input objects.
//     // The executor is a special (optional) argument that allows accessing the context.
//     field human(&executor, id: String) -> FieldResult<Human> {
//         // Get the context from the executor.
//         let context = executor.context();
//         // Get a db connection.
//         let connection = context.pool.get_connection()?;
//         // Execute a db query.
//         // Note the use of `?` to propagate errors.
//         let human = connection.find_human(&id)?;
//         // Return the result.
//         Ok(human)
//     }
// });

graphql_object!(QueryRoot: Database |&self| {
    field apiVersion() -> &str {
        "1.0"
    }

    field bookList(&executor) -> FieldResult<Vec<Book>> {
        use ::schema::books::dsl::*;
        let context = executor.context();
        let conn = context.connection.get()?;
        let book_list = books.load::<dBook>(&conn)?;
        let mut v: Vec<Book> = Vec::new();
        for book_in_list in book_list.into_iter() {
            v.push(Book{
                id: book_in_list.book_id,
                name: book_in_list.name,
                book_type_id: book_in_list.book_type_id,
                add_date: book_in_list.add_date,
                file_id: book_in_list.file_id,
            });
        }
        Ok(v)
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Database |&self| {
    // field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
    //     Ok(Human{
    //         id: "1234".to_owned(),
    //         name: new_human.name,
    //         appears_in: new_human.appears_in,
    //         home_planet: new_human.home_planet,
    //     })
    // }

    field addBook(&executor, new_book: NewBook) -> FieldResult<Book> {
        use ::schema::books;
        // use ::schema::books::dsl::*;

        let new_book_d = dNewBook {
            book_id: &new_book.id,
            name: &new_book.name,
            add_date: &diesel::dsl::now,
            book_type_id: &new_book.book_type_id,
            file_id: &new_book.file_id,
        };
        let context = executor.context();
        let conn = context.connection.get()?;
        diesel::insert_into(books::table)
            .values(&new_book_d)
            .execute(&conn).expect("insert failed");

        let result = books::table.find(&new_book.id).first::<dBook>(&conn)?;
        Ok(Book{
            id: result.book_id,
            name: result.name,
            add_date: result.add_date,
            book_type_id: result.book_type_id,
            file_id: result.file_id,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
