-- Your SQL goes here
ALTER TABLE session_table ADD CONSTRAINT unique_user_id UNIQUE (user_id);
