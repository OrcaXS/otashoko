CREATE TABLE folders (
	folder_id text NOT NULL PRIMARY KEY,
	folder_path text NOT NULL,
	folder_size integer,
	FOREIGN KEY (folder_id) REFERENCES books(folder_id),
	unique (folder_path)
);