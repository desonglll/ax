CREATE OR REPLACE FUNCTION update_engagement_rate()
RETURNS TRIGGER AS $$
BEGIN
    IF (NEW.like_count + NEW.dislike_count) = 0 THEN
        NEW.engagement_rate := 0.0;
    ELSE
        -- 使用浮点数计算
        NEW.engagement_rate := NEW.like_count::FLOAT / (NEW.like_count + NEW.dislike_count)::FLOAT;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 在 posts 表上创建触发器，当 like_count 或 dislike_count 发生变化时，自动更新 engagement_rate
CREATE TRIGGER calculate_engagement_rate_trigger
BEFORE INSERT OR UPDATE ON posts
FOR EACH ROW
EXECUTE FUNCTION update_engagement_rate();
