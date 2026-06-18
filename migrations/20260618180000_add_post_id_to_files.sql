-- Add migration script here
ALTER TABLE files ADD COLUMN post_id UUID REFERENCES posts(id) ON DELETE SET NULL;
