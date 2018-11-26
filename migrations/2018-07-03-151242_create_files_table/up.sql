CREATE TABLE files (
	file_id text NOT NULL PRIMARY KEY,
	folder_id text NOT NULL,
	file_name text NOT NULL,
	file_size integer,
	FOREIGN KEY (folder_id) REFERENCES folders(folder_id)
);