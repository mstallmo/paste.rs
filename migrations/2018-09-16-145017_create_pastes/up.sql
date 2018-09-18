-- Your SQL goes here
CREATE TABLE pastes (
    id SERIAL PRIMARY KEY,
    hash TEXT NOT NULL,
    paste TEXT NOT NULL
)