-- Your SQL goes here
-- Your SQL goes here
CREATE OR REPLACE FUNCTION set_comment_reactions_name()
RETURNS TRIGGER AS $$
BEGIN
    -- 查找 post_actions 表中对应 reaction_id 的 name
    SELECT "name" INTO NEW.reaction_name FROM "post_actions" WHERE "id" = NEW.reaction_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION set_comment_reactions_id()
RETURNS TRIGGER AS $$
BEGIN
    -- 查找 post_actions 表中对应 reaction_id 的 name
    SELECT "id" INTO NEW.reaction_id FROM "post_actions" WHERE "name" = NEW.reaction_name;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 创建插入触发器
CREATE TRIGGER set_comment_reactions_name_before_insert_reaction
BEFORE INSERT ON public.comment_reactions
FOR EACH ROW
EXECUTE FUNCTION set_comment_reactions_name();

-- 创建更新触发器
CREATE TRIGGER set_comment_reactions_name_before_update_reaction
BEFORE UPDATE ON public.comment_reactions
FOR EACH ROW
EXECUTE FUNCTION set_comment_reactions_name();

-- 创建插入触发器
CREATE TRIGGER set_comment_reactions_id_before_insert_reaction
BEFORE INSERT ON public.comment_reactions
FOR EACH ROW
EXECUTE FUNCTION set_comment_reactions_id();

-- 创建更新触发器
CREATE TRIGGER set_comment_reactions_id_before_update_reaction
BEFORE UPDATE ON public.comment_reactions
FOR EACH ROW
EXECUTE FUNCTION set_comment_reactions_id();