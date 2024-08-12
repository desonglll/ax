-- Your SQL goes here
CREATE TABLE "comment_reactions" (
	"id" SERIAL NOT NULL UNIQUE,
	"user_id" INTEGER NOT NULL,
	"comment_id" INTEGER NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"reaction_id" INTEGER NOT NULL,
	"reaction_name" VARCHAR NOT NULL,
	PRIMARY KEY("id"),
	UNIQUE ("user_id", "comment_id", "reaction_name") -- 添加唯一约束
);

