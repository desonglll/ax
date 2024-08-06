-- Your SQL goes here
CREATE TABLE "users" (
	"id" SERIAL NOT NULL UNIQUE,
	"user_name" VARCHAR NOT NULL UNIQUE,
	"email" VARCHAR NOT NULL UNIQUE,
	"password_hash" VARCHAR NOT NULL,
	"full_name" VARCHAR,
	"phone" VARCHAR,
	"created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"last_login" TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	"is_active" BOOLEAN NOT NULL DEFAULT true,
	"is_admin" BOOLEAN NOT NULL DEFAULT false,
	"profile_picture" UUID,
	PRIMARY KEY("id")
);

INSERT INTO "users" ("user_name", "email", "password_hash", "full_name", "phone", "created_at", "updated_at", "last_login", "is_active", "is_admin", "profile_picture") VALUES
('alice', 'alice@example.com', '$2b$12$o/MDTP/oBct4J/yT2tGbUek5DZdBOrFWCvxy5UERnrda6.MniuHBu', 'Alice Johnson', '123-456-7890', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL),
('bob', 'bob@example.com', 'hash2', 'Bob Smith', '234-567-8901', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL),
('carol', 'carol@example.com', 'hash3', 'Carol White', '345-678-9012', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL),
('dave', 'dave@example.com', 'hash4', 'Dave Brown', '456-789-0123', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL),
('eve', 'eve@example.com', 'hash5', 'Eve Davis', '567-890-1234', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL),
('frank', 'frank@example.com', 'hash6', 'Frank Miller', '678-901-2345', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL),
('grace', 'grace@example.com', 'hash7', 'Grace Lee', '789-012-3456', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL),
('hank', 'hank@example.com', 'hash8', 'Hank Wilson', '890-123-4567', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL),
('ida', 'ida@example.com', 'hash9', 'Ida Martinez', '901-234-5678', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL),
('jack', 'jack@example.com', 'hash10', 'Jack Taylor', '012-345-6789', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL);
