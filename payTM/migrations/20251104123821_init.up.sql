-- Add up migration script here

CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username VARCHAR(30) NOT NULL UNIQUE,
    password TEXT NOT NULL,
    firstname VARCHAR(30) NOT NULL,
    lastname VARCHAR(30) 
);
