pub struct Tag {
    pub tag_id: i32,
    pub tag_name: String,
}

pub struct NewTag<'a> {
    pub tag_name: &'a str,
}

pub struct UpdateTag<'a> {
    pub tag_id: &'a i32,
    pub tag_name: &'a str,
}

pub struct BookTag {
    pub id: i32,
    pub book_id: String,
    pub tag_id: i32,
}

pub struct NewBookTag<'a> {
    pub book_id: &'a str,
    pub tag_id: &'a i32,
}