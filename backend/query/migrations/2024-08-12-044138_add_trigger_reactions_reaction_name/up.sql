-- Your SQL goes here
CREATE OR REPLACE FUNCTION set_reaction_name()
RETURNS TRIGGER AS $$
BEGIN
    -- 查找 post_actions 表中对应 reaction_id 的 name
    SELECT "name" INTO NEW.reaction_name FROM "post_actions" WHERE "id" = NEW.reaction_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
-- 创建插入触发器
CREATE TRIGGER before_insert_reaction
BEFORE INSERT ON public.reactions
FOR EACH ROW
EXECUTE FUNCTION set_reaction_name();

-- 创建更新触发器
CREATE TRIGGER before_update_reaction
BEFORE UPDATE ON public.reactions
FOR EACH ROW
EXECUTE FUNCTION set_reaction_name();