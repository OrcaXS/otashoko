use uuid::Uuid;
use chrono::prelude::*;

pub struct File {
    pub file_id: String,
    pub file_type_id: i32,
    pub file_path: Option<String>,
    pub file_size: Option<i32>,
}

pub struct NewFile<'a> {
    pub file_id: &'a str,
    pub file_type_id: &'a i32,
    pub file_path: Option<&'a str>,
    pub file_size: Option<&'a i32>,
}

pub struct UpdateFile<'a> {
    pub file_id: &'a str,
    pub file_type_id: Option<&'a i32>,
    pub file_path: Option<&'a str>,
    pub file_size: Option<&'a i32>,
}