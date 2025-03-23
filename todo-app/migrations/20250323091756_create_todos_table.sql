-- Add migration script here
CREATE TABLE IF NOT EXISTS todos 
(
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    name            VARCHAR(250) NOT NULL,
    description     VARCHAR(1000)
);