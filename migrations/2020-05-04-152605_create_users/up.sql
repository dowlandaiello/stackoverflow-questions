CREATE TABLE users (
	id SERIAL PRIMARY KEY,

	username VARCHAR(20) UNIQUE KEY
);

CREATE TABLE roles (
	-- Just a UID for this entry
	id SERIAL,

	-- The use whose roles will be specified
	user_id BIGINT UNSIGNED NOT NULL UNIQUE PRIMARY KEY,

	administrator BOOLEAN,
	sponsor BOOLEAN,
	bot BOOLEAN
);
