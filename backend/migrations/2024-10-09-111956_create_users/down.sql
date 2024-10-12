-- Drop indices first as they depend on the table
DROP INDEX IF EXISTS idx_users_github_username;
DROP INDEX IF EXISTS idx_users_github_email;

-- Drop the table
DROP TABLE IF EXISTS users;
