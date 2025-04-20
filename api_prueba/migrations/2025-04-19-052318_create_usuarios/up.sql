-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE usuarios (
    id SERIAL,
    nombre TEXT NOT NULL,
    email TEXT NOT NULL,
    edad int NOT NULL,
    tipoUsuario TEXT NOT NULL,
    activo BOOLEAN NOT NULL DEFAULT 'f',
    PRIMARY KEY (id, email)
);