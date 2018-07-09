extern crate diesel;
extern crate otashoko_server;

use self::diesel::prelude::*;
use self::otashoko_server::*;
use std::env::args;

fn main() {
    use otashoko_server::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&connection)
        .expect(&format!("Unable to find post {}", id));

    let post: models::Post = posts
        .find(id)
        .first(&connection)
        .expect(&format!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}