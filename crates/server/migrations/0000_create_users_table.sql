CREATE TABLE users
(
    id           BIGSERIAL NOT NULL UNIQUE,
    email        TEXT      NOT NULL UNIQUE,
    username     TEXT      NOT NULL,
    srp_verifier BYTEA     NOT NULL,
    srp_salt     BYTEA     NOT NULL,
    PRIMARY KEY (id)
);
