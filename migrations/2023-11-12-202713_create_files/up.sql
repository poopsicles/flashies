-- Your SQL goes here
CREATE TABLE files (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    hash VARCHAR(255),
    data LONGBLOB
)