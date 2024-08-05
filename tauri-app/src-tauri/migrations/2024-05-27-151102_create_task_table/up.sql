-- Your SQL goes here

CREATE TABLE "task_table" (
  "task_id" serial,
  "title" text NOT NULL,
  "content" text DEFAULT '',
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "updated_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "user_id" int4 DEFAULT -1,
  PRIMARY KEY ("task_id")
);

-- 向 tasks 表插入示例数据
INSERT INTO task_table (title, content, user_id, created_at, updated_at)
VALUES
('Task 1', 'Task 1 content', 1, '2023-01-01', '2023-01-01'),
('Task 2', 'Task 2 content', 2, '2023-02-01', '2023-02-01'),
('Task 3', 'Task 3 content', 3, '2023-03-01', '2023-03-01'),
('Task 4', 'Task 4 content', 1, '2023-04-01', '2023-04-01'),
('Task 5', 'Task 5 content', 2, '2023-05-01', '2023-05-01'),
('Task 6', 'Task 6 content', 3, '2023-06-01', '2023-06-01'),
('Task 7', 'Task 7 content', 4, '2023-07-01', '2023-07-01'),
('Task 8', 'Task 8 content', 5, '2023-08-01', '2023-08-01'),
('Task 9', 'Task 9 content', 6, '2023-09-01', '2023-09-01'),
('Task 10', 'Task 10 content', 7, '2023-10-01', '2023-10-01'),
('Task 11', 'Task 11 content', 8, '2023-11-01', '2023-11-01'),
('Task 12', 'Task 12 content', 9, '2023-12-01', '2023-12-01'),
('Task 13', 'Task 13 content', 10, '2024-01-01', '2024-01-01'),
('Task 14', 'Task 14 content', 1, '2024-02-01', '2024-02-01'),
('Task 15', 'Task 15 content', 2, '2024-03-01', '2024-03-01'),
('Task 16', 'Task 16 content', 3, '2024-04-01', '2024-04-01'),
('Task 17', 'Task 17 content', 4, '2024-05-01', '2024-05-01'),
('Task 18', 'Task 18 content', 5, '2024-06-01', '2024-06-01'),
('Task 19', 'Task 19 content', 6, '2024-07-01', '2024-07-01'),
('Task 20', 'Task 20 content', 7, '2024-08-01', '2024-08-01');

ALTER TABLE "task_table" ADD CONSTRAINT "user_id" FOREIGN KEY ("user_id") REFERENCES "user_table" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
