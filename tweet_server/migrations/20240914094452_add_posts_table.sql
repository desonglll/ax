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
	"reactions" JSONB,
	PRIMARY KEY("id")
);