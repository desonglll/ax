-- Your SQL goes here
------------------------------------------------------------------------------------------------
-- 创建触发器函数
CREATE OR REPLACE FUNCTION fill_user_name()
RETURNS TRIGGER AS $$
BEGIN
    -- 查询 user_name 并赋值给 NEW.user_name
    SELECT "user_name" INTO NEW.user_name FROM "users" WHERE "id" = NEW.user_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器
CREATE TRIGGER set_comments_user_name
BEFORE INSERT ON public.comments
FOR EACH ROW
EXECUTE FUNCTION fill_user_name();

CREATE TRIGGER update_comments_user_name
BEFORE UPDATE ON public.comments
FOR EACH ROW
EXECUTE FUNCTION fill_user_name();
------------------------------------------------------------------------------------------------

