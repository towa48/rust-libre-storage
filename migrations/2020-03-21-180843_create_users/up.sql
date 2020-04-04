CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    date_created TIMESTAMP NOT NULL,
    salt TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT 0,
    should_initialize BOOLEAN NOT NULL DEFAULT 0,

    UNIQUE (username)
);

insert into users (username, date_created, salt, password_hash, is_admin, should_initialize) values ('admin', CURRENT_TIMESTAMP, '', '', 1, 1);
