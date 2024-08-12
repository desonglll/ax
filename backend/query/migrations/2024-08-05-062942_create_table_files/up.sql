-- Your SQL goes here
CREATE TABLE public.files (
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
	PRIMARY KEY("id")
);
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

INSERT INTO public.files ("id", "name", "path", "size", "content_type", "created_at", "updated_at", "user_id", "description", "checksum", "is_deleted") VALUES
(uuid_generate_v4(), 'file1.txt', '/uploads/file1.txt', 1024, 'text/plain', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 'Sample text file', 'checksum1', false),
(uuid_generate_v4(), 'image1.jpg', '/uploads/image1.jpg', 2048, 'image/jpeg', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 'Sample image file', 'checksum2', false),
(uuid_generate_v4(), 'document1.pdf', '/uploads/document1.pdf', 3072, 'application/pdf', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 2, 'Sample PDF document', 'checksum3', false),
(uuid_generate_v4(), 'video1.mp4', '/uploads/video1.mp4', 4096, 'video/mp4', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 2, 'Sample video file', 'checksum4', false);
