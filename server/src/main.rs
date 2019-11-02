#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate lazy_static;
extern crate tide;
#[macro_use]
extern crate log;

use std::env;
use log::Level;
use http::status::StatusCode;


use tide::{error::ResultExt, response, App, Context, EndpointResult};
// use r2d2_sqlite::SqliteConnectionManager;

mod migrations;

// mod db;
// use db::{CreateUser, DbExecutor};

// mod schema;
// mod gqlschema;

// use gqlschema::create_schema;
// use gqlschema::Schema;

mod dbpool;
// mod dbqueries;

// use dbqueries::Db;

// mod models;

mod errors;
mod models;
mod utils;
// use models::book;

// #[derive(Serialize, Deserialize)]
// pub struct GraphQLData(GraphQLRequest);

// pub struct GraphQLExecutor {
//     gqlschema: std::sync::Arc<Schema>,
// }

// impl GraphQLExecutor {
//     fn new(gqlschema: std::sync::Arc<Schema>) -> GraphQLExecutor {
//         GraphQLExecutor { gqlschema: gqlschema }
//     }
// }

// pub struct Database {
//     pub db: Arc<Db>,
// }

// impl juniper::Context for Database {}

// async fn handle_graphql(mut cx: Context<Database>) -> EndpointResult {
//     let query: juniper::http::GraphQLRequest = cx.body_json().await.client_err()?;
//     let schema = create_schema();
//     let response = query.execute(&schema, cx.state());
//     let status = if response.is_ok() {
//         StatusCode::OK
//     } else {
//         StatusCode::BAD_REQUEST
//     };
//     let mut resp = response::json(response);
//     *resp.status_mut() = status;
//     Ok(resp)
// }

fn start_app() -> Result<(), std::io::Error> {
    // let mut app = App::new(Database::default());
    let app = App::new();
    // app.at("/graphql").post(handle_graphql);
    info!("Starting http server: 127.0.0.1:3080");
    app.run("127.0.0.1:3080")
}

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args.len());
    match env::args().nth(1).as_ref().map(|s| &s[..]) {
        Some("run_migrations") => migrations::migrate_to_pool().unwrap(),
        _ => start_app().unwrap(),
    }
}
