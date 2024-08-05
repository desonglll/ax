-- Your SQL goes here
CREATE TABLE file_table (
    file_id UUID PRIMARY KEY,
    file_name VARCHAR NOT NULL,
    file_url VARCHAR NOT NULL,
    uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
