CREATE TABLE keys (
	name VARCHAR NOT NULL, 
	description VARCHAR, 
	active BOOLEAN NOT NULL, 
	PRIMARY KEY (name)
);

CREATE TABLE users (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	username VARCHAR(64) NOT NULL UNIQUE, 
    display_name VARCHAR(120) UNIQUE, 
	email VARCHAR(120) NOT NULL UNIQUE, 
	password_hash VARCHAR(128), 
	can_login BOOLEAN NOT NULL
);

CREATE TABLE assignments (
	id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
	"user" VARCHAR NOT NULL, 
	"key" VARCHAR NOT NULL, 
	date_out DATE NOT NULL, 
	date_in DATE, 
    UNIQUE("user","key"),
	FOREIGN KEY("key") REFERENCES keys (name), 
	FOREIGN KEY("user") REFERENCES users (username)
);
