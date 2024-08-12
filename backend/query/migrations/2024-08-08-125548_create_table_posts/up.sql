-- Your SQL goes here
CREATE TABLE public.posts (
	"id" SERIAL NOT NULL UNIQUE,
	"content" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"user_id" INTEGER NOT NULL,
	"reply_to" INTEGER,
	"user_name" VARCHAR NOT NULL,
	"reactions" JSONB,
	PRIMARY KEY("id")
);

