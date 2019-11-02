use uuid::Uuid;
use chrono::prelude::*;

pub struct FileType {
    pub file_type_id: i32,
    pub file_type_name: String,
}

pub struct NewFileType<'a> {
    pub file_type_name: &'a str,
}

pub struct UpdateFileType<'a> {
    pub file_type_id: &'a i32,
    pub file_type_name: &'a str,
}
