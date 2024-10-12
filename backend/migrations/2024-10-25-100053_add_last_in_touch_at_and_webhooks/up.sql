ALTER TABLE users
    ADD COLUMN last_last_in_touch_at TIMESTAMP;

CREATE TABLE webhooks (
    id uuid DEFAULT uuid_generate_v4 () PRIMARY KEY NOT NULL,
    webhook_content jsonb NOT NULL,
    installation_id bigint NOT NULL
);

