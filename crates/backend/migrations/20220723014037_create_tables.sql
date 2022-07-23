CREATE TABLE keys (
	name VARCHAR NOT NULL, 
	description VARCHAR, 
	status VARCHAR, 
	PRIMARY KEY (name)
);

CREATE TABLE users (
	id INTEGER NOT NULL, 
	username VARCHAR(64), 
	email VARCHAR(120), 
	password_hash VARCHAR(128), 
	can_login BOOLEAN, display_name VARCHAR(120), 
	PRIMARY KEY (id)
);

CREATE UNIQUE INDEX ix_users_email ON users (email);
CREATE UNIQUE INDEX ix_users_username ON users (username);
CREATE TABLE assignments (
	id INTEGER NOT NULL, 
	"user" VARCHAR, 
	"key" VARCHAR, 
	date_out DATE, 
	date_in DATE, 
	PRIMARY KEY (id), 
	FOREIGN KEY("key") REFERENCES keys (name), 
	FOREIGN KEY("user") REFERENCES users (username)
);
