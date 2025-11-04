-- Add up migration script here

CREATE TABLE IF NOT EXISTS Users (
    id CHAR(36) PRIMARY KEY NOT NULL,
    username VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR(30) NOT NULL,
    firstname VARCHAR(30) NOT NULL,
    lastname VARCHAR(30) 
);
