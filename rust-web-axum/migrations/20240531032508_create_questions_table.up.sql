-- Add up migration script here
CREATE TABLE IF NOT EXISTS questions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    question VARCHAR(255) NOT NULL,
    content VARCHAR(255) NOT NULL,
    tags JSON NOT NULL
)