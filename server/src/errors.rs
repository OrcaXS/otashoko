use std::io;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct MigrationError {
    kind: String,    // type of the error
    message: String, // error message
}

#[derive(Debug)]
pub struct AppError {
    kind: String,
    message: String,
}

#[derive(Debug)]
pub struct DbError {
    kind: String,    // type of the error
    message: String, // error message
}

impl fmt::Display for MigrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Migration error.")
    }
}

// This is important for other errors to wrap this one.
impl error::Error for MigrationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl From<io::Error> for MigrationError {
    fn from(error: io::Error) -> Self {
        MigrationError { kind: String::from("io"), message: error.to_string() }
    }
}

impl From<r2d2::Error> for MigrationError {
    fn from(error: r2d2::Error) -> Self {
        MigrationError { kind: String::from("r2d2"), message: error.to_string() }
    }
}

impl From<rusqlite::Error> for MigrationError {
    fn from(error: rusqlite::Error) -> Self {
        MigrationError { kind: String::from("sqlite"), message: error.to_string() }
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError { kind: String::from("io"), message: error.to_string() }
    }
}

impl From<uuid::Error> for AppError {
    fn from(error: uuid::Error) -> Self {
        AppError { kind: String::from("uuid"), message: error.to_string() }
    }
}

impl From<std::time::SystemTimeError> for AppError {
    fn from(error: std::time::SystemTimeError) -> Self {
        AppError { kind: String::from("time"), message: error.to_string() }
    }
}

impl From<rusqlite::Error> for DbError {
    fn from(error: rusqlite::Error) -> Self {
        DbError { kind: String::from("sqlite"), message: error.to_string() }
    }
}

impl From<uuid::Error> for DbError {
    fn from(error: uuid::Error) -> Self {
        DbError { kind: String::from("uuid"), message: error.to_string() }
    }
}

impl From<r2d2::Error> for DbError {
    fn from(error: r2d2::Error) -> Self {
        DbError { kind: String::from("r2d2"), message: error.to_string() }
    }
}