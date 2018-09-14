use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use juniper::{Context as JuniperContext, FieldResult, RootNode};
use models::{Book as dBook, NewBook as dNewBook};

use super::Database;
use database;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

#[derive(GraphQLObject)]
#[graphql(description = "Query of Book object")]
struct Book {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
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

    // field bookList(&executor) -> FieldResult<Book> {
    //     use ::schema::books::dsl::*;
    //     let context = executor.context();
    //     let conn = context.connect();
    //     let bookList = books.load::<dBook>(&conn)?;
    //     Ok(bookList)
    // }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Database |&self| {
    field createHuman(&executor, new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human{
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
