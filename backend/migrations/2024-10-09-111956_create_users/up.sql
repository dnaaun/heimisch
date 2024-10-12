CREATE TABLE users (
    github_user_id bigint PRIMARY KEY,
    github_username varchar(255),
    github_email varchar(255),
    github_access_token text NOT NULL
);

-- Create an index on github_username if you frequently query by username
CREATE UNIQUE INDEX idx_users_github_username ON users (github_username)
WHERE
    github_username IS NOT NULL;

-- Indexing email could be optional based on your access patterns.
CREATE INDEX idx_users_github_email ON users (github_email);

