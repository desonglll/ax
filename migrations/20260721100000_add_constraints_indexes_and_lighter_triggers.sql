-- Schema hardening and performance pass.
--
-- 1. Clean orphaned rows, then add the missing foreign keys with explicit
--    ON DELETE behavior (previously only user_stats/files had any FK at all).
-- 2. Add the missing indexes: before this migration the schema had zero
--    CREATE INDEX statements, so every trigger and list query was a
--    sequential scan.
-- 3. Tighten column definitions (NOT NULL counters/timestamps, CHECK on the
--    polymorphic type columns, UNIQUE user_stats.user_id).
-- 4. Make the hot-path triggers cheap: reaction counts become incremental
--    instead of a full COUNT(*) per reaction, and the user_stats /
--    engagement_rate triggers only fire when the columns they depend on
--    actually change.

------------------------------------------------------------------------------
-- 1a. Remove rows that would violate the new foreign keys.
------------------------------------------------------------------------------
DELETE FROM reactions r WHERE NOT EXISTS (SELECT 1 FROM users u WHERE u.id = r.user_id);
DELETE FROM comments c WHERE NOT EXISTS (SELECT 1 FROM users u WHERE u.id = c.user_id);
DELETE FROM files f WHERE NOT EXISTS (SELECT 1 FROM users u WHERE u.id = f.user_id);
DELETE FROM posts p WHERE NOT EXISTS (SELECT 1 FROM users u WHERE u.id = p.user_id);
UPDATE posts SET reply_to = NULL
WHERE reply_to IS NOT NULL
  AND NOT EXISTS (SELECT 1 FROM posts p2 WHERE p2.id = posts.reply_to);
UPDATE users SET profile_picture = NULL
WHERE profile_picture IS NOT NULL
  AND NOT EXISTS (SELECT 1 FROM files f WHERE f.id = users.profile_picture);

-- user_stats: drop duplicates, keep one row per user.
DELETE FROM user_stats a USING user_stats b
WHERE a.ctid < b.ctid AND a.user_id = b.user_id;

------------------------------------------------------------------------------
-- 1b. Foreign keys.
------------------------------------------------------------------------------
ALTER TABLE posts
    ADD CONSTRAINT posts_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) ON DELETE CASCADE,
    ADD CONSTRAINT posts_reply_to_fkey FOREIGN KEY (reply_to)
        REFERENCES posts (id) ON DELETE SET NULL;

ALTER TABLE comments
    ADD CONSTRAINT comments_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE reactions
    ADD CONSTRAINT reactions_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE files
    ADD CONSTRAINT files_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE users
    ADD CONSTRAINT users_profile_picture_fkey FOREIGN KEY (profile_picture)
        REFERENCES files (id) ON DELETE SET NULL;

-- user_stats: one row per user, and stats should disappear with the user.
ALTER TABLE user_stats
    ADD CONSTRAINT user_stats_user_id_key UNIQUE (user_id);
DO $$
DECLARE
    fk_name TEXT;
BEGIN
    SELECT conname INTO fk_name
    FROM pg_constraint
    WHERE conrelid = 'user_stats'::regclass AND contype = 'f';
    IF fk_name IS NOT NULL THEN
        EXECUTE format('ALTER TABLE user_stats DROP CONSTRAINT %I', fk_name);
    END IF;
END $$;
ALTER TABLE user_stats
    ADD CONSTRAINT user_stats_user_id_fkey FOREIGN KEY (user_id)
        REFERENCES users (id) ON DELETE CASCADE;

------------------------------------------------------------------------------
-- 2. Indexes for every FK column and hot filter.
------------------------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_posts_user_id ON posts (user_id);
CREATE INDEX IF NOT EXISTS idx_posts_reply_to ON posts (reply_to) WHERE reply_to IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_posts_created_at ON posts (created_at DESC);
CREATE INDEX IF NOT EXISTS idx_comments_user_id ON comments (user_id);
CREATE INDEX IF NOT EXISTS idx_comments_reply_to ON comments (reply_to, reply_to_type);
CREATE INDEX IF NOT EXISTS idx_reactions_to_id ON reactions (to_id, to_type, reaction_name);
CREATE INDEX IF NOT EXISTS idx_files_user_id ON files (user_id);
CREATE INDEX IF NOT EXISTS idx_files_post_id ON files (post_id) WHERE post_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_files_comment_id ON files (comment_id) WHERE comment_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_files_checksum ON files (checksum);

-- Trigram index so `content ILIKE '%kw%'` search stops being a full scan.
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX IF NOT EXISTS idx_posts_content_trgm ON posts USING gin (content gin_trgm_ops);

