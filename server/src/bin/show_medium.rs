extern crate diesel;
extern crate otashoko_server;

use self::diesel::prelude::*;
use self::otashoko_server::models::*;
use self::otashoko_server::*;

fn main() {
    use otashoko_server::schema::medium::dsl::*;

    let connection = establish_connection();
    let results = medium
        // .filter(published.eq(true))
        .limit(5)
        .load::<Media>(&connection)
        .expect("Error loading medium");

    println!("Displaying {} medium", results.len());
    for media in results {
        println!("{}", media.name);
        println!("----------\n");
        println!("{}", media.media_type_id);
    }
}