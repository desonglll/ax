-- Add migration script here
DROP TABLE IF EXISTS "users";
CREATE TABLE "users" (
	"id" SERIAL NOT NULL UNIQUE,
	"user_name" VARCHAR NOT NULL UNIQUE,
	"email" VARCHAR NOT NULL UNIQUE,
	"password_hash" VARCHAR NOT NULL,
	"full_name" VARCHAR,
	"phone" VARCHAR,
	"created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
	"last_login" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
	"is_active" BOOLEAN NOT NULL DEFAULT true,
	"is_admin" BOOLEAN NOT NULL DEFAULT false,
	"profile_picture" UUID,
	PRIMARY KEY("id")
);
