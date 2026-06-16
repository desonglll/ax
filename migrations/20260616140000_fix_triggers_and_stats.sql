-- Add migration script here

-- Fix update_post_reaction_counts function to only update posts table for reactions with to_type = 'post'
CREATE OR REPLACE FUNCTION update_post_reaction_counts()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'DELETE' AND OLD.to_type = 'post') OR (TG_OP != 'DELETE' AND NEW.to_type = 'post') THEN
        UPDATE posts
        SET like_count = (SELECT COUNT(*) FROM reactions WHERE to_id = posts.id AND reaction_name = 'Like' AND to_type = 'post'),
            dislike_count = (SELECT COUNT(*) FROM reactions WHERE to_id = posts.id AND reaction_name = 'Dislike' AND to_type = 'post')
        WHERE id = (CASE 
                      WHEN TG_OP = 'DELETE' THEN OLD.to_id 
                      ELSE NEW.to_id 
                    END);
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Fix update_user_stats function to calculate average_comment_count using comments table, and handle DELETE operations
CREATE OR REPLACE FUNCTION update_user_stats()
RETURNS TRIGGER AS $$
DECLARE
    uid INT;
BEGIN
    IF TG_OP = 'DELETE' THEN
        uid := OLD.user_id;
    ELSE
        uid := NEW.user_id;
    END IF;

    -- Update statistics in user_stats table
    UPDATE user_stats
    SET
        liked_posts_count = (SELECT COUNT(*) FROM posts WHERE user_id = uid AND like_count > 0),
        average_like_count = (SELECT COALESCE(AVG(like_count), 0) FROM posts WHERE user_id = uid),
        average_comment_count = (SELECT COALESCE(AVG(content_count), 0) FROM (
            SELECT COUNT(*) AS content_count FROM comments WHERE comments.user_id = uid GROUP BY reply_to
        ) subquery),
        recent_activity_score = (SELECT COUNT(*) FROM posts WHERE user_id = uid AND created_at >= NOW() - INTERVAL '7 days'),
        engagement_rate = COALESCE((
            SELECT AVG(like_count::float / NULLIF(like_count + dislike_count, 0))
            FROM posts WHERE user_id = uid
        ), 0)
    WHERE user_id = uid;

    IF TG_OP = 'DELETE' THEN
        RETURN OLD;
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- Drop existing trigger on posts
DROP TRIGGER IF EXISTS update_user_stats_trigger ON posts;

-- Recreate trigger on posts to fire on INSERT, UPDATE, and DELETE
CREATE TRIGGER update_user_stats_trigger
AFTER INSERT OR UPDATE OR DELETE ON posts
FOR EACH ROW
EXECUTE FUNCTION update_user_stats();

-- Recalculate stats for all users to fix any wrong values caused by the bugs
UPDATE user_stats
SET
    liked_posts_count = (SELECT COUNT(*) FROM posts WHERE posts.user_id = user_stats.user_id AND like_count > 0),
    average_like_count = (SELECT COALESCE(AVG(like_count), 0) FROM posts WHERE posts.user_id = user_stats.user_id),
    average_comment_count = (SELECT COALESCE(AVG(content_count), 0) FROM (
        SELECT COUNT(*) AS content_count FROM comments WHERE comments.user_id = user_stats.user_id GROUP BY reply_to
    ) subquery),
    recent_activity_score = (SELECT COUNT(*) FROM posts WHERE posts.user_id = user_stats.user_id AND created_at >= NOW() - INTERVAL '7 days'),
    engagement_rate = COALESCE((
        SELECT AVG(like_count::float / NULLIF(like_count + dislike_count, 0))
        FROM posts WHERE posts.user_id = user_stats.user_id
    ), 0);
