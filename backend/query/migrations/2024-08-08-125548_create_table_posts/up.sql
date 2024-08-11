-- Your SQL goes here
CREATE TABLE "posts" (
	"id" SERIAL NOT NULL UNIQUE,
	"content" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"user_id" INTEGER NOT NULL,
	"reply_to" INTEGER,
	PRIMARY KEY("id")
);


------------------------------------------------------------------------------------------------
-- 添加 user_name 字段
ALTER TABLE "posts"
ADD COLUMN "user_name" VARCHAR NOT NULL;

-- 创建触发器函数
CREATE OR REPLACE FUNCTION fill_user_name()
RETURNS TRIGGER AS $$
BEGIN
    -- 查询 user_name 并赋值给 NEW.user_name
    SELECT "user_name" INTO NEW.user_name FROM "users" WHERE "id" = NEW.user_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器
CREATE TRIGGER set_user_name
BEFORE INSERT ON "posts"
FOR EACH ROW
EXECUTE FUNCTION fill_user_name();
------------------------------------------------------------------------------------------------

INSERT INTO "posts" ("content", "created_at", "updated_at", "user_id", "reply_to") VALUES
('Content of post 1', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Mike Content of post 2', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 2, NULL),
('Mike Content of post 3', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 2, NULL),
('Content of post 4', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 5', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 6', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 7', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 8', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 9', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 10', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL);
