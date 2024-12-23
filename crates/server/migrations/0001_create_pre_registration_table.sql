CREATE TABLE pre_registrations
(
    id    BIGSERIAL NOT NULL UNIQUE,
    email TEXT      NOT NULL,
    otp   TEXT      NOT NULL
);