------------------------------------------------------------------------------
-- 3. Column tightening.
------------------------------------------------------------------------------
UPDATE posts SET like_count = 0 WHERE like_count IS NULL;
UPDATE posts SET dislike_count = 0 WHERE dislike_count IS NULL;
ALTER TABLE posts
    ALTER COLUMN like_count SET DEFAULT 0,
    ALTER COLUMN like_count SET NOT NULL,
    ALTER COLUMN dislike_count SET DEFAULT 0,
    ALTER COLUMN dislike_count SET NOT NULL;

UPDATE users SET created_at = NOW() WHERE created_at IS NULL;
UPDATE users SET updated_at = NOW() WHERE updated_at IS NULL;
ALTER TABLE users
    ALTER COLUMN created_at SET NOT NULL,
    ALTER COLUMN updated_at SET NOT NULL;

-- Polymorphic discriminators: normalize then constrain to known values.
UPDATE reactions SET to_type = 'post' WHERE to_type IS NULL OR to_type NOT IN ('post', 'comment');
ALTER TABLE reactions
    ALTER COLUMN to_type SET NOT NULL,
    ADD CONSTRAINT reactions_to_type_check CHECK (to_type IN ('post', 'comment'));

UPDATE comments SET reply_to_type = 'post' WHERE reply_to_type IS NULL OR reply_to_type NOT IN ('post', 'comment');
ALTER TABLE comments
    ALTER COLUMN reply_to_type SET NOT NULL,
    ADD CONSTRAINT comments_reply_to_type_check CHECK (reply_to_type IN ('post', 'comment'));

------------------------------------------------------------------------------
-- 4a. Incremental reaction counting: O(1) per reaction instead of two
--     COUNT(*) scans over reactions.
------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION update_post_reaction_counts()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP IN ('DELETE', 'UPDATE') AND OLD.to_type = 'post' THEN
        UPDATE posts
        SET like_count = GREATEST(like_count - (OLD.reaction_name = 'Like')::int, 0),
            dislike_count = GREATEST(dislike_count - (OLD.reaction_name = 'Dislike')::int, 0)
        WHERE id = OLD.to_id;
    END IF;

    IF TG_OP IN ('INSERT', 'UPDATE') AND NEW.to_type = 'post' THEN
        UPDATE posts
        SET like_count = like_count + (NEW.reaction_name = 'Like')::int,
            dislike_count = dislike_count + (NEW.reaction_name = 'Dislike')::int
        WHERE id = NEW.to_id;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Re-sync the counters once, since older buggy trigger versions may have
-- left stale values behind.
UPDATE posts
SET like_count = sub.likes,
    dislike_count = sub.dislikes
FROM (
    SELECT p.id,
           COUNT(*) FILTER (WHERE r.reaction_name = 'Like') AS likes,
           COUNT(*) FILTER (WHERE r.reaction_name = 'Dislike') AS dislikes
    FROM posts p
    LEFT JOIN reactions r ON r.to_id = p.id AND r.to_type = 'post'
    GROUP BY p.id
) sub
WHERE posts.id = sub.id
  AND (posts.like_count IS DISTINCT FROM sub.likes
       OR posts.dislike_count IS DISTINCT FROM sub.dislikes);

------------------------------------------------------------------------------
-- 4b. Only recompute user_stats when stats inputs change, not on every
--     content edit; same for engagement_rate.
------------------------------------------------------------------------------
-- user_stats rows were only ever created by a one-off backfill in
-- 20240914094460, so users registered after that never get a row and the
-- recommendation features query comes back empty. Backfill the gap and keep
-- it closed with an AFTER INSERT trigger on users.
INSERT INTO user_stats (user_id, liked_posts_count, average_like_count,
                        average_comment_count, recent_activity_score, engagement_rate)
SELECT u.id, 0, 0, 0, 0, 0
FROM users u
WHERE NOT EXISTS (SELECT 1 FROM user_stats s WHERE s.user_id = u.id);

CREATE OR REPLACE FUNCTION create_user_stats_row()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO user_stats (user_id, liked_posts_count, average_like_count,
                            average_comment_count, recent_activity_score, engagement_rate)
    VALUES (NEW.id, 0, 0, 0, 0, 0)
    ON CONFLICT (user_id) DO NOTHING;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS create_user_stats_trigger ON users;
CREATE TRIGGER create_user_stats_trigger
AFTER INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION create_user_stats_row();

DROP TRIGGER IF EXISTS update_user_stats_trigger ON posts;
CREATE TRIGGER update_user_stats_trigger
AFTER INSERT OR DELETE OR UPDATE OF like_count, dislike_count ON posts
FOR EACH ROW
EXECUTE FUNCTION update_user_stats();

DROP TRIGGER IF EXISTS calculate_engagement_rate_trigger ON posts;
CREATE TRIGGER calculate_engagement_rate_trigger
BEFORE INSERT OR UPDATE OF like_count, dislike_count ON posts
FOR EACH ROW
EXECUTE FUNCTION update_engagement_rate();
