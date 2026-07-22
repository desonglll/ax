-- A user can have exactly one reaction state for a given target. The previous
-- constraint included reaction_name, which allowed Like and Dislike to coexist
-- and forced the API to issue a DELETE followed by an INSERT.

DELETE FROM reactions older
USING reactions newer
WHERE older.user_id = newer.user_id
  AND older.to_id = newer.to_id
  AND older.to_type = newer.to_type
  AND (older.created_at, older.id) < (newer.created_at, newer.id);

ALTER TABLE reactions
    DROP CONSTRAINT IF EXISTS unique_user_post_reaction;

ALTER TABLE reactions
    ADD CONSTRAINT reactions_name_check
        CHECK (reaction_name IN ('Like', 'Dislike')),
    ADD CONSTRAINT reactions_user_target_key
        UNIQUE (user_id, to_id, to_type);

DROP INDEX IF EXISTS idx_reactions_to_id;
CREATE INDEX idx_reactions_target
    ON reactions (to_type, to_id, reaction_name);

DROP INDEX IF EXISTS idx_notifications_user_unread;
CREATE INDEX idx_notifications_unread
    ON notifications (user_id, created_at DESC)
    WHERE is_read = FALSE;
