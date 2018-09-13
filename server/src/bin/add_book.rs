extern crate diesel;
extern crate otashoko_server;

use self::diesel::prelude::*;
use self::otashoko_server::models::*;
use self::otashoko_server::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name = &name[..(name.len() - 1)]; // Drop the newline character
    
    println!("Book_type_id?");
    let mut book_type_id = String::new();
    stdin().read_line(&mut book_type_id).unwrap();
    let book_type_id = &book_type_id[..(book_type_id.len() - 1)].parse::<i32>().unwrap();
    
    println!("File_id?");
    let mut file_id = String::new();
    stdin().read_line(&mut file_id).unwrap();
    let file_id = &file_id[..(file_id.len() - 1)].parse::<i32>().unwrap();

    let _ = add_book(&connection, name, &book_type_id, &file_id);
    println!("\nSaved NewBook {}", name);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";