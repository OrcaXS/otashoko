CREATE TABLE medium (
	media_id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
	name text NOT NULL,
	media_type_id integer NOT NULL,
	add_date timestamp NOT NULL,
	last_open_date timestamp,
	file_id integer NOT NULL,
	media_meta text,
	FOREIGN KEY (media_type_id) REFERENCES media_types(media_type_id)
);