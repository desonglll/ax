-- Your SQL goes here
INSERT INTO public.posts ("content", "created_at", "updated_at", "user_id", "reply_to") VALUES
('Content of post 1', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Mike Content of post 2', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 2, NULL),
('Mike Content of post 3', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 2, NULL),
('Content of post 4', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 5', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 6', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 7', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 8', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 9', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL),
('Content of post 10', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, 1, NULL);
