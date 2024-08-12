CREATE OR REPLACE FUNCTION posts_increase_reaction()
RETURNS TRIGGER AS $$
BEGIN
    -- Initialize reactions to '{}' if it is NULL
    UPDATE posts
    SET reactions = COALESCE(reactions, '{}'::jsonb)
    WHERE id = NEW.post_id;


    -- 将 reactions 字段中的 reaction_name 数量加一
    UPDATE posts
    SET reactions = jsonb_set(
        reactions,
        ARRAY[NEW.reaction_name]::text[],  -- JSON 路径
        (COALESCE((reactions->NEW.reaction_name)::int, 0) + 1)::text::jsonb,
        true
    )
    WHERE id = NEW.post_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER    posts_increase_reaction_trigger
AFTER INSERT ON reactions
FOR EACH ROW
EXECUTE FUNCTION posts_increase_reaction();

---------------------------------------------------------------------------------------------

CREATE OR REPLACE FUNCTION posts_decrease_reaction()
RETURNS TRIGGER AS $$
BEGIN
    -- 确保 reactions 字段被初始化为 '{}' 如果它是 NULL
    UPDATE posts
    SET reactions = COALESCE(reactions, '{}'::jsonb)
    WHERE id = OLD.post_id;

    -- 将 reactions 字段中的 reaction_name 数量减一
    UPDATE posts
    SET reactions = jsonb_set(
        reactions,
        ARRAY[OLD.reaction_name]::text[],  -- JSON 路径
        (GREATEST(COALESCE((reactions->OLD.reaction_name)::int, 0) - 1, 0))::text::jsonb,  -- 当前值 - 1，确保不小于0
        true
    )
    WHERE id = OLD.post_id;

    RETURN OLD;
END;
$$ LANGUAGE plpgsql;


CREATE TRIGGER    posts_decrease_reaction_trigger
AFTER DELETE ON reactions
FOR EACH ROW
EXECUTE FUNCTION posts_decrease_reaction();