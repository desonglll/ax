-- Indexes for GET /api/search/posts and GET /api/search/users.
--
-- Posts match two ways, OR-ed together:
--   * full-text: `posts_search_document(title, content) @@ websearch_to_tsquery('simple', q)`
--     — word matches anywhere, any order, with `"phrases"`, `or` and `-exclusions`;
--     the title is weighted above the body for ranking.
--   * substring: `title ILIKE '%q%' OR content ILIKE '%q%'` — catches partial
--     words and CJK text, which the 'simple' parser does not split into words.
-- Both sides are served by GIN indexes (the trigram ones need a query of at
-- least three characters; shorter ones fall back to a scan, which is fine).
--
-- The 'simple' configuration is used rather than 'english' so non-English
-- posts are not mangled by an English stemmer.

CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- IMMUTABLE so it can be used in an index expression; queries must call it
-- with the same arguments for the planner to pick the index.
CREATE OR REPLACE FUNCTION posts_search_document(title text, content text)
RETURNS tsvector
LANGUAGE sql
IMMUTABLE PARALLEL SAFE
AS $$
    SELECT setweight(to_tsvector('simple'::regconfig, coalesce(title, '')), 'A')
        || setweight(to_tsvector('simple'::regconfig, coalesce(content, '')), 'B')
$$;

CREATE INDEX IF NOT EXISTS posts_search_document_idx
    ON posts USING gin (posts_search_document(title, content));
CREATE INDEX IF NOT EXISTS posts_title_trgm_idx
    ON posts USING gin (title gin_trgm_ops);
CREATE INDEX IF NOT EXISTS posts_content_trgm_idx
    ON posts USING gin (content gin_trgm_ops);

CREATE INDEX IF NOT EXISTS users_user_name_trgm_idx
    ON users USING gin (user_name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS users_full_name_trgm_idx
    ON users USING gin (full_name gin_trgm_ops);
