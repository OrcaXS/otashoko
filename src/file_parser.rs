use errors::DataError;
use models::*;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct FsFolder {
    pub folder_name: Option<String>,
    pub path: PathBuf,
}

pub struct OwnedNewFile {
    pub file_id: String,
    pub folder_id: String,
    pub file_name: String,
    pub file_size: Option<i32>,
}

use super::dbqueries;
fn read_file(folder_path: &Path, folder_uuid: Uuid) -> Result<Vec<OwnedNewFile>, DataError> {
    let mut files = Vec::new();
    let folder_id = folder_uuid.to_string();
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name_str = match entry.file_name().into_string() {
            Ok(file_name_str) => file_name_str,
            Err(_) => continue,
        };
        if path.is_file() {
            let file_uuid = Uuid::new_v4();
            let size = entry.metadata()?.len() as i32;
            let file_id = file_uuid.to_string();
            let new_file = OwnedNewFile {
                file_id: file_id,
                folder_id: folder_id.clone(),
                file_name: file_name_str,
                file_size: Some(size),
            };
            files.push(new_file);
        }
    }
    Ok(files)
}

pub fn scan_root_folder() -> Result<(), DataError> {
    let db = dbqueries::Db::new();
    let path = Path::new("files");
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path_str = match path.to_str() {
            Some(path_str) => path_str,
            None => continue,
        };
        if path.is_dir() {
            let folder_uuid = Uuid::new_v4();
            let size = entry.metadata()?.len() as i32;
            let new_folder = NewFolder {
                folder_id: &folder_uuid.to_string(),
                folder_path: path_str,
                folder_size: Some(&size),
            };
            db.add_folder(new_folder, folder_uuid)?;
            let files = read_file(path.as_path(), folder_uuid)?;
            db.add_files(files, folder_uuid)?;
        }
    }
    Ok(())
}

// pub fn scan_root_for_book() -> Result<(), DataError> {
//     let db = dbqueries::Db::new();
//     let path = Path::new("files");
//     for entry in fs::read_dir(path)? {
//         let entry = entry?;
//         let path = entry.path();
//         let path_str = match path.to_str() {
//             Some(path_str) => path_str,
//             None => continue,
//         };
//         if path.is_dir() {
//             let folder_uuid = Uuid::new_v4();
//             let size = entry.metadata()?.len() as i32;
//             let new_folder = NewFolder {
//                 folder_id: &folder_uuid.to_string(),
//                 folder_path: path_str,
//                 folder_size: Some(&size),
//             };
//             db.add_folder(new_folder, folder_uuid)?;
//             let files = read_file(path.as_path(), folder_uuid)?;
//             db.add_files(files, folder_uuid)?;
//         }
//     }
//     Ok(())
// }

pub fn add_folder_from_path(folder_path: &str) -> Result<Folder, DataError> {
    let db = dbqueries::Db::new();
    let path = Path::new(folder_path);
    if path.is_dir() {
        let folder_uuid = Uuid::new_v4();
        let size = path.metadata()?.len() as i32;
        let path_str = match path.to_str() {
            Some(path_str) => path_str,
            None => return Err(DataError::Bail(String::from("Path cannot be converted to string."))),
        };
        let new_folder = NewFolder {
            folder_id: &folder_uuid.to_string(),
            folder_path: path_str,
            folder_size: Some(&size),
        };
        db.add_folder(new_folder, folder_uuid)?;
        let files = read_file(path, folder_uuid)?;
        db.add_files(files, folder_uuid)?;
    }
    Ok(db.get_folder_by_path(folder_path.to_string())?)
}

pub fn get_folders() -> Result<Vec<FsFolder>, DataError> {
    let root_path = Path::new("files");
    let folders: Vec<FsFolder> = fs::read_dir(root_path)?
                                .into_iter()
                                .filter_map(|entry| entry.ok())
                                .filter(|entry| entry.path().is_dir())
                                .filter(|entry| entry.path().to_str().is_some())
                                .map(|entry| FsFolder {
                                    folder_name: entry.path().file_stem().map(|x| x.to_str().unwrap_or("No folder name").to_owned()),
                                    path: entry.path().to_owned(),
                                })
                                .collect();
    Ok(folders)
}

// fn add_files(type_id: i32) {
//     let db = dbqueries::Db::new();
//     let path = Path::new("files\\extracted");
//     for entry in fs::read_dir(path).expect("dir cannot be read") {
//         let entry = entry.expect("entry cannot be read");
//         let entry_path = entry.path();
//         let entry_path_str = entry_path.to_str().expect("path_not_found");
//         println!("{}", entry.path().display());
//         let entry_name = entry.file_name();
//         let entry_size = entry.metadata().expect("metadata cannot be read").len() as i32;
//         match db.filepath_exists(entry_path_str.to_string()) {
//             Ok(exist) => {
//                 if !exist {
//                     let file_uuid = Uuid::new_v4();
//                     let new_file = NewFile {
//                         file_id: &file_uuid.to_string(),
//                         file_type_id: &type_id,
//                         file_path: Some(&entry_path_str),
//                         file_size: Some(&entry_size),
//                     };
//                     Ok(db.add_file(new_file, file_uuid).expect("cannot add file"))
//                 } else {
//                     Err(DataError::from("File exists".to_string()))
//                 }
//             }
//             Err(e) => Err(e),
//         };
//     }
// }
//
// fn add_file_type() -> Result<FileType, DataError> {
//     let db = dbqueries::Db::new();
//
//     let dir_file_type = NewFileType {
//         file_type_name: "directory",
//     };
//
//     Ok(db.add_file_type(dir_file_type)?)
// }
//
// // fn add_files() -> Result<FileType, DataError> {
// //
// // }
//
// pub fn add_files_to_db() {
//     let dir_str = String::from("directory");
//     let db = dbqueries::Db::new();
//     let file_type;
//     match db.get_file_type_by_name(dir_str) {
//         Ok(ft) => { file_type = ft },
//         Err(_) => match add_file_type() {
//             Ok(ft_added) => { file_type = ft_added },
//             Err(e) => { println!("{:#?}", e); return },
//         },
//     };
//     add_files(file_type.file_type_id);
//     // println!("{:#?}", file_type);
// }
