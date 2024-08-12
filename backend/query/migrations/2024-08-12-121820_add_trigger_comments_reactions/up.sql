-- Your SQL goes here
CREATE OR REPLACE FUNCTION comments_increase_reaction()
RETURNS TRIGGER AS $$
BEGIN
    -- Initialize reactions to '{}' if it is NULL
    UPDATE comments
    SET reactions = COALESCE(reactions, '{}'::jsonb)
    WHERE id = NEW.comment_id;


    -- 将 reactions 字段中的 reaction_name 数量加一
    UPDATE comments
    SET reactions = jsonb_set(
        reactions,
        ARRAY[NEW.reaction_name]::text[],  -- JSON 路径
        (COALESCE((reactions->NEW.reaction_name)::int, 0) + 1)::text::jsonb,
        true
    )
    WHERE id = NEW.comment_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER    comments_increase_reaction_trigger
AFTER INSERT ON comment_reactions
FOR EACH ROW
EXECUTE FUNCTION comments_increase_reaction();

---------------------------------------------------------------------------------------------

CREATE OR REPLACE FUNCTION comments_decrease_reaction()
RETURNS TRIGGER AS $$
BEGIN
    -- 确保 reactions 字段被初始化为 '{}' 如果它是 NULL
    UPDATE comments
    SET reactions = COALESCE(reactions, '{}'::jsonb)
    WHERE id = OLD.comment_id;

    -- 将 reactions 字段中的 reaction_name 数量减一
    UPDATE comments
    SET reactions = jsonb_set(
        reactions,
        ARRAY[OLD.reaction_name]::text[],  -- JSON 路径
        (GREATEST(COALESCE((reactions->OLD.reaction_name)::int, 0) - 1, 0))::text::jsonb,  -- 当前值 - 1，确保不小于0
        true
    )
    WHERE id = OLD.comment_id;

    RETURN OLD;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER    comments_decrease_reaction_trigger
AFTER DELETE ON comment_reactions
FOR EACH ROW
EXECUTE FUNCTION comments_decrease_reaction();