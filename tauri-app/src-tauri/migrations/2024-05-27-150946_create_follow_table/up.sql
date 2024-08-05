-- Your SQL goes here
CREATE TABLE "follow_table" (
  "following_user_id" int4 NOT NULL DEFAULT -1,
  "followed_user_id" int4 NOT NULL DEFAULT -1,
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "follow_id" serial,
  PRIMARY KEY ("follow_id")
);


-- 向 follow 表插入示例数据
INSERT INTO follow_table (following_user_id, followed_user_id, created_at)
VALUES
(1, 2, '2023-01-01'),
(2, 3, '2023-02-01'),
(3, 1, '2023-03-01'),
(1, 4, '2023-04-01'),
(4, 5, '2023-05-01'),
(5, 6, '2023-06-01'),
(6, 1, '2023-07-01'),
(2, 5, '2023-08-01'),
(3, 6, '2023-09-01'),
(4, 2, '2023-10-01'),
(5, 3, '2023-11-01'),
(6, 4, '2023-12-01'),
(1, 5, '2023-01-15'),
(2, 6, '2023-02-15'),
(3, 4, '2023-03-15'),
(4, 1, '2023-04-15'),
(5, 2, '2023-05-15'),
(6, 3, '2023-06-15');

ALTER TABLE "follow_table" ADD CONSTRAINT "following_user_id" FOREIGN KEY ("following_user_id") REFERENCES "user_table" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "follow_table" ADD CONSTRAINT "followed_user_id" FOREIGN KEY ("followed_user_id") REFERENCES "user_table" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
