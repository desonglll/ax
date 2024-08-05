-- Your SQL goes here
CREATE TABLE "user_table" (
  "user_id" serial,
  "username" varchar(255) NOT NULL,
  "role_id" int4 DEFAULT -1,
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "email" varchar(255) DEFAULT '',
  "password" varchar(255) NOT NULL,
  "full_name" varchar(255) DEFAULT '',
  "avatar_url" varchar(255) DEFAULT '',
  "bio" text DEFAULT '',
  "updated_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "mobile_phone" varchar(255) NOT NULL,
  PRIMARY KEY ("user_id"),
  CONSTRAINT "username" UNIQUE ("username"),
  CONSTRAINT "phonenumber" UNIQUE ("mobile_phone")
);

-- 向 users 表插入示例数据
INSERT INTO user_table (username, role_id, email, password, full_name, avatar_url, bio, mobile_phone)
VALUES
('john_doe', 1, 'john.doe@example.com', 'password1', 'John Doe', 'https://example.com/avatar1.jpg', 'Hello, I am John Doe', '123-456-7890'),
('jane_smith', 2, 'jane.smith@example.com', 'password2', 'Jane Smith', 'https://example.com/avatar2.jpg', 'Hi, I am Jane Smith', '987-654-3210'),
('mike_johnson', 3, 'mike.johnson@example.com', 'password3', 'Mike Johnson', 'https://example.com/avatar3.jpg', 'Nice to meet you!', '111-222-3333'),
('sara_williams', 2, 'sara.williams@example.com', 'password4', 'Sara Williams', 'https://example.com/avatar4.jpg', 'Marketing enthusiast', '444-555-6666'),
('tom_brown', 1, 'tom.brown@example.com', 'password5', 'Tom Brown', 'https://example.com/avatar5.jpg', 'Experienced Sales Director', '777-888-9999'),
('emily_davis', 2, 'emily.davis@example.com', 'password6', 'Emily Davis', 'https://example.com/avatar6.jpg', 'Passionate about product management', '123-321-1234'),
('robert_martinez', 1, 'robert.martinez@example.com', 'password7', 'Robert Martinez', 'https://example.com/avatar7.jpg', 'Operations Manager with a focus on efficiency', '456-654-4567'),
('linda_garcia', 2, 'linda.garcia@example.com', 'password8', 'Linda Garcia', 'https://example.com/avatar8.jpg', 'Dedicated customer support lead', '789-987-7890'),
('david_wilson', 3, 'david.wilson@example.com', 'password9', 'David Wilson', 'https://example.com/avatar9.jpg', 'Software engineer with a love for coding', '321-432-3210'),
('jessica_moore', 2, 'jessica.moore@example.com', 'password10', 'Jessica Moore', 'https://example.com/avatar10.jpg', 'UX designer creating seamless experiences', '654-765-6543');

ALTER TABLE "user_table" ADD CONSTRAINT "role_id" FOREIGN KEY ("role_id") REFERENCES "role_table" ("role_id") ON DELETE CASCADE ON UPDATE CASCADE;
