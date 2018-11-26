CREATE TABLE book_tags (
	id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
	book_id text NOT NULL,
	tag_id integer NOT NULL,
	FOREIGN KEY(book_id) REFERENCES books(book_id),
	FOREIGN KEY(tag_id) REFERENCES tags(tag_id)
);