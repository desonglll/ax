-- Your SQL goes here
CREATE TABLE "files" (
	"id" UUID NOT NULL UNIQUE,
	"name" VARCHAR NOT NULL,
	"path" VARCHAR NOT NULL,
	"size" BIGINT NOT NULL,
	"content_type" VARCHAR NOT NULL,
	"created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"user_id" INTEGER NOT NULL,
	"description" TEXT,
	"checksum" VARCHAR NOT NULL,
	"is_deleted" BOOLEAN NOT NULL DEFAULT false,
	PRIMARY KEY("id")
);
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

INSERT INTO "files" ("id", "name", "path", "size", "content_type", "created_at", "updated_at", "user_id", "description", "checksum", "is_deleted") VALUES
(uuid_generate_v4(), 'file1.txt', '/uploads/file1.txt', 1024, 'text/plain', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 'Sample text file', 'checksum1', false),
(uuid_generate_v4(), 'image1.jpg', '/uploads/image1.jpg', 2048, 'image/jpeg', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, 'Sample image file', 'checksum2', false),
(uuid_generate_v4(), 'document1.pdf', '/uploads/document1.pdf', 3072, 'application/pdf', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 2, 'Sample PDF document', 'checksum3', false),
(uuid_generate_v4(), 'video1.mp4', '/uploads/video1.mp4', 4096, 'video/mp4', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 2, 'Sample video file', 'checksum4', false),
(uuid_generate_v4(), 'audio1.mp3', '/uploads/audio1.mp3', 5120, 'audio/mpeg', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 3, 'Sample audio file', 'checksum5', false),
(uuid_generate_v4(), 'archive1.zip', '/uploads/archive1.zip', 6144, 'application/zip', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 3, 'Sample archive file', 'checksum6', false),
(uuid_generate_v4(), 'spreadsheet1.xlsx', '/uploads/spreadsheet1.xlsx', 7168, 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 4, 'Sample spreadsheet file', 'checksum7', false),
(uuid_generate_v4(), 'presentation1.pptx', '/uploads/presentation1.pptx', 8192, 'application/vnd.openxmlformats-officedocument.presentationml.presentation', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 4, 'Sample presentation file', 'checksum8', false),
(uuid_generate_v4(), 'script1.js', '/uploads/script1.js', 9216, 'application/javascript', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 5, 'Sample JavaScript file', 'checksum9', false),
(uuid_generate_v4(), 'archive2.tar.gz', '/uploads/archive2.tar.gz', 10240, 'application/gzip', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 5, 'Sample compressed archive file', 'checksum10', false);
