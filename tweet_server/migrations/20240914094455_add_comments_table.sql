-- Add migration script here
DROP TABLE IF EXISTS "comments";
CREATE TABLE "comments" (
	"id" SERIAL NOT NULL UNIQUE,
	"content" TEXT NOT NULL,
	"reply_to" INTEGER NOT NULL,
	"user_id" INTEGER NOT NULL,
	"user_name" VARCHAR NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"reactions" VARCHAR NOT NULL,
	"reply_to_type" VARCHAR NOT NULL DEFAULT 'post',
	PRIMARY KEY("id")
);

