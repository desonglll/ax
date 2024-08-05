-- Your SQL goes here
CREATE TABLE "role_table" (
  "role_id" serial,
  "role_name" varchar(255) NOT NULL,
  "description" varchar(255) DEFAULT '',
  "permissions" varchar(255) DEFAULT '',
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "updated_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY ("role_id"),
  CONSTRAINT "role_name" UNIQUE ("role_name")
);

-- 向 role_table 表插入示例数据
INSERT INTO role_table (role_name, description, permissions)
VALUES
('Admin', 'Administrator role with full access', 'all'),
('User', 'Standard user role with limited access', 'read,write'),
('Guest', 'Guest role with read-only access', 'read');
-- 向 role_table 表插入示例数据
INSERT INTO role_table (role_id, role_name, description, permissions)
VALUES
(-1, 'NULL', 'NULL', 'NULL');
