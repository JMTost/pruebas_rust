-- Your SQL goes here
CREATE TABLE posts (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);
CREATE TABLE usuarios (
    id SERIAL,
    nombre TEXT,
    email TEXT,
    PRIMARY KEY (id, email)  
);