-- 创建 user_stats 表
DROP TABLE IF EXISTS user_stats;
CREATE TABLE user_stats (
    user_id INT NOT NULL,
    liked_posts_count INT NOT NULL DEFAULT 0,
    average_like_count FLOAT NOT NULL DEFAULT 0.0,
    average_comment_count FLOAT NOT NULL DEFAULT 0.0,
    recent_activity_score FLOAT NOT NULL DEFAULT 0.0,
    engagement_rate FLOAT NOT NULL DEFAULT 0.0,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE OR REPLACE FUNCTION manual_update_user_stats(uid INT)
RETURNS VOID AS $$
BEGIN
    -- 更新 user_stats 表中的统计信息
    UPDATE user_stats
    SET
        liked_posts_count = (SELECT COUNT(*) FROM posts WHERE posts.user_id = uid AND like_count > 0),
        average_like_count = (SELECT COALESCE(AVG(like_count), 0) FROM posts WHERE posts.user_id = uid),
        average_comment_count = (SELECT COALESCE(AVG(content_count), 0) FROM (
            SELECT COUNT(*) AS content_count FROM comments WHERE comments.user_id = uid GROUP BY reply_to
        ) subquery),
        recent_activity_score = (SELECT COUNT(*) FROM posts WHERE posts.user_id = uid AND created_at >= NOW() - INTERVAL '7 days'),
        engagement_rate = COALESCE((
            SELECT AVG(like_count::float / NULLIF(like_count + dislike_count, 0))
            FROM posts WHERE posts.user_id = uid
        ), 0)  -- 确保 engagement_rate 不为 null
    WHERE user_stats.user_id = uid;
END;
$$ LANGUAGE plpgsql;


-- 初始化时，插入所有用户的统计信息
DO $$
DECLARE
    user_record RECORD;
BEGIN
    -- 循环每个用户并插入他们的初始统计信息
    FOR user_record IN SELECT DISTINCT u.id FROM users u LOOP
        INSERT INTO user_stats (user_id, liked_posts_count, average_like_count, average_comment_count, recent_activity_score, engagement_rate)
        VALUES (
            user_record.id,
            (SELECT COUNT(*) FROM posts WHERE posts.user_id = user_record.id AND like_count > 0),
            (SELECT COALESCE(AVG(like_count), 0) FROM posts WHERE posts.user_id = user_record.id),
            (SELECT COALESCE(AVG(content_count), 0) FROM (
                SELECT COUNT(*) AS content_count FROM comments WHERE comments.user_id = user_record.id GROUP BY reply_to
            ) subquery),
            (SELECT COUNT(*) FROM posts WHERE posts.user_id = user_record.id AND created_at >= NOW() - INTERVAL '7 days'),
            (SELECT COALESCE(AVG(like_count::float / NULLIF(like_count + dislike_count, 0)), 0) FROM posts WHERE posts.user_id = user_record.id)
        );
    END LOOP;
END;
$$;
