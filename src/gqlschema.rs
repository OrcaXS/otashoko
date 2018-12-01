use juniper::{FieldResult, RootNode};
use models;
use models::{Book, BookTag, BookType, File, Folder, Tag};
// use chrono::prelude::*;
use file_parser::{get_folders, FsFolder, add_folder_from_path};
use uuid::Uuid;

use std::path::{Path, PathBuf};

use super::Database;

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewBook {
    name: String,
    book_type_id: i32,
    folder_id: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct UpdateBook {
    book_id: String,
    name: Option<String>,
    book_type_id: Option<i32>,
    folder_id: Option<String>,
    book_meta: Option<String>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewBookType {
    book_type_name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct UpdateBookType {
    book_type_id: i32,
    book_type_name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewFile {
    folder_id: String,
    file_name: String,
    file_size: Option<i32>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct UpdateFile {
    file_id: String,
    folder_id: Option<String>,
    file_name: Option<String>,
    file_size: Option<i32>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewFolder {
    folder_id: String,
    folder_path: String,
    folder_size: Option<i32>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct UpdateFolder {
    folder_id: String,
    folder_path: Option<String>,
    folder_size: Option<i32>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewFileType {
    file_type_name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct UpdateFileType {
    file_type_id: i32,
    file_type_name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewTag {
    tag_name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct UpdateTag {
    tag_id: i32,
    tag_name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewBookTag {
    book_id: String,
    tag_id: i32,
}

graphql_object!(Book: Database |&self| {
    field id() -> &str { self.book_id.as_str() }
    field name() -> &str { self.name.as_str() }
    field book_type_id() -> i32 { self.book_type_id }
    field book_type(&executor) -> FieldResult<BookType> {
        Ok(executor.context().db.get_book_type(self.book_type_id)?)
    }
    field add_date() -> String { self.add_date.to_string() }
    field last_open_date() -> String {
        self.last_open_date.as_ref().map_or(String::from(""), |d| d.to_string())
    }
    field folder(&executor) -> FieldResult<Folder> {
        let folder_uuid = Uuid::parse_str(&self.folder_id)?;
        Ok(executor.context().db.get_folder(folder_uuid)?)
    }
    field book_meta() -> &str { self.book_meta.as_ref().map_or("", |i| i.as_str()) }
});

graphql_object!(BookType: Database |&self| {
    field book_type_id() -> i32 { self.book_type_id }
    field book_type_name() -> &str { self.book_type_name.as_str() }
});

graphql_object!(File: Database |&self| {
    field file_id() -> &str { self.file_id.as_str() }
    field folder_id() -> &str { self.folder_id.as_str() }
    field file_name() -> &str { self.file_name.as_str() }
    field file_size() -> i32 { self.file_size.unwrap_or(0) }
});

// graphql_object!(FileType: Database |&self| {
//     field file_type_id() -> i32 { self.file_type_id }
//     field file_type_name() -> &str { self.file_type_name.as_str() }
// });

graphql_object!(Folder: Database |&self| {
    field folder_id() -> &str { self.folder_id.as_str() }
    field folder_path() -> &str { self.folder_path.as_str() }
    field folder_size() -> i32 { self.folder_size.unwrap_or(0) }
});

graphql_object!(Tag: Database |&self| {
    field tag_id() -> i32 { self.tag_id }
    field tag_name() -> &str { self.tag_name.as_str() }
});

graphql_object!(FsFolder: () |&self| {
    field folder_name() -> &str {
        match self.folder_name {
            Some(ref x) => x.as_str(),
            None => "",
        }
    }
    field folder_path() -> &str { self.folder_path.to_str().unwrap_or("") }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Database |&self| {
    field apiVersion() -> &str {
        "0.1"
    }

    field bookList(&executor) -> FieldResult<Vec<Book>> {
        let context = executor.context();
        Ok(context.db.get_books()?)
    }

    field book(&executor, book_id: String) -> FieldResult<Book> {
        let book_uuid = Uuid::parse_str(&book_id)?;
        let context = executor.context();
        Ok(context.db.get_book(book_uuid)?)
    }

    field bookTypes(&executor) -> FieldResult<Vec<BookType>> {
        let context = executor.context();
        Ok(context.db.get_book_types()?)
    }

    field file(&executor, file_id: String) -> FieldResult<File> {
        let file_uuid = Uuid::parse_str(&file_id)?;
        let context = executor.context();
        Ok(context.db.get_file(file_uuid)?)
    }

    field fileList(&executor) -> FieldResult<Vec<File>> {
        let context = executor.context();
        Ok(context.db.get_files()?)
    }

    field folder(&executor, folder_id: String) -> FieldResult<Folder> {
        let folder_uuid = Uuid::parse_str(&folder_id)?;
        let context = executor.context();
        Ok(context.db.get_folder(folder_uuid)?)
    }

    field folderByPath(&executor, folder_path: String) -> FieldResult<Folder> {
        let context = executor.context();
        Ok(context.db.get_folder_by_path(folder_path)?)
    }

    field dbFolderList(&executor) -> FieldResult<Vec<Folder>> {
        let context = executor.context();
        Ok(context.db.get_folders()?)
    }

    // field fileTypes(&executor) -> FieldResult<Vec<FileType>> {
    //     let context = executor.context();
    //     Ok(context.db.get_file_types()?)
    // }

    field tagList(&executor) -> FieldResult<Vec<Tag>> {
        let context = executor.context();
        Ok(context.db.get_tags()?)
    }

    field fsFolderList() -> FieldResult<Vec<FsFolder>> {
        Ok(get_folders()?)
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Database |&self| {
    field addBook(&executor, new_book: NewBook) -> FieldResult<Book> {
        let book_uuid = Uuid::new_v4();
        let add_book = models::NewBook {
            book_id: &book_uuid.to_string(),
            name: &new_book.name,
            add_date: &diesel::dsl::now,
            book_type_id: &new_book.book_type_id,
            folder_id: &new_book.folder_id,
        };
        let context = executor.context();
        Ok(context.db.add_book(add_book, book_uuid)?)
    }

    field addBookType(&executor, new_book_type: NewBookType) -> FieldResult<BookType> {
        let add_book_type = models::NewBookType {
            book_type_name: &new_book_type.book_type_name,
        };
        let context = executor.context();
        Ok(context.db.add_book_type(add_book_type)?)
    }

    field addFile(&executor, new_file: NewFile) -> FieldResult<File> {
        let file_uuid = Uuid::new_v4();
        let add_file = models::NewFile {
            file_id: &file_uuid.to_string(),
            folder_id: &new_file.folder_id,
            file_name: &new_file.file_name,
            file_size: new_file.file_size.as_ref(),
        };
        let context = executor.context();
        Ok(context.db.add_file(add_file, file_uuid)?)
    }

    field addFolder(&executor, new_folder: NewFolder) -> FieldResult<Folder> {
        let folder_uuid = Uuid::new_v4();
        let add_folder = models::NewFolder {
            folder_id: &folder_uuid.to_string(),
            folder_path: &new_folder.folder_path,
            folder_size: new_folder.folder_size.as_ref(),
        };
        let context = executor.context();
        Ok(context.db.add_folder(add_folder, folder_uuid)?)
    }

    // field addFileType(&executor, new_file_type: NewFileType) -> FieldResult<FileType> {
    //     let add_file_type = models::NewFileType {
    //         file_type_name: &new_file_type.file_type_name,
    //     };
    //     let context = executor.context();
    //     Ok(context.db.add_file_type(add_file_type)?)
    // }

    field addTag(&executor, new_tag: NewTag) -> FieldResult<Tag> {
        let add_tag = models::NewTag {
            tag_name: &new_tag.tag_name,
        };
        let context = executor.context();
        Ok(context.db.add_tag(add_tag)?)
    }

    field removeBook(&executor, id: String) -> FieldResult<Book> {
        let book_uuid = Uuid::parse_str(&id)?;
        let context = executor.context();
        Ok(context.db.remove_book(book_uuid)?)
    }

    field removeBookType(&executor, book_type_name: String) -> FieldResult<Vec<BookType>> {
        let context = executor.context();
        Ok(context.db.remove_book_type(book_type_name)?)
    }

    field removeFile(&executor, file_id: String) -> FieldResult<File> {
        let file_uuid = Uuid::parse_str(&file_id)?;
        let context = executor.context();
        Ok(context.db.remove_file(file_uuid)?)
    }

    field removeFolder(&executor, folder_id: String) -> FieldResult<Vec<Folder>> {
        let folder_uuid = Uuid::parse_str(&folder_id)?;
        let context = executor.context();
        Ok(context.db.remove_folder(folder_uuid)?)
    }

    // field removeFileType(&executor, file_type_name: String) -> FieldResult<Vec<FileType>> {
    //     let context = executor.context();
    //     Ok(context.db.remove_file_type(file_type_name)?)
    // }

    field removeTag(&executor, tag_name: String) -> FieldResult<Vec<Tag>> {
        let context = executor.context();
        Ok(context.db.remove_tag(tag_name)?)
    }

    field removeBookTag(&executor, book_tag: NewBookTag) -> FieldResult<Vec<Tag>> {
        let context = executor.context();
        let del_book_tag = models::NewBookTag {
            book_id: &book_tag.book_id,
            tag_id: &book_tag.tag_id,
        };
        Ok(context.db.remove_book_tag(del_book_tag)?)
    }

    field updateBook(&executor, new_book: UpdateBook) -> FieldResult<Book> {
        let update_book = models::UpdateBook {
            book_id: &new_book.book_id,
            name: new_book.name.as_ref().map(|x| &**x),
            book_type_id: new_book.book_type_id.as_ref(),
            folder_id: new_book.folder_id.as_ref().map(|x| &**x),
            book_meta: new_book.book_meta.as_ref().map(|x| &**x),
        };
        let context = executor.context();
        Ok(context.db.update_book(update_book)?)
    }

    field updateBookType(&executor, new_book_type: UpdateBookType) -> FieldResult<BookType> {
        let update_book_type = models::UpdateBookType {
            book_type_id: &new_book_type.book_type_id,
            book_type_name: &new_book_type.book_type_name,
        };
        let context = executor.context();
        Ok(context.db.update_book_type(update_book_type)?)
    }

    field updateFolder(&executor, new_folder: UpdateFolder) -> FieldResult<Folder> {
        let update_folder = models::UpdateFolder {
            folder_id: &new_folder.folder_id,
            folder_path: new_folder.folder_path.as_ref().map(|x| &**x),
            folder_size: new_folder.folder_size.as_ref(),
        };
        let context = executor.context();
        Ok(context.db.update_folder(update_folder)?)
    }

    field updateFile(&executor, new_file: UpdateFile) -> FieldResult<File> {
        let update_file = models::UpdateFile {
            file_id: &new_file.file_id,
            folder_id: new_file.folder_id.as_ref().map(|x| &**x),
            file_name: new_file.file_name.as_ref().map(|x| &**x),
            file_size: new_file.file_size.as_ref(),
        };
        let context = executor.context();
        Ok(context.db.update_file(update_file)?)
    }

    // field updateFileType(&executor, new_file_type: UpdateFileType) -> FieldResult<FileType> {
    //     let update_file_type = models::UpdateFileType {
    //         file_type_id: &new_file_type.file_type_id,
    //         file_type_name: &new_file_type.file_type_name,
    //     };
    //     let context = executor.context();
    //     Ok(context.db.update_file_type(update_file_type)?)
    // }

    field updateTag(&executor, new_tag: UpdateTag) -> FieldResult<Tag> {
        let update_tag = models::UpdateTag {
            tag_id: &new_tag.tag_id,
            tag_name: &new_tag.tag_name,
        };
        let context = executor.context();
        Ok(context.db.update_tag(update_tag)?)
    }

    field parseFolderPath(&executor, folder_path: String) -> FieldResult<Folder> {
        // add_folder_from_path(&folder_path)?;
        // let context = executor.context();
        // Ok(context.db.get_folder_by_path(folder_path)?)
        Ok(add_folder_from_path(&folder_path)?)
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
