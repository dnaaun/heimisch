ALTER TABLE webhooks
    ADD CONSTRAINT fk_installation_id FOREIGN KEY (installation_id) REFERENCES installations (installation_id);

