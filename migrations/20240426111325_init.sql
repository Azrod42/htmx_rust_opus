-- Add migration script here
CREATE TABLE main.users (
	id serial4 NOT NULL,
	username varchar(255) NULL,
	email varchar(255) NULL,
	"password" varchar(255) NULL,
	CONSTRAINT users_email_key UNIQUE (email),
	CONSTRAINT users_pkey PRIMARY KEY (id)
);

CREATE TABLE main.visit (
	id serial4 NOT NULL,
	"date" timestamp DEFAULT CURRENT_TIMESTAMP NULL,
	user_agent varchar(255) NULL,
	CONSTRAINT visit_pkey PRIMARY KEY (id)
);
