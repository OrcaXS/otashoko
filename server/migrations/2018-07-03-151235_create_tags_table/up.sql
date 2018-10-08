CREATE TABLE tags (
  tag_id integer NOT NULL PRIMARY KEY AUTOINCREMENT,
  tag_name text,
  unique (tag_name)
);