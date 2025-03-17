-- Add migration script here
CREATE TABLE IF NOT EXISTS tasks (
    task_id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    priority BOOLEAN NOT NULL DEFAULT false
);