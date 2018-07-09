extern crate diesel;
extern crate otashoko_server;

use self::diesel::prelude::*;
use self::otashoko_server::*;
use std::env::args;

fn main() {
    use otashoko_server::schema::books::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(books.filter(name.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}