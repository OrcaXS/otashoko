CREATE TABLE books (
	book_id text NOT NULL PRIMARY KEY,
	name text NOT NULL,
	book_type_id integer NOT NULL,
	add_date timestamp NOT NULL,
	last_open_date timestamp,
	file_id text NOT NULL,
	book_meta text,
	FOREIGN KEY (book_type_id) REFERENCES book_types(book_type_id)
);