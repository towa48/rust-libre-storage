-- Your SQL goes here

CREATE TABLE folders (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    owner_id INTEGER NOT NULL REFERENCES users(id),
    date_created TIMESTAMP NOT NULL,
    date_validated TIMESTAMP,
    depth INTEGER NOT NULL DEFAULT 0,
    lft INTEGER NOT NULL,
    rgt INTEGER NOT NULL,

    UNIQUE(owner_id, name, depth)
);

CREATE TABLE files (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    folder_id INTEGER NOT NULL REFERENCES folders(id),
    owner_id INTEGER NOT NULL REFERENCES users(id),
    date_created TIMESTAMP NOT NULL,
    date_validated TIMESTAMP,
    sha1_hash TEXT NOT NULL
);