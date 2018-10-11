// use chrono::prelude::*;
use diesel::prelude::*;

use diesel::r2d2;
// use diesel::dsl::exists;
// use diesel::query_builder::AsQuery;
// use diesel::select;

use errors::DataError;
use dbpool::connection;
use models::*;
use schema;
use uuid::Uuid;

type Pool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

fn validate_rows(rows: usize, affected: usize) -> Result<usize, DataError> {
    if rows == affected { return Ok(rows); }
    Err(DataError::Bail(String::from("affected rows mismatched")))
}

pub struct Db {
    pool: Pool,
}

impl Db {
    pub fn new() -> Db {
        let pool = connection();
        Db { pool }
    }

    pub fn get_book(&self, id: Uuid) -> Result<Book, DataError> {
        let db = &self.pool;
        let conn = db.get()?;
        let result = schema::books::table
            .find(id.to_string())
            .first::<Book>(&conn)?;
        Ok(result)
    }

    pub fn get_books(&self) -> Result<Vec<Book>, DataError> {
        use schema::books::dsl::*;
        // use schema::book_types::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let results: Vec<Book> = books
            .order(add_date.desc())
            .load(&conn)?;
        Ok(results)
    }

    pub fn get_book_type(&self, id: i32) -> Result<BookType, DataError> {
        let db = &self.pool;
        let conn = db.get()?;
        let result = schema::book_types::table
            .find(id)
            .first::<BookType>(&conn)?;
        Ok(result)
    }

    pub fn get_book_types(&self) -> Result<Vec<BookType>, DataError> {
        use schema::book_types::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let results = book_types
            .load::<BookType>(&conn)?;
        Ok(results)
    }

    pub fn get_file(&self, id: Uuid) -> Result<File, DataError> {
        let db = &self.pool;
        let conn = db.get()?;
        let result = schema::files::table
            .find(id.to_string())
            .first::<File>(&conn)?;
        Ok(result)
    }

    pub fn get_files(&self) -> Result<Vec<File>, DataError> {
        let db = &self.pool;
        let conn = db.get()?;
        let result = schema::files::table
            .load::<File>(&conn)?;
        Ok(result)
    }

    pub fn get_file_type(&self, id: i32) -> Result<FileType, DataError> {
        let db = &self.pool;
        let conn = db.get()?;
        let result = schema::file_types::table
            .find(id)
            .first::<FileType>(&conn)?;
        Ok(result)
    }

    pub fn get_file_types(&self) -> Result<Vec<FileType>, DataError> {
        use schema::file_types::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let results = file_types
            .load::<FileType>(&conn)?;
        Ok(results)
    }

    pub fn get_tags(&self) -> Result<Vec<Tag>, DataError> {
        use schema::tags::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let results = tags
            .load::<Tag>(&conn)?;
        Ok(results)
    }

    pub fn get_tags_of_book(&self, book_uuid: Uuid) -> Result<Vec<Tag>, DataError> {
        use schema::book_tags::dsl::*;
        use schema::tags::dsl::*;

        let db = &self.pool;
        let conn = db.get()?;

        let book_tag_ids: Vec<i32> = book_tags
            .filter(book_id.eq(book_uuid.to_string()))
            .select(schema::book_tags::columns::tag_id)
            .load(&conn)?;

        let results = tags
            .filter(schema::tags::columns::tag_id.eq_any(book_tag_ids))
            .load::<Tag>(&conn)?;
        Ok(results)
    }

    pub fn get_books_of_tag(&self, of_tag_id: i32) -> Result<Vec<Book>, DataError> {
        use schema::book_tags::dsl::*;
        use schema::books::dsl::*;

        let db = &self.pool;
        let conn = db.get()?;

        let tag_book_ids: Vec<String> = book_tags
            .filter(schema::book_tags::columns::tag_id.eq(of_tag_id))
            .select(schema::book_tags::columns::book_id)
            .load(&conn)?;

        let results = books
            .filter(schema::books::columns::book_id.eq_any(tag_book_ids))
            .load::<Book>(&conn)?;
        Ok(results)
    }

    pub fn add_book(&self, new_book: NewBook, book_uuid: Uuid) -> Result<Book, DataError> {
        // use schema::books::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::insert_into(schema::books::table)
            .values(&new_book)
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::books::table
            .find(book_uuid.to_string())
            .first::<Book>(&conn)?;
        Ok(result)
    }

    pub fn add_book_type(&self, new_book_type: NewBookType) -> Result<BookType, DataError> {
        use schema::book_types::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::insert_into(schema::book_types::table)
            .values(&new_book_type)
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::book_types::table
            .order(book_type_id.desc())
            .first::<BookType>(&conn)?;
        Ok(result)
    }

    pub fn add_file(&self, new_file: NewFile, file_uuid: Uuid) -> Result<File, DataError> {
        // use schema::books::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::insert_into(schema::files::table)
            .values(&new_file)
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::files::table
            .find(file_uuid.to_string())
            .first::<File>(&conn)?;
        Ok(result)
    }

    pub fn add_file_type(&self, new_file_type: NewFileType) -> Result<FileType, DataError> {
        use schema::file_types::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::insert_into(schema::file_types::table)
            .values(&new_file_type)
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::file_types::table
            .order(file_type_id.desc())
            .first::<FileType>(&conn)?;
        Ok(result)
    }

    pub fn add_tag(&self, new_tag: NewTag) -> Result<Tag, DataError> {
        use schema::tags::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::insert_into(schema::tags::table)
            .values(&new_tag)
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::tags::table
            .order(tag_id.desc())
            .first::<Tag>(&conn)?;
        Ok(result)
    }

    pub fn add_book_tag(&self, new_book_tag: NewBookTag) -> Result<Vec<Tag>, DataError> {
        // use schema::books::dsl::*;
        use schema::book_tags::dsl::*;
        use schema::tags::dsl::*;

        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::insert_into(schema::book_tags::table)
            .values(&new_book_tag)
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let book_tag_ids: Vec<i32> = book_tags
            .filter(book_id.eq(new_book_tag.book_id))
            .select(schema::book_tags::columns::tag_id)
            .load(&conn)?;
        let results = tags
            .filter(schema::tags::columns::tag_id.eq_any(book_tag_ids))
            .load::<Tag>(&conn)?;
        Ok(results)
    }

    pub fn remove_book(&self, book_uuid: Uuid) -> Result<Book, DataError> {
        use schema::books::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::delete(schema::books::table
            .filter(book_id
                .eq(book_uuid.to_string())))
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::books::table
            .find(book_uuid.to_string())
            .first::<Book>(&conn)?;
        Ok(result)
    }

    pub fn remove_book_type(&self, del_book_type: String) -> Result<Vec<BookType>, DataError> {
        use schema::book_types::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::delete(schema::book_types::table
            .filter(book_type_name
                .eq(del_book_type)))
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::book_types::table
            .order(book_type_id.desc())
            .load::<BookType>(&conn)?;
        Ok(result)
    }

    pub fn remove_file(&self, file_uuid: Uuid) -> Result<File, DataError> {
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::delete(schema::files::table
            .filter(schema::files::columns::file_id
                .eq(file_uuid.to_string())))
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::files::table
            .find(file_uuid.to_string())
            .first::<File>(&conn)?;
        Ok(result)
    }

    pub fn remove_file_type(&self, del_file_type: String) -> Result<Vec<FileType>, DataError> {
        use schema::file_types::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::delete(schema::file_types::table
            .filter(file_type_name
                .eq(del_file_type)))
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::file_types::table
            .order(file_type_id.desc())
            .load::<FileType>(&conn)?;
        Ok(result)
    }

    pub fn remove_tag(&self, del_tag: String) -> Result<Vec<Tag>, DataError> {
        use schema::tags::dsl::*;
        let db = &self.pool;
        let conn = db.get()?;

        let rows =  diesel::delete(schema::tags::table
            .filter(tag_name
                .eq(del_tag)))
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let result = schema::tags::table
            .order(tag_id.desc())
            .load::<Tag>(&conn)?;
        Ok(result)
    }

    pub fn remove_book_tag(&self, del_book_tag: NewBookTag) -> Result<Vec<Tag>, DataError> {
        // use schema::books::dsl::*;
        use schema::book_tags::dsl::*;
        use schema::tags::dsl::*;

        let db = &self.pool;
        let conn = db.get()?;

        let rows = diesel::delete(schema::book_tags::table
            .filter(book_id
                .eq(del_book_tag.book_id)
            .and(schema::book_tags::columns::tag_id
                .eq(del_book_tag.tag_id))))
            .execute(&conn)?;
        validate_rows(rows, 1)?;
        let book_tag_ids: Vec<i32> = book_tags
            .filter(book_id
                .eq(del_book_tag.book_id))
            .select(schema::book_tags::columns::tag_id)
            .load(&conn)?;
        let results = tags
            .filter(schema::tags::columns::tag_id.eq_any(book_tag_ids))
            .load::<Tag>(&conn)?;
        Ok(results)
    }

    pub fn update_book(&self, update_book: UpdateBook) -> Result<Book, DataError> {
        let db = &self.pool;
        let conn = db.get()?;

        let result = update_book.save_changes::<Book>(&conn)?;
        Ok(result)
    }

    pub fn update_book_type(&self, update_book_type: UpdateBookType) -> Result<BookType, DataError> {
        let db = &self.pool;
        let conn = db.get()?;

        let result = update_book_type.save_changes::<BookType>(&conn)?;
        Ok(result)
    }

    pub fn update_file(&self, update_file: UpdateFile) -> Result<File, DataError> {
        let db = &self.pool;
        let conn = db.get()?;

        let result = update_file.save_changes::<File>(&conn)?;
        Ok(result)
    }

    pub fn update_file_type(&self, update_file_type: UpdateFileType) -> Result<FileType, DataError> {
        let db = &self.pool;
        let conn = db.get()?;

        let result = update_file_type.save_changes::<FileType>(&conn)?;
        Ok(result)
    }

    pub fn update_tag(&self, update_tag: UpdateTag) -> Result<Tag, DataError> {
        let db = &self.pool;
        let conn = db.get()?;

        let result = update_tag.save_changes::<Tag>(&conn)?;
        Ok(result)
    }

    // pub fn add_book_tag_bystr(&self, new_file: NewFile, file_uuid: Uuid) -> Result<File, DataError> {
    //     // use schema::books::dsl::*;
    //     let db = &self.pool;
    //     let conn = db.get()?;
    //
    //     let rows =  diesel::insert_into(schema::files::table)
    //         .values(&new_file)
    //         .execute(&conn)?;
    //     validate_rows(rows, 1)?;
    //     let result = schema::files::table
    //         .find(file_uuid.to_string())
    //         .first::<File>(&conn)?;
    //     Ok(result)
    // }

}
