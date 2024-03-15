CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255),
    email VARCHAR(255) UNIQUE,
    password VARCHAR(255)
);

CREATE TABLE visit (
 id SERIAL primary key,
 date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
ALTER TABLE visite ADD COLUMN user_agent VARCHAR(255);

CREATE TABLE mariage_response (
    id SERIAL PRIMARY KEY,
    number_of_guests VARCHAR(255),
    exigences_alimentaires VARCHAR(255),
    name VARCHAR(255),
    phone VARCHAR(255),
    email VARCHAR(255)
);

CREATE TABLE mariage_music (
    id SERIAL PRIMARY KEY,
    sugest_by VARCHAR(255),
    musics VARCHAR(255),
);
