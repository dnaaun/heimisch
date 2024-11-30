CREATE UNLOGGED TABLE sessions (
    id uuid PRIMARY KEY NOT NULL,
    data jsonb NOT NULL,
    expiry_date timestamp NOT NULL
);

