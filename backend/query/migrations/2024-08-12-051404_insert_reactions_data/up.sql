-- Your SQL goes here
-- 插入 reactions 表的示例数据
INSERT INTO public.reactions (user_id, post_id, created_at, reaction_id, reaction_name) VALUES
(1, 1, CURRENT_TIMESTAMP, 1, 'like'),    -- 用户 1 对帖子 1 进行 'like' 反应
(2, 1, CURRENT_TIMESTAMP, 2, 'love'),    -- 用户 2 对帖子 1 进行 'love' 反应
(3, 1, CURRENT_TIMESTAMP, 1, 'love'),    -- 用户 3 对帖子 1 进行 'like' 反应
(3, 2, CURRENT_TIMESTAMP, 3, 'wow'),     -- 用户 3 对帖子 2 进行 'wow' 反应
(1, 3, CURRENT_TIMESTAMP, 4, 'angry'),   -- 用户 1 对帖子 3 进行 'angry' 反应
(2, 3, CURRENT_TIMESTAMP, 5, 'sad');     -- 用户 2 对帖子 3 进行 'sad' 反应
