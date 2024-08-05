-- Your SQL goes here

CREATE TABLE "post_table" (
  "post_id" serial,
  "title" varchar(255) DEFAULT 'Untitled',
  "body" text DEFAULT 'No Content',
  "user_id" int4 DEFAULT -1,
  "status" varchar(255) DEFAULT 'unknown status',
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "updated_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "username" varchar(255),
  PRIMARY KEY ("post_id")
);

-- 向 posts 表插入示例数据
INSERT INTO post_table (title, body, user_id, status, username, created_at, updated_at)
VALUES
('Post 1', 'This is post 1', 1, 'active', 'john_doe', '2023-01-01', '2023-01-01'),
('Post 2', 'This is post 2', 2, 'active', 'jane_smith', '2023-02-01', '2023-02-01'),
('Post 3', 'This is post 3', 3, 'active', 'mike_johnson', '2023-03-01', '2023-03-01'),
('Post 4', 'This is post 4', 1, 'active', 'john_doe', '2023-04-01', '2023-04-01'),
('Post 5', 'This is post 5', 2, 'active', 'jane_smith', '2023-05-01', '2023-05-01'),
('Post 6', 'This is post 6', 3, 'active', 'mike_johnson', '2023-06-01', '2023-06-01'),
('Post 7', 'This is post 7', 4, 'active', 'sara_williams', '2023-07-01', '2023-07-01'),
('Post 8', 'This is post 8', 5, 'active', 'tom_brown', '2023-08-01', '2023-08-01'),
('Post 9', 'This is post 9', 6, 'active', 'emily_davis', '2023-09-01', '2023-09-01'),
('Post 10', 'This is post 10', 7, 'active', 'robert_martinez', '2023-10-01', '2023-10-01'),
('Post 11', 'This is post 11', 8, 'active', 'linda_garcia', '2023-11-01', '2023-11-01'),
('Post 12', 'This is post 12', 9, 'active', 'david_wilson', '2023-12-01', '2023-12-01'),
('Post 13', 'This is post 13', 10, 'active', 'jessica_moore', '2024-01-01', '2024-01-01'),
('Post 14', 'This is post 14', 1, 'active', 'john_doe', '2024-02-01', '2024-02-01'),
('Post 15', 'This is post 15', 2, 'active', 'jane_smith', '2024-03-01', '2024-03-01'),
('Post 16', 'This is post 16', 3, 'active', 'mike_johnson', '2024-04-01', '2024-04-01'),
('Post 17', 'This is post 17', 4, 'active', 'sara_williams', '2024-05-01', '2024-05-01'),
('Post 18', 'This is post 18', 5, 'active', 'tom_brown', '2024-06-01', '2024-06-01'),
('Post 19', 'This is post 19', 6, 'active', 'emily_davis', '2024-07-01', '2024-07-01'),
('Post 20', 'This is post 20', 7, 'active', 'robert_martinez', '2024-08-01', '2024-08-01');

ALTER TABLE "post_table" ADD CONSTRAINT "user_id" FOREIGN KEY ("user_id") REFERENCES "user_table" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "post_table" ADD CONSTRAINT "username" FOREIGN KEY ("username") REFERENCES "user_table" ("username");
