-- Add up migration script here
CREATE TABLE IF NOT EXISTS questions (
    id INT PRIMARY KEY NOT NULL,
    question VARCHAR(255) NOT NULL,
    answer VARCHAR(255) NOT NULL,
    tags VARCHAR(255) NOT NULL,
);