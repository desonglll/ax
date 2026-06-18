# Database Schema & Triggers

This chapter details the relational database design, table structures, constraints, and trigger functions that maintain Project Ax's data.

## Relational Tables

### 1. `users`
Stores user profile information and authentication credentials.
- `id` (SERIAL, Primary Key): Unique identifier.
- `user_name` (VARCHAR, Unique): Login identifier.
- `email` (VARCHAR, Unique): User email address.
- `password_hash` (VARCHAR): Bcrypt-encoded password hash.
- `full_name` (VARCHAR): User real name.
- `phone` (VARCHAR): Phone number.
- `created_at` (TIMESTAMPTZ): Account creation time.
- `updated_at` (TIMESTAMPTZ): Profile update time.
- `last_login` (TIMESTAMPTZ): User last login time.
- `is_active` (BOOLEAN): Status flag.
- `is_admin` (BOOLEAN): Administrator flag.
- `profile_picture` (UUID): Reference to the uploaded profile picture.

### 2. `posts`
Stores the micro-blogging messages.
- `id` (SERIAL, Primary Key): Unique identifier.
- `content` (TEXT): Text content of the post.
- `created_at`, `updated_at` (TIMESTAMPTZ)
- `user_id` (INTEGER): Creator's ID.
- `reply_to` (INTEGER): Target post ID if replying.
- `user_name` (VARCHAR): Creator's username (cached).
- `like_count` (INTEGER): Current like reaction count.
- `dislike_count` (INTEGER): Current dislike reaction count.
- `engagement_rate` (FLOAT): Like/Dislike engagement ratio.

### 3. `comments`
Stores replies to posts and comments.
- `id` (SERIAL, Primary Key)
- `content` (TEXT)
- `reply_to` (INTEGER): Target ID.
- `reply_to_type` (VARCHAR): "post" or "comment".
- `user_id` (INTEGER)
- `user_name` (VARCHAR)
- `created_at`, `updated_at` (TIMESTAMPTZ)

### 4. `reactions`
Stores user reactions (likes/dislikes) for posts or comments.
- `id` (SERIAL, Primary Key)
- `user_id` (INTEGER)
- `to_id` (INTEGER): Target post or comment ID.
- `to_type` (VARCHAR): "post" or "comment".
- `reaction_name` (VARCHAR): "Like" or "Dislike".
- `created_at` (TIMESTAMPTZ)
- Constraint: `unique_user_post_reaction UNIQUE (user_id, to_id, reaction_name, to_type)`.

### 5. `user_stats`
Stores aggregated statistics for user activity.
- `user_id` (INTEGER, Foreign Key referencing `users(id)`)
- `liked_posts_count` (INTEGER)
- `average_like_count` (FLOAT)
- `average_comment_count` (FLOAT)
- `recent_activity_score` (FLOAT)
- `engagement_rate` (FLOAT)

### 6. `files`
Stores file metadata for user uploads and post attachments.
- `id` (UUID, Primary Key): Unique identifier.
- `name` (VARCHAR): Original filename.
- `path` (VARCHAR): Storage path on disk.
- `size` (BIGINT): File size in bytes.
- `content_type` (VARCHAR): MIME content type.
- `created_at`, `updated_at` (TIMESTAMPTZ)
- `user_id` (INTEGER): Uploader's user ID.
- `description` (TEXT): Optional description.
- `checksum` (VARCHAR): SHA-256 hash.
- `is_deleted` (BOOLEAN): Soft delete flag.
- `is_pub` (BOOLEAN): Publicly readable flag.
- `post_id` (UUID, Foreign Key referencing `posts(id)`): Attached post ID (optional).

---

## Triggers and Stored Procedures

To ensure data consistency and reduce runtime computational overhead, several PostgreSQL triggers are established:

### 1. Reaction Count Updates (`update_post_reaction_counts`)
Runs `AFTER INSERT OR UPDATE OR DELETE ON reactions`.
- It dynamically updates the `like_count` and `dislike_count` fields of a post when reactions targeting that post are modified.
- **Bug Fix**: It has been restricted to only run when `to_type = 'post'` to prevent comment reactions from corrupting post counts.

### 2. User Stats Real-time Updates (`update_user_stats`)
Runs `AFTER INSERT OR UPDATE OR DELETE ON posts`.
- It recalculates `liked_posts_count`, `average_like_count`, `average_comment_count`, `recent_activity_score`, and `engagement_rate` for the author whenever a post is created, updated, or deleted.
- **Bug Fix**: Comment counts are aggregated using the `comments` table instead of using `dislike_count`.
- **Bug Fix**: `DELETE` operations are fully supported and safely recalibrate user statistics.
