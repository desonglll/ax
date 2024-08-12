-- Your SQL goes here
CREATE TABLE public.users (
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

INSERT INTO public.users ("user_name", "email", "password_hash", "full_name", "phone", "created_at", "updated_at", "last_login", "is_active", "is_admin", "profile_picture") VALUES
('root', 'root@example.com', '$2b$12$2Fn1PES7J8hwxs479IKRd.N2840/5Lh8oc53Lly4/I4hlR7l5dblq', 'root', '345-678-9012', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, true, NULL),
('mike', 'mike@example.com', '$2b$12$o/MDTP/oBct4J/yT2tGbUek5DZdBOrFWCvxy5UERnrda6.MniuHBu', 'Mike Shinoda', '123-456-7890', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, true, NULL),
('lds150', 'lds150@example.com', '$2b$12$d.YL4iSlqJvShLFQcuh.0edMV4Z7Qv9mnScxfuPTHVL/WrvRX4sii', 'Desong Lin', '234-567-8901', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, true, false, NULL);
