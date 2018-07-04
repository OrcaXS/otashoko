CREATE TABLE medium (
	media_id integer PRIMARY KEY AUTOINCREMENT,
	name text NOT NULL,
	media_type_id integer NOT NULL,
	add_date datetime NOT NULL,
	last_open_date datetime,
	file_id integer NOT NULL,
	media_meta blob,
	FOREIGN KEY (media_type_id) REFERENCES media_types(media_type_id)
);