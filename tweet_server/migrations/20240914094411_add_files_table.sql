-- Add migration script here
DROP TABLE IF EXISTS "files";
CREATE TABLE "files" (
	"id" UUID NOT NULL UNIQUE,
	"name" VARCHAR NOT NULL,
	"path" VARCHAR NOT NULL,
	"size" BIGINT NOT NULL,
	"content_type" VARCHAR NOT NULL,
	"created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
	"user_id" INTEGER NOT NULL,
	"description" TEXT,
	"checksum" VARCHAR NOT NULL,
	"is_deleted" BOOLEAN NOT NULL DEFAULT false,
	"is_pub" BOOLEAN NOT NULL DEFAULT false,
	PRIMARY KEY("id")
);
