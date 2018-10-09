CREATE TABLE book_tag_maps (
	book_id text NOT NULL,
	tag_id integer NOT NULL,
	PRIMARY KEY(book_id, tag_id),
	FOREIGN KEY(book_id) REFERENCES books(book_id),
	FOREIGN KEY(tag_id) REFERENCES tags(tag_id)
);