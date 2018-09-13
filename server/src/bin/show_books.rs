extern crate diesel;
extern crate otashoko_server;

use self::diesel::prelude::*;
use self::otashoko_server::models::*;
use self::otashoko_server::*;

fn main() {
    use otashoko_server::schema::books::dsl::*;

    let connection = establish_connection();
    let results = books
        // .filter(published.eq(true))
        .limit(5)
        .load::<Book>(&connection)
        .expect("Error loading books");

    println!("Displaying {} books", results.len());
    for book in results {
        println!("{}", book.name);
        println!("----------\n");
        println!("{}", book.book_type_id);
        println!("{}", book.add_date);
    }
}
