-- Toggling a like off and on again used to notify the author every time, and
-- an undone reaction left its notification behind. Keep at most one reaction
-- notification per (recipient, actor, post), and withdraw it while it is still
-- unread when the reaction is removed.
CREATE OR REPLACE FUNCTION notify_on_reaction()
RETURNS TRIGGER AS $$
DECLARE
    owner INT;
BEGIN
    IF NEW.to_type = 'post' THEN
        SELECT user_id INTO owner FROM posts WHERE id = NEW.to_id;
        IF owner IS NOT NULL AND owner <> NEW.user_id AND NOT EXISTS (
            SELECT 1 FROM notifications
            WHERE user_id = owner AND actor_id = NEW.user_id
              AND kind = 'reaction' AND post_id = NEW.to_id
        ) THEN
            INSERT INTO notifications (user_id, actor_id, kind, post_id)
            VALUES (owner, NEW.user_id, 'reaction', NEW.to_id);
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION withdraw_reaction_notification()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.to_type = 'post' THEN
        DELETE FROM notifications
        WHERE actor_id = OLD.user_id AND kind = 'reaction'
          AND post_id = OLD.to_id AND is_read = FALSE;
    END IF;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS withdraw_reaction_notification_trigger ON reactions;
CREATE TRIGGER withdraw_reaction_notification_trigger
AFTER DELETE ON reactions
FOR EACH ROW
EXECUTE FUNCTION withdraw_reaction_notification();
