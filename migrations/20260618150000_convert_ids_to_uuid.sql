-- Add migration script here

-- 1. Alter posts table:
ALTER TABLE posts DROP CONSTRAINT IF EXISTS posts_pkey CASCADE;

ALTER TABLE posts RENAME COLUMN id TO old_id;
ALTER TABLE posts RENAME COLUMN reply_to TO old_reply_to;

ALTER TABLE posts ADD COLUMN id UUID;
ALTER TABLE posts ADD COLUMN reply_to UUID;

UPDATE posts SET id = ('00000000-0000-0000-0000-' || lpad(to_hex(old_id), 12, '0'))::uuid;
UPDATE posts SET reply_to = ('00000000-0000-0000-0000-' || lpad(to_hex(old_reply_to), 12, '0'))::uuid WHERE old_reply_to IS NOT NULL;

ALTER TABLE posts ALTER COLUMN id SET NOT NULL;
ALTER TABLE posts ALTER COLUMN id SET DEFAULT gen_random_uuid();
ALTER TABLE posts ADD CONSTRAINT posts_pkey PRIMARY KEY (id);

ALTER TABLE posts DROP COLUMN old_id;
ALTER TABLE posts DROP COLUMN old_reply_to;


-- 2. Alter comments table:
ALTER TABLE comments DROP CONSTRAINT IF EXISTS comments_pkey CASCADE;

ALTER TABLE comments RENAME COLUMN id TO old_id;
ALTER TABLE comments RENAME COLUMN reply_to TO old_reply_to;

ALTER TABLE comments ADD COLUMN id UUID;
ALTER TABLE comments ADD COLUMN reply_to UUID;

UPDATE comments SET id = ('00000000-0000-0000-0000-' || lpad(to_hex(old_id), 12, '0'))::uuid;
UPDATE comments SET reply_to = ('00000000-0000-0000-0000-' || lpad(to_hex(old_reply_to), 12, '0'))::uuid;

ALTER TABLE comments ALTER COLUMN id SET NOT NULL;
ALTER TABLE comments ALTER COLUMN id SET DEFAULT gen_random_uuid();
ALTER TABLE comments ALTER COLUMN reply_to SET NOT NULL;
ALTER TABLE comments ADD CONSTRAINT comments_pkey PRIMARY KEY (id);

ALTER TABLE comments DROP COLUMN old_id;
ALTER TABLE comments DROP COLUMN old_reply_to;


-- 3. Alter reactions table:
ALTER TABLE reactions DROP CONSTRAINT IF EXISTS unique_user_post_reaction CASCADE;

ALTER TABLE reactions RENAME COLUMN to_id TO old_to_id;

ALTER TABLE reactions ADD COLUMN to_id UUID;

UPDATE reactions SET to_id = ('00000000-0000-0000-0000-' || lpad(to_hex(old_to_id), 12, '0'))::uuid;

ALTER TABLE reactions ALTER COLUMN to_id SET NOT NULL;

ALTER TABLE reactions DROP COLUMN old_to_id;

ALTER TABLE reactions ADD CONSTRAINT unique_user_post_reaction UNIQUE (user_id, to_id, reaction_name, to_type);
