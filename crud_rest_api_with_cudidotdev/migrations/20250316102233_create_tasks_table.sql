-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    priority BOOLEAN NOT NULL DEFAULT false
);