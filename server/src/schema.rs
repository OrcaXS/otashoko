table! {
    book_tags (id) {
        id -> Integer,
        book_id -> Text,
        tag_id -> Integer,
    }
}

table! {
    book_types (book_type_id) {
        book_type_id -> Integer,
        book_type_name -> Text,
    }
}

table! {
    books (book_id) {
        book_id -> Text,
        name -> Text,
        book_type_id -> Integer,
        add_date -> Timestamp,
        last_open_date -> Nullable<Timestamp>,
        file_id -> Text,
        book_meta -> Nullable<Text>,
    }
}

table! {
    file_types (file_type_id) {
        file_type_id -> Integer,
        file_type_name -> Text,
    }
}

table! {
    files (file_id) {
        file_id -> Text,
        file_type_id -> Integer,
        file_path -> Nullable<Text>,
        file_size -> Nullable<Integer>,
    }
}

table! {
    tags (tag_id) {
        tag_id -> Integer,
        tag_name -> Text,
    }
}

joinable!(book_tags -> books (book_id));
joinable!(book_tags -> tags (tag_id));
joinable!(books -> book_types (book_type_id));
joinable!(files -> file_types (file_type_id));

allow_tables_to_appear_in_same_query!(
    book_tags,
    book_types,
    books,
    file_types,
    files,
    tags,
);
