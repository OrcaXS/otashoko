//! Actix web juniper example
//!
//! A simple example integrating juniper in actix-web
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate juniper;
// extern crate r2d2;
// extern crate r2d2_sqlite;
// extern crate rusqlite;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate uuid;
#[macro_use]
extern crate lazy_static;
extern crate chrono;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
// #[macro_use]
// extern crate failure_derive;
use actix::prelude::*;
use actix_web::{
    http, middleware, middleware::cors::Cors, server, App, AsyncResponder, Error, FutureResponse, HttpRequest,
    HttpResponse, Json, State,
};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;
// use r2d2_sqlite::SqliteConnectionManager;

// use diesel::sqlite::SqliteConnection;
// use diesel::r2d2;

// mod db;
// use db::{CreateUser, DbExecutor};

mod gqlschema;
mod schema;

use gqlschema::create_schema;
use gqlschema::Schema;

mod dbpool;
mod dbqueries;

use dbqueries::Db;

mod models;

mod errors;

mod file_parser;

struct AppState {
    executor: Addr<GraphQLExecutor>,
    // db: Addr<DbExecutor>,
}

#[derive(Serialize, Deserialize)]
pub struct GraphQLData(GraphQLRequest);

impl Message for GraphQLData {
    type Result = Result<String, Error>;
}

pub struct GraphQLExecutor {
    gqlschema: std::sync::Arc<Schema>,
}

impl GraphQLExecutor {
    fn new(gqlschema: std::sync::Arc<Schema>) -> GraphQLExecutor {
        GraphQLExecutor {
            gqlschema: gqlschema,
        }
    }
}

impl Actor for GraphQLExecutor {
    type Context = SyncContext<Self>;
}

pub struct Database {
    // pub connection: Pool,
    pub db: Arc<Db>,
}

impl juniper::Context for Database {}

impl Handler<GraphQLData> for GraphQLExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: GraphQLData, _: &mut Self::Context) -> Self::Result {
        let ctx = Database {
            db: Arc::new(Db::new()),
        };
        let res = msg.0.execute(&self.gqlschema, &ctx);
        let res_text = serde_json::to_string(&res)?;
        Ok(res_text)
    }
}

fn graphiql(_req: &HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    let html = graphiql_source("http://127.0.0.1:3080/graphql");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}

fn graphql((st, data): (State<AppState>, Json<GraphQLData>)) -> FutureResponse<HttpResponse> {
    st.executor
        .send(data.0)
        .from_err()
        .and_then(|res| match res {
            Ok(user) => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(user)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}

// fn context_factory(_: &mut Request) -> gqlschema::Context {
//     gqlschema::Context {
//         connection: database::connection(),
//     }
// }

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("juniper-example");

    let schema = std::sync::Arc::new(create_schema());
    let addr = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));
    // let db_addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    // Start http server
    server::new(move || {
        App::with_state(AppState {
            executor: addr.clone(),
        })
        // enable logger
        .middleware(middleware::Logger::default())
        .configure(|app| {
                Cors::for_app(app)
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .resource("/graphql", |r| r.method(http::Method::POST).with(graphql))
                    .resource("/graphiql", |r| r.method(http::Method::GET).h(graphiql))
                    .register()
        })
    })
    .bind("localhost:3080")
    .unwrap()
    .start();

    println!("Started http server: 127.0.0.1:3080");
    // file_parser::print_files();
    let _ = sys.run();
}
