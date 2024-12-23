CREATE TABLE users
(
    id       BIGSERIAL NOT NULL UNIQUE,
    email    TEXT      NOT NULL UNIQUE,
    username TEXT      NOT NULL,
    PRIMARY KEY (id)
);
