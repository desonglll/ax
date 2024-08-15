-- Your SQL goes here
CREATE TABLE "comments" (
	"id" SERIAL NOT NULL UNIQUE,
	"content" TEXT NOT NULL,
	"reply_to" INTEGER NOT NULL,
	"user_id" INTEGER NOT NULL,
	"user_name" VARCHAR NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"reactions" JSONB,
	"reply_to_type" VARCHAR NOT NULL DEFAULT 'post',
	PRIMARY KEY("id")
);

-- ALTER TABLE "comments"
-- ADD FOREIGN KEY("reply_to") REFERENCES "posts"("id")
-- ON UPDATE CASCADE ON DELETE CASCADE;
ALTER TABLE "comments"
ADD FOREIGN KEY("user_id") REFERENCES "users"("id")
ON UPDATE CASCADE ON DELETE CASCADE;

------------------------------------------------------------------------------------------------
-- 创建触发器函数
CREATE OR REPLACE FUNCTION fill_comment_user_name()
RETURNS TRIGGER AS $$
BEGIN
    -- 查询 user_name 并赋值给 NEW.user_name
    SELECT "user_name" INTO NEW.user_name FROM "users" WHERE "id" = NEW.user_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器
CREATE TRIGGER set_comment_user_name
BEFORE INSERT ON public.comments
FOR EACH ROW
EXECUTE FUNCTION fill_comment_user_name();

CREATE TRIGGER update_comment_user_name
BEFORE UPDATE ON public.comments
FOR EACH ROW
EXECUTE FUNCTION fill_comment_user_name();
------------------------------------------------------------------------------------------------