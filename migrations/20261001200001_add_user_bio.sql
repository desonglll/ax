-- A short self-description shown on the profile page.
ALTER TABLE users
    ADD COLUMN bio VARCHAR(280) NULL;
