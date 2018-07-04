table! {
    file_types (file_type_id) {
        file_type_id -> Integer,
        file_type_name -> Text,
    }
}

table! {
    files (file_id) {
        file_id -> Integer,
        file_type_id -> Nullable<Integer>,
        file_path -> Nullable<Text>,
        file_size -> Nullable<Integer>,
    }
}

table! {
    media_tag_maps (media_id, tag_id) {
        media_id -> Integer,
        tag_id -> Integer,
    }
}

table! {
    media_types (media_type_id) {
        media_type_id -> Integer,
        media_type_name -> Text,
    }
}

table! {
    medium (media_id) {
        media_id -> Integer,
        name -> Text,
        media_type_id -> Integer,
        add_date -> Timestamp,
        last_open_date -> Nullable<Timestamp>,
        file_id -> Integer,
        media_meta -> Nullable<Text>,
    }
}

table! {
    tags (tag_id) {
        tag_id -> Integer,
        tag_name -> Nullable<Text>,
    }
}

joinable!(files -> file_types (file_type_id));
joinable!(media_tag_maps -> tags (tag_id));
joinable!(medium -> media_types (media_type_id));

allow_tables_to_appear_in_same_query!(
    file_types,
    files,
    media_tag_maps,
    media_types,
    medium,
    tags,
);
