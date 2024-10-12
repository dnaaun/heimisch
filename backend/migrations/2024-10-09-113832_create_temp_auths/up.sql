-- Your SQL goes here
CREATE UNLOGGED TABLE temp_auths (
    csrf_token text PRIMARY KEY,
    created_at TIMESTAMP NOT NULL
);

