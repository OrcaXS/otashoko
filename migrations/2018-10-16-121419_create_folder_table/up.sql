CREATE TABLE folders (
	folder_id text NOT NULL PRIMARY KEY,
	folder_path text NOT NULL,
	folder_size integer,
	unique (folder_path)
);