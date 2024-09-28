-- Add migration script here
DROP TABLE IF EXISTS "posts";
CREATE TABLE "posts" (
	"id" SERIAL NOT NULL UNIQUE,
	"content" TEXT NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"user_id" INTEGER NOT NULL,
	"reply_to" INTEGER,
	"user_name" VARCHAR NOT NULL,
	"like_count" INTEGER DEFAULT 0,
	"dislike_count" INTEGER DEFAULT 0,
	"engagement_rate" FLOAT NOT NULL DEFAULT 0.0,
	PRIMARY KEY("id")
);