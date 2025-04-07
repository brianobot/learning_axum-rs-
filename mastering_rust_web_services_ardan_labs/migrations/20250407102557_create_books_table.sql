-- Add migration script here
CREATE TABLE books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    author TEXT
);

INSERT INTO books (title, author) VALUES ("hands-on Rust", "Wolverson, Herbert");
INSERT INTO books (title, author) VALUES ("Rust Brain Teaser", "Wolverson, Herbert");
