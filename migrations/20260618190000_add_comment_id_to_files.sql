-- Add migration script here
ALTER TABLE files ADD COLUMN comment_id UUID REFERENCES comments(id) ON DELETE SET NULL;
