CREATE TABLE users (
    github_user_id bigint PRIMARY KEY NOT NULL,
    github_username text NOT NULL,
    github_email text,
    github_access_token text NOT NULL
);

