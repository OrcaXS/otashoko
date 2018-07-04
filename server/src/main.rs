#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_cors;
extern crate juniper;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
// extern crate juniper_rocket;

mod preview_zip;

use std::io;
use std::path::{Path, PathBuf};

use rocket::http::Method;
use rocket::response::NamedFile;
use rocket_cors::{AllowedOrigins, AllowedHeaders};
use rocket_contrib::{Json, Value};


type ID = usize;

#[derive(Serialize, Deserialize)]
struct Message {
    id: Option<ID>,
    contents: String
}


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("files/index.html")
}

#[get("/files/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("files/").join(file)).ok()
}

#[get("/test")]
fn test() -> Json<Value> {
    let file_path = Path::new("files/archive.zip");
    let extract_path = Path::new("files/extracted");
    // println!("{:#?}", preview_zip::read_zip(file_path));
    let j = json!(&preview_zip::read_zip(file_path, extract_path));
    Json(j)
}


fn rocket() -> rocket::Rocket {
    let (_allowed_origins, failed_origins) = AllowedOrigins::some(&["http://localhost"]);
    assert!(failed_origins.is_empty());

    let options = rocket_cors::Cors {
        // allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };


    rocket::ignite().mount("/", routes![test, index, files]).attach(options)
}

fn main() {
    rocket().launch();
}
