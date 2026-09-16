# Database Schema & Triggers

PostgreSQL 16. Schema changes are SQLx migrations in `migrations/`, applied
automatically when the backend starts. The tables below reflect the schema
after all migrations.

## Tables

### `users`
| Column | Type | Notes |
|--------|------|-------|
| `id` | SERIAL PK | |
| `user_name` | VARCHAR UNIQUE | login name |
| `email` | VARCHAR UNIQUE | stored lowercase |
| `password_hash` | VARCHAR | bcrypt |
| `full_name`, `phone` | VARCHAR NULL | |
| `created_at`, `updated_at` | TIMESTAMPTZ NOT NULL | |
| `last_login` | TIMESTAMPTZ NULL | updated on login |
| `is_active` | BOOLEAN | deactivated users cannot log in |
| `is_admin` | BOOLEAN | |
| `profile_picture` | UUID NULL → `files.id` | ON DELETE SET NULL |

### `posts`
| Column | Type | Notes |
|--------|------|-------|
| `id` | UUID PK | `gen_random_uuid()` |
| `title` | VARCHAR NOT NULL DEFAULT '' | empty = "needs AI title" |
| `content` | TEXT | |
| `user_id` | INTEGER → `users.id` | ON DELETE CASCADE |
| `user_name` | VARCHAR | cached from `users` by trigger |
| `reply_to` | UUID NULL → `posts.id` | unused by the UI |
| `like_count`, `dislike_count` | INTEGER NOT NULL DEFAULT 0 | maintained by trigger |
| `engagement_rate` | FLOAT NOT NULL | `likes / (likes + dislikes)`, trigger |
| `created_at`, `updated_at` | TIMESTAMPTZ NOT NULL | |

Indexes: `user_id`, `reply_to`, `created_at DESC`, trigram GIN on `content`
(for `ILIKE` search).

### `comments`
| Column | Type | Notes |
|--------|------|-------|
| `id` | UUID PK | |
| `content` | TEXT | |
| `reply_to` | UUID NOT NULL | post (or comment) id |
| `reply_to_type` | VARCHAR CHECK IN ('post','comment') | set by trigger |
| `user_id` | INTEGER → `users.id` | ON DELETE CASCADE |
| `user_name` | VARCHAR | cached by trigger |
| `created_at`, `updated_at` | TIMESTAMPTZ NOT NULL | |

### `reactions`
| Column | Type | Notes |
|--------|------|-------|
| `id` | SERIAL PK | |
| `user_id` | INTEGER → `users.id` | ON DELETE CASCADE |
| `to_id` | UUID NOT NULL | post or comment id |
| `to_type` | VARCHAR CHECK IN ('post','comment') | set by trigger |
| `reaction_name` | VARCHAR CHECK IN ('Like','Dislike') | |
| `created_at` | TIMESTAMPTZ | |

`UNIQUE (user_id, to_id, to_type)` — one reaction per user per target; the
API upserts on that key to flip Like ↔ Dislike.

### `follows`
`(follower_id, followee_id)` PK, both → `users.id` ON DELETE CASCADE,
`CHECK (follower_id <> followee_id)`, index on `followee_id`.

### `notifications`
| Column | Type | Notes |
|--------|------|-------|
| `id` | BIGINT identity PK | |
| `user_id` | INTEGER → `users.id` | recipient |
| `actor_id` | INTEGER → `users.id` | who caused it |
| `kind` | VARCHAR CHECK IN ('follow','comment','reaction') | |
| `post_id`, `comment_id` | UUID NULL | ON DELETE CASCADE |
| `is_read` | BOOLEAN DEFAULT false | |
| `created_at` | TIMESTAMPTZ | |

Indexes: `(user_id, created_at DESC)` and a partial one on unread rows.

### `files`
| Column | Type | Notes |
|--------|------|-------|
| `id` | UUID PK | also the on-disk file name |
| `name` | VARCHAR | original file name |
| `path` | VARCHAR | absolute path under `UPLOAD_DIR` |
| `size` | BIGINT | bytes |
| `content_type` | VARCHAR | |
| `user_id` | INTEGER → `users.id` | uploader, ON DELETE CASCADE |
| `description` | TEXT NULL | |
| `checksum` | VARCHAR | SHA-256 hex, indexed |
| `is_deleted` | BOOLEAN | soft delete (set when re-uploading identical content) |
| `is_pub` | BOOLEAN | private files are visible to owner and admins only |
| `post_id` | UUID NULL → `posts.id` | attachment link |
| `comment_id` | UUID NULL → `comments.id` | attachment link |
| `created_at`, `updated_at` | TIMESTAMPTZ NULL | |

### `user_stats`
Per-user aggregates kept by the `update_user_stats` trigger. Not read by the
application any more; retained for compatibility.

## Triggers

| Trigger | Table | What it does |
|---------|-------|--------------|
| `set_user_name` / `update_user_name` | posts | Copies `users.user_name` into `posts.user_name` |
| `set_comments_user_name` / `update_comments_user_name` | comments | Same for comments |
| `trg_set_comment_reply_to_type` | comments | Sets `reply_to_type` by looking up the target |
| `trg_set_reaction_to_type` | reactions | Sets `to_type` by looking up the target |
| `update_reaction_counts` | reactions | Incrementally adjusts `posts.like_count` / `dislike_count` (post targets only) |
| `calculate_engagement_rate_trigger` | posts | Recomputes `engagement_rate` when counts change |
| `update_user_stats_trigger` | posts | Refreshes `user_stats` for the author |
| `create_user_stats_trigger` | users | Creates the `user_stats` row for a new user |
| `notify_on_follow_trigger` | follows | Inserts a `follow` notification for the followee |
| `notify_on_comment_trigger` | comments | Inserts a `comment` notification for the post owner (not for self-comments) |
| `notify_on_reaction_trigger` | reactions | Inserts a `reaction` notification for the post owner (not for self-reactions) |

Because counters and notifications are produced by triggers, every write
path (API, scripts, future jobs) stays consistent without application code.

## Demo data

The migrations seed four users (password `070011`): `root` (admin),
`mike` (admin), `joe`, `otis`, plus a handful of posts.
