-- Add migration script here
CREATE OR REPLACE FUNCTION update_post_reaction_counts()
RETURNS TRIGGER AS $$
BEGIN
    -- Update counts based on the reaction
    UPDATE posts
    SET like_count = (SELECT COUNT(*) FROM reactions WHERE to_id = posts.id AND reaction_name = 'Like' AND to_type = 'post'),
        dislike_count = (SELECT COUNT(*) FROM reactions WHERE to_id = posts.id AND reaction_name = 'Dislike' AND to_type = 'post')
    WHERE id = (CASE 
                  WHEN TG_OP = 'DELETE' THEN OLD.to_id 
                  ELSE NEW.to_id 
                END);

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER update_reaction_counts
AFTER INSERT OR UPDATE OR DELETE ON reactions
FOR EACH ROW
EXECUTE FUNCTION update_post_reaction_counts();
