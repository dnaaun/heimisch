ALTER TABLE installations
    ADD CONSTRAINT fk_github_user_id FOREIGN KEY (github_user_id) REFERENCES login_users (github_user_id);

