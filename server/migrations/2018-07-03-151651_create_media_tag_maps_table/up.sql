CREATE TABLE media_tag_maps (
	media_id integer NOT NULL,
	tag_id integer NOT NULL,
	PRIMARY KEY(media_id, tag_id),
	FOREIGN KEY(media_id) REFERENCES medium(medium_id),
	FOREIGN KEY(tag_id) REFERENCES tags(tag_id)
);