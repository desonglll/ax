-- Add migration script here
-- 创建触发器函数，用于更新 user_stats 表
CREATE OR REPLACE FUNCTION update_user_stats()
RETURNS TRIGGER AS $$
BEGIN
    -- 更新 user_stats 表中的统计信息
    UPDATE user_stats
    SET
        liked_posts_count = (SELECT COUNT(*) FROM posts WHERE user_id = NEW.user_id AND like_count > 0),
        average_like_count = (SELECT AVG(like_count) FROM posts WHERE user_id = NEW.user_id),
        average_comment_count = (SELECT AVG(dislike_count) FROM posts WHERE user_id = NEW.user_id),
        recent_activity_score = (SELECT COUNT(*) FROM posts WHERE user_id = NEW.user_id AND created_at >= NOW() - INTERVAL '7 days'),
        engagement_rate = COALESCE((
            SELECT AVG(like_count::float / NULLIF(like_count + dislike_count, 0))
            FROM posts WHERE user_id = NEW.user_id
        ), 0) -- 确保 engagement_rate 不为 null
    WHERE user_id = NEW.user_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;


-- 创建触发器，当在 posts 表中插入或更新时调用
CREATE TRIGGER update_user_stats_trigger
AFTER INSERT OR UPDATE ON posts
FOR EACH ROW
EXECUTE FUNCTION update_user_stats();
-- 初始化时更新所有用户的统计信息
