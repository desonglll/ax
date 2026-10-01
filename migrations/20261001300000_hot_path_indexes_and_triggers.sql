-- Performance pass driven by EXPLAIN ANALYZE on a seeded database
-- (2k users, 50k posts, 100k comments, 180k reactions, 320k notifications).
--
-- 1. Notifications had no index on post_id / comment_id / actor_id, so
--    un-liking a post (withdraw trigger), the reaction de-dup check, and the
--    ON DELETE CASCADE from posts, comments and users all scanned the whole
--    table (~110-180 ms each).
-- 2. Follower / following lists sort by created_at; index that order.
-- 3. user_stats was recomputed from scratch (five aggregates over all of the
--    author's posts and comments) on every like, dislike, post insert and
--    post delete, yet nothing reads it. It becomes a view with the same
--    columns, computed only when queried.
-- 4. posts/comments re-looked-up users.user_name on *every* UPDATE (including
--    each like-count change). Restrict that to user_id changes, and instead
--    propagate renames from users, which previously left the cached
--    user_name on old posts and comments stale.

------------------------------------------------------------------------------
-- 1. Notifications.
------------------------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_notifications_post_actor
    ON notifications (post_id, actor_id) WHERE post_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_notifications_comment
    ON notifications (comment_id) WHERE comment_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_notifications_actor
    ON notifications (actor_id);

------------------------------------------------------------------------------
-- 2. Follows. (follower_id, followee_id) is the primary key; the old
--    followee-only index is superseded by the composite one.
------------------------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_follows_followee_created
    ON follows (followee_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_follows_follower_created
    ON follows (follower_id, created_at DESC);
DROP INDEX IF EXISTS idx_follows_followee;

------------------------------------------------------------------------------
-- 3. user_stats: table + per-row triggers -> on-demand view.
------------------------------------------------------------------------------
DROP TRIGGER IF EXISTS update_user_stats_trigger ON posts;
DROP TRIGGER IF EXISTS create_user_stats_trigger ON users;
DROP FUNCTION IF EXISTS update_user_stats();
DROP FUNCTION IF EXISTS create_user_stats_row();
DROP FUNCTION IF EXISTS manual_update_user_stats(INT);
DROP TABLE IF EXISTS user_stats;

CREATE VIEW user_stats AS
SELECT
    u.id AS user_id,
    (SELECT COUNT(*) FROM posts p WHERE p.user_id = u.id AND p.like_count > 0)::INT
        AS liked_posts_count,
    (SELECT COALESCE(AVG(p.like_count), 0) FROM posts p WHERE p.user_id = u.id)::FLOAT
        AS average_like_count,
    (SELECT COALESCE(AVG(n), 0) FROM (
        SELECT COUNT(*) AS n FROM comments c WHERE c.user_id = u.id GROUP BY c.reply_to
    ) per_target)::FLOAT
        AS average_comment_count,
    (SELECT COUNT(*) FROM posts p
     WHERE p.user_id = u.id AND p.created_at >= NOW() - INTERVAL '7 days')::FLOAT
        AS recent_activity_score,
    COALESCE((SELECT AVG(p.like_count::FLOAT / NULLIF(p.like_count + p.dislike_count, 0))
              FROM posts p WHERE p.user_id = u.id), 0)::FLOAT
        AS engagement_rate
FROM users u;

------------------------------------------------------------------------------
-- 4. Cached user_name.
------------------------------------------------------------------------------
DROP TRIGGER IF EXISTS update_user_name ON posts;
CREATE TRIGGER update_user_name
    BEFORE UPDATE OF user_id ON posts
    FOR EACH ROW EXECUTE FUNCTION fill_user_name();

DROP TRIGGER IF EXISTS update_comments_user_name ON comments;
CREATE TRIGGER update_comments_user_name
    BEFORE UPDATE OF user_id ON comments
    FOR EACH ROW EXECUTE FUNCTION fill_user_name();

CREATE OR REPLACE FUNCTION propagate_user_name()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE posts SET user_name = NEW.user_name WHERE user_id = NEW.id;
    UPDATE comments SET user_name = NEW.user_name WHERE user_id = NEW.id;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER propagate_user_name_trigger
    AFTER UPDATE OF user_name ON users
    FOR EACH ROW
    WHEN (OLD.user_name IS DISTINCT FROM NEW.user_name)
    EXECUTE FUNCTION propagate_user_name();

-- Bring any names that went stale before this migration back in sync.
UPDATE posts p SET user_name = u.user_name
FROM users u WHERE u.id = p.user_id AND p.user_name IS DISTINCT FROM u.user_name;
UPDATE comments c SET user_name = u.user_name
FROM users u WHERE u.id = c.user_id AND c.user_name IS DISTINCT FROM u.user_name;
