-- Add up migration script here
CREATE TABLE IF NOT EXISTS questions (
    id SERIAL PRIMARY KEY,
    question VARCHAR NOT NULL,
    content VARCHAR NOT NULL,
    tags VARCHAR NOT NULL
)