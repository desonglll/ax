-- Add migration script here
ALTER TABLE posts ADD COLUMN title VARCHAR NOT NULL DEFAULT '';
