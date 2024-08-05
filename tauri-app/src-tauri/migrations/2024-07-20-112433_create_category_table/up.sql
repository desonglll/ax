-- Your SQL goes here
CREATE TABLE category_table (
    category_id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    parent_id INTEGER REFERENCES category_table(category_id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
-- 插入頂級分類
INSERT INTO category_table (name, description) VALUES ('電子產品', '各類電子產品');

-- 插入次級分類
INSERT INTO category_table (name, description, parent_id) VALUES ('手機', '各類手機', 1);
INSERT INTO category_table (name, description, parent_id) VALUES ('電腦', '各類電腦', 1);

-- 插入三級分類
INSERT INTO category_table (name, description, parent_id) VALUES ('筆記本電腦', '各類筆記本電腦', 3);
INSERT INTO category_table (name, description, parent_id) VALUES ('台式電腦', '各類台式電腦', 3);
