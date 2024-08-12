-- 删除现有的 post_actions 表（如果存在）
DROP TABLE IF EXISTS public.post_actions;

-- 创建 post_actions 表
CREATE TABLE public.post_actions (
  id SERIAL NOT NULL UNIQUE,              -- 主键，自增长
  name VARCHAR(255) NOT NULL UNIQUE,            -- 操作名称，指定长度
  PRIMARY KEY (id, name)                       -- 设置 id 为主键
);

-- 插入示例数据
INSERT INTO public.post_actions (name) VALUES
('like'),    -- 添加 like 操作
('dislike'), -- 添加 dislike 操作
('love'),    -- 添加 love 操作
('wow'),     -- 添加 wow 操作
('angry'),   -- 添加 angry 操作
('sad');     -- 添加 sad 操作
