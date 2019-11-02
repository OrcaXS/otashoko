use std::fs::{self, DirEntry, read_to_string};
use std::path::Path;
use rusqlite::{Connection, NO_PARAMS};

use crate::dbpool;
use crate::errors;

const MIGRATIONS_PATH: &str = "src/migrations/sql/";

fn execute_migration(conn: &Connection, entry: &Path) -> Result<(), errors::MigrationError> {
    let sql_stmt = read_to_string(entry)?;
    println!("{}", sql_stmt);
    let result = conn.execute_batch(&sql_stmt)?;
    Ok(result)
}

pub fn run_migration(conn: &Connection) -> Result<(), errors::MigrationError>  {
    print!("running migration");
    let path = std::env::current_dir()?.join(Path::new(MIGRATIONS_PATH));
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            match path.extension() {
                None => (),
                Some(os_str) => match os_str.to_str() {
                    Some("sql") => execute_migration(&conn, &path)?,
                    _ => (),
                }
            }
        }
    }
    Ok(())
}

pub fn migrate_to_pool() -> Result<(), errors::MigrationError> {
    let conn = dbpool::connection().get()?;
    run_migration(&conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_migration() {
        let conn = Connection::open_in_memory().unwrap();
        run_migration(&conn).unwrap();
        conn.execute("INSERT INTO schema_version (version) VALUES (?)", &[1i32]).unwrap();
        let result: i32 = conn.query_row("SELECT version from schema_version", NO_PARAMS, |row| row.get(0)).unwrap();
        assert_eq!(result, 1);
        // execute_migration(path).unwrap;
    }
}