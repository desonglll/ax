-- Saved posts. One row per (user, post); deleting either side removes it.
CREATE TABLE bookmarks (
    user_id INTEGER NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    post_id UUID NOT NULL REFERENCES posts (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, post_id)
);

-- "My saved posts, newest saved first" and the cascade from posts.
CREATE INDEX idx_bookmarks_user_created ON bookmarks (user_id, created_at DESC);
CREATE INDEX idx_bookmarks_post ON bookmarks (post_id);
