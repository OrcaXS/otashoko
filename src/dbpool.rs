// use rusqlite::Connection;

// use r2d2_sqlite::SqliteConnectionManager;
// use r2d2;

use diesel::prelude::*;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

use std::io;
use std::path::PathBuf;

// use errors::DataError;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

lazy_static! {
    static ref POOL: Pool = init_pool("db/otashoko.db");
}

/// Get an r2d2 `SqliteConnection`.
pub fn connection() -> Pool {
    POOL.clone()
}

fn init_pool(db_path: &str) -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(db_path);
    let pool = r2d2::Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool.");

    {
        let _db = pool.get().expect("Failed to initialize pool.");
        // run_migration_on(&*db).expect("Failed to run migrations during init.");
    }
    // info!("Database pool initialized.");
    pool
}
