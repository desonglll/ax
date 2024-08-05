-- Your SQL goes here
-- up.sql
CREATE TABLE session_table (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES user_table(user_id) ON DELETE CASCADE,
    username VARCHAR(255) NOT NULL REFERENCES user_table(username) ON DELETE CASCADE,
    session_id UUID NOT NULL,
    expires TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 创建触发器函数
CREATE OR REPLACE FUNCTION fill_username()
RETURNS TRIGGER AS $$
BEGIN
    SELECT username INTO NEW.username FROM user_table WHERE user_id = NEW.user_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
-- 创建触发器
CREATE TRIGGER before_insert_session
BEFORE INSERT ON session_table
FOR EACH ROW
EXECUTE FUNCTION fill_username();

-- 插入会话数据
INSERT INTO session_table (user_id, session_id, expires) VALUES
(1, 'd290f1ee-6c54-4b01-90e6-d701748f0851', '2024-07-23 12:00:00'),
(2, 'c552f1ee-1c22-4b01-91e1-c702748f0912', '2024-07-23 12:00:00');