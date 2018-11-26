CREATE TABLE file_types (
	file_type_id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
	file_type_name text NOT NULL,
	unique (file_type_name)
);