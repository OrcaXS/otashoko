use chrono::prelude::*;
use diesel::prelude::*;

use diesel::r2d2;
use diesel::dsl::exists;
use diesel::query_builder::AsQuery;
use diesel::select;

use errors::DataError;
use database::connection;
use models::*;

pub fn get_books() -> Result<Vec<Book>, DataError> {
    use schema::books::dsl::*;
    let db = connection();
    let con = db.get()?;

    let results = books
        .limit(5)
        .load::<Book>(&con)?;
    Ok(results)
}