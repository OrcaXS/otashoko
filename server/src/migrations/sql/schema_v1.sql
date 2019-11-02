CREATE TABLE schema_version (
  version integer NOT NULL
);

CREATE TABLE books (
  book_id text NOT NULL PRIMARY KEY,
  name text NOT NULL,
  book_type_id integer NOT NULL,
  add_date timestamp NOT NULL,
  last_open_date timestamp,
  file_id text NOT NULL,
  book_meta text,
  FOREIGN KEY (book_type_id) REFERENCES book_types (book_type_id)
);

CREATE TABLE tags (
  tag_id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
  tag_name text NOT NULL,
  UNIQUE (tag_name)
);

CREATE TABLE files (
  file_id text NOT NULL PRIMARY KEY,
  file_type_id integer NOT NULL,
  file_path text,
  file_size integer,
  FOREIGN KEY (file_id) REFERENCES books (file_id),
  FOREIGN KEY (file_type_id) REFERENCES file_types (file_type_id)
);

CREATE TABLE book_types (
  book_type_id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
  book_type_name text NOT NULL,
  UNIQUE (book_type_name)
);

CREATE TABLE file_types (
  file_type_id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
  file_type_name text NOT NULL,
  UNIQUE (file_type_name)
);

CREATE TABLE book_tags (
  id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
  book_id text NOT NULL,
  tag_id integer NOT NULL,
  FOREIGN KEY (book_id) REFERENCES books (book_id),
  FOREIGN KEY (tag_id) REFERENCES tags (tag_id)
);

