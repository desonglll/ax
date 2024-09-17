-- Add migration script here
DROP TABLE IF EXISTS "reactions";
CREATE TABLE "reactions" (
	"id" SERIAL NOT NULL UNIQUE,
	"user_id" INTEGER NOT NULL,
	"to_id" INTEGER NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"reaction_name" VARCHAR NOT NULL,
	"to_type" VARCHAR NOT NULL DEFAULT 'post',
	PRIMARY KEY("id")
);
-- 为 `user_id` 和 `post_id` 添加唯一约束，确保同一用户不能对同一帖子添加相同的反应
ALTER TABLE reactions ADD CONSTRAINT unique_user_post_reaction UNIQUE (user_id, to_id, reaction_name);
