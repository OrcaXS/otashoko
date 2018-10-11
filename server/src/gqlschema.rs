use juniper::{FieldResult, RootNode};
use models;
use models::{Book, BookType, FileType, File, Tag, BookTag};
// use chrono::prelude::*;
use uuid::Uuid;

use super::Database;

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewBook {
    name: String,
    book_type_id: i32,
    file_id: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct UpdateBook {
    book_id: String,
    name: Option<String>,
    book_type_id: Option<i32>,
    file_id: Option<String>,
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
    file_type_id: i32,
    file_path: Option<String>,
    file_size: Option<i32>,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct UpdateFile {
    file_id: String,
    file_type_id: Option<i32>,
    file_path: Option<String>,
    file_size: Option<i32>
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
    field file(&executor) -> FieldResult<File> {
        let file_uuid = Uuid::parse_str(&self.file_id)?;
        Ok(executor.context().db.get_file(file_uuid)?)
    }
    field book_meta() -> &str { self.book_meta.as_ref().map_or("", |i| i.as_str()) }
});

graphql_object!(BookType: Database |&self| {
    field book_type_id() -> i32 { self.book_type_id }
    field book_type_name() -> &str { self.book_type_name.as_str() }
});

graphql_object!(File: Database |&self| {
    field file_id() -> &str { self.file_id.as_str() }
    field file_type_id() -> i32 { self.file_type_id }
    field file_path() -> &str { self.file_path.as_ref().map_or("", |i| i.as_str()) }
    field file_size() -> i32 { self.file_size.unwrap_or(0) }
});

graphql_object!(FileType: Database |&self| {
    field file_type_id() -> i32 { self.file_type_id }
    field file_type_name() -> &str { self.file_type_name.as_str() }
});

graphql_object!(Tag: Database |&self| {
    field tag_id() -> i32 { self.tag_id }
    field tag_name() -> &str { self.tag_name.as_str() }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Database |&self| {
    field apiVersion() -> &str {
        "1.0"
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

    field fileTypes(&executor) -> FieldResult<Vec<FileType>> {
        let context = executor.context();
        Ok(context.db.get_file_types()?)
    }

    field tagList(&executor) -> FieldResult<Vec<Tag>> {
        let context = executor.context();
        Ok(context.db.get_tags()?)
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
            file_id: &new_book.file_id,
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
            file_type_id: &new_file.file_type_id,
            file_path: new_file.file_path.as_ref().map(|x| &**x),
            file_size: new_file.file_size.as_ref(),
        };
        let context = executor.context();
        Ok(context.db.add_file(add_file, file_uuid)?)
    }

    field addFileType(&executor, new_file_type: NewFileType) -> FieldResult<FileType> {
        let add_file_type = models::NewFileType {
            file_type_name: &new_file_type.file_type_name,
        };
        let context = executor.context();
        Ok(context.db.add_file_type(add_file_type)?)
    }

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

    field removeFileType(&executor, file_type_name: String) -> FieldResult<Vec<FileType>> {
        let context = executor.context();
        Ok(context.db.remove_file_type(file_type_name)?)
    }

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
            file_id: new_book.file_id.as_ref().map(|x| &**x),
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

    field updateFile(&executor, new_file: UpdateFile) -> FieldResult<File> {
        let update_file = models::UpdateFile {
            file_id: &new_file.file_id,
            file_type_id: new_file.file_type_id.as_ref(),
            file_path: new_file.file_path.as_ref().map(|x| &**x),
            file_size: new_file.file_size.as_ref(),
        };
        let context = executor.context();
        Ok(context.db.update_file(update_file)?)
    }

    field updateFileType(&executor, new_file_type: UpdateFileType) -> FieldResult<FileType> {
        let update_file_type = models::UpdateFileType {
            file_type_id: &new_file_type.file_type_id,
            file_type_name: &new_file_type.file_type_name,
        };
        let context = executor.context();
        Ok(context.db.update_file_type(update_file_type)?)
    }

    field updateTag(&executor, new_tag: UpdateTag) -> FieldResult<Tag> {
        let update_tag = models::UpdateTag {
            tag_id: &new_tag.tag_id,
            tag_name: &new_tag.tag_name,
        };
        let context = executor.context();
        Ok(context.db.update_tag(update_tag)?)
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
