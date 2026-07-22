-- The demo users seeded in 20240914094338 carry bcrypt hashes that do not
-- match the password documented everywhere in the codebase ("070011"), so
-- logging in as root/mike/joe/otis has never worked. Reset them to the
-- documented password using pgcrypto's bcrypt (produces a $2a$ hash, which
-- the server's bcrypt verifier accepts).
CREATE EXTENSION IF NOT EXISTS pgcrypto;

UPDATE users
SET password_hash = crypt('070011', gen_salt('bf', 12))
WHERE user_name IN ('root', 'mike', 'joe', 'otis');
