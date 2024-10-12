CREATE TABLE installations (
    installation_id bigint PRIMARY KEY NOT NULL,
    created_at timestamp NOT NULL,
    github_user_id bigint NOT NULL,
    FOREIGN KEY (github_user_id) REFERENCES users (github_user_id)
);

