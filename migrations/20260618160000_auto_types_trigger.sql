-- Add migration script here

-- Create or replace trigger function for comments.reply_to_type
CREATE OR REPLACE FUNCTION set_comment_reply_to_type()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM posts WHERE id = NEW.reply_to) THEN
        NEW.reply_to_type := 'post';
    ELSIF EXISTS (SELECT 1 FROM comments WHERE id = NEW.reply_to) THEN
        NEW.reply_to_type := 'comment';
    ELSE
        NEW.reply_to_type := 'post';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_set_comment_reply_to_type ON comments;
CREATE TRIGGER trg_set_comment_reply_to_type
BEFORE INSERT ON comments
FOR EACH ROW
EXECUTE FUNCTION set_comment_reply_to_type();


-- Create or replace trigger function for reactions.to_type
CREATE OR REPLACE FUNCTION set_reaction_to_type()
RETURNS TRIGGER AS $$
BEGIN
    IF EXISTS (SELECT 1 FROM posts WHERE id = NEW.to_id) THEN
        NEW.to_type := 'post';
    ELSIF EXISTS (SELECT 1 FROM comments WHERE id = NEW.to_id) THEN
        NEW.to_type := 'comment';
    ELSE
        NEW.to_type := 'post';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_set_reaction_to_type ON reactions;
CREATE TRIGGER trg_set_reaction_to_type
BEFORE INSERT ON reactions
FOR EACH ROW
EXECUTE FUNCTION set_reaction_to_type();
