-- Follow graph. The original schema had followers_count/following_count
-- columns on user_stats (dropped again in 20240914094460) but never a table
-- to back them; this adds the actual relationship.
CREATE TABLE follows (
    follower_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    followee_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (follower_id, followee_id),
    CHECK (follower_id <> followee_id)
);

-- The PK covers follower-side lookups ("who do I follow"); this covers the
-- reverse direction ("who follows me").
CREATE INDEX idx_follows_followee ON follows (followee_id);
