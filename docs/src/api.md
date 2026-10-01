# API Reference

All endpoints live under `/api`. Requests and responses are JSON with
camelCase keys; uploads use `multipart/form-data`. Authentication is a
session cookie set by `POST /api/auth/login`.

## Envelope

```json
{ "code": 200, "message": "OK", "body": { "data": …, "pagination": { "limit": 10, "offset": 0, "count": 42 } } }
```

`pagination` is present on list endpoints. Errors use the matching HTTP
status and the same shape with `"body": null`:

| Status | Meaning                                                    |
|--------|------------------------------------------------------------|
| 400    | Invalid input (validation, malformed JSON, duplicate value) |
| 401    | Not signed in                                              |
| 403    | Signed in but not allowed (not owner / not admin)          |
| 404    | Resource not found                                         |
| 413    | Upload larger than `MAX_UPLOAD_MB`                         |
| 429    | Rate limit exceeded on a write request (plain-text body, `Retry-After` header) |
| 500    | Unexpected server error                                    |
| 503    | Database unavailable / connection pool exhausted — retry   |

Every response carries an `X-Request-Id` (an incoming one of up to 64
`[A-Za-z0-9._-]` characters is kept, so ids can be traced through a proxy;
it is also in the access log), plus `X-Content-Type-Options: nosniff`,
`X-Frame-Options: DENY`, `Referrer-Policy` and a deny-all
`Content-Security-Policy`. JSON responses are compressed (gzip, brotli or
zstd) when the client accepts it; file bodies never are.

List endpoints accept `limit` (1–100) and `offset` (≥ 0).

## Auth

| Method | Path                | Auth | Body / Query                          | Returns |
|--------|---------------------|------|---------------------------------------|---------|
| POST   | `/auth/login`       | –    | `{ userName, password }`              | `User`  |
| POST   | `/auth/logout`      | –    |                                       | message |
| GET    | `/auth/me`          | ✓    |                                       | `User` (fresh profile) |

Deactivated accounts receive 403 on login.

## Users

| Method | Path                         | Auth        | Body / Query                                      | Returns |
|--------|------------------------------|-------------|---------------------------------------------------|---------|
| POST   | `/users`                     | –           | `{ userName, email, password, fullName?, phone? }` | `User` |
| GET    | `/users`                     | –           | `limit`, `offset`                                 | `User[]` |
| GET    | `/users/{id}`                | –           |                                                   | `User` |
| PUT    | `/users/{id}`                | self/admin  | any of `userName, email, password, fullName, phone, bio, profilePicture`; admins also `isActive, isAdmin` | `User` |
| DELETE | `/users/{id}`                | self/admin  |                                                   | `User` |
| POST   | `/users/{id}/follow`         | ✓           |                                                   | `FollowStats` |
| DELETE | `/users/{id}/follow`         | ✓           |                                                   | `FollowStats` |
| GET    | `/users/{id}/follow-stats`   | –           |                                                   | `FollowStats` (`isFollowing` reflects the caller) |
| GET    | `/users/{id}/followers`      | –           | `limit`, `offset`                                 | `User[]` |
| GET    | `/users/{id}/following`      | –           | `limit`, `offset`                                 | `User[]` |

Validation: `userName` 3–32 chars, `password` 8–128 chars, `email` must
look like an address, `fullName` ≤ 64 chars, `bio` ≤ 280 chars. An empty
`fullName` or `bio` clears it. `profilePicture` is a file id (the avatar) and
must be a public image uploaded by the profile's owner; `null` removes it,
leaving it out keeps the current one. `passwordHash` is never returned.

`User` fields: `id, userName, email, fullName, phone, bio, profilePicture,
createdAt, updatedAt, lastLogin, isActive, isAdmin`. `email`, `phone` and
`lastLogin` are only present when the viewer is that user or an admin
(sign-in, `/auth/me`, registration and profile edits always include them);
lists, profiles, follower lists and people search omit them for everyone
else.

## Posts

| Method | Path               | Auth        | Body / Query                                                                 | Returns |
|--------|--------------------|-------------|------------------------------------------------------------------------------|---------|
| GET    | `/posts`           | –           | `limit`, `offset`, `search`, `user_id`, `order_by` (`created_at` default, `updated_at`, `like_count`, `dislike_count`, `engagement_rate`), `sort` (`desc` default, `asc`) | `Post[]` |
| GET    | `/posts/feed`      | ✓           | `limit`, `offset`                                                            | `Post[]` from followed users |
| GET    | `/posts/trending`  | –           | `limit` (default 10)                                                         | `Post[]` ranked |
| POST   | `/posts`           | ✓           | `{ content, title?, attachments?: uuid[] }`                                  | `Post` |
| GET    | `/posts/{id}`      | –           |                                                                              | `Post` |
| PUT    | `/posts/{id}`      | owner/admin | `{ content?, title?, attachments? }` (attachments replaces the whole set)    | `Post` |
| DELETE | `/posts/{id}`      | owner/admin |                                                                              | `Post` |
| PUT    | `/posts/{id}/bookmark` | ✓       |                                                                              | `{ postId, bookmarked: true }` (idempotent; 404 for unknown posts) |
| DELETE | `/posts/{id}/bookmark` | ✓       |                                                                              | `{ postId, bookmarked: false }` (idempotent) |

`content` is Markdown. `search` matches title and content (case-insensitive). A post created
without a title gets an AI-generated one later when `OPENAI_API_KEY` is set.

`Post` fields: `id, title, content, createdAt, updatedAt, userId, userName,
likeCount, dislikeCount, engagementRate, attachments: File[], commentCount,
viewerReaction: "Like" | "Dislike" | null, viewerBookmarked, authorAvatar`.
`viewerBookmarked` is always `false` for guests; `authorAvatar` is the
author's `profilePicture` (or `null`).

## Bookmarks

| Method | Path          | Auth | Query             | Returns |
|--------|---------------|------|-------------------|---------|
| GET    | `/bookmarks`  | ✓    | `limit`, `offset` | `Post[]` the caller saved, most recently saved first |

Save and unsave with `PUT` / `DELETE /posts/{id}/bookmark` (above).

## Search

| Method | Path             | Auth | Query                           | Returns |
|--------|------------------|------|---------------------------------|---------|
| GET    | `/search/posts`  | –    | `q` (required), `limit`, `offset` | `Post[]` best match first |
| GET    | `/search/users`  | –    | `q` (required), `limit`, `offset` | `User[]` (active users only) |

`q` is 1–100 characters after trimming (whitespace runs collapse); a missing
or blank `q` is a 400. Posts match when their words match the query
(`websearch_to_tsquery` syntax: `"exact phrase"`, `or`, `-excluded`) **or**
their title/content contains the query literally, which also covers partial
words and Chinese text. Results are ranked by relevance (title hits first),
then newest, and are hydrated like every other post list. People match on
`userName` or `fullName` containing `q`: exact user name, then prefix
matches, then closest by similarity. `%`, `_` and `\` in `q` match
literally. `pagination.count` is the total number of matches.

## Comments

| Method | Path              | Auth        | Body / Query                                   | Returns |
|--------|-------------------|-------------|------------------------------------------------|---------|
| GET    | `/comments`       | –           | `replyTo` (required), `limit`, `offset`        | `Comment[]` oldest first |
| POST   | `/comments`       | ✓           | `{ content, replyTo, attachments?: uuid[] }`   | `Comment` |
| DELETE | `/comments/{id}`  | owner/admin |                                                | `Comment` |

`Comment` fields: `id, content, replyTo, userId, userName, createdAt,
updatedAt, attachments, likeCount, dislikeCount, viewerReaction, authorAvatar`.

## Reactions

A user has at most one reaction per target; setting a different one flips it.

| Method | Path          | Auth | Body / Query                                                  | Returns |
|--------|---------------|------|---------------------------------------------------------------|---------|
| PUT    | `/reactions`  | ✓    | `{ toId, toType: "post" \| "comment", reaction: "Like" \| "Dislike" }` | `Reaction` |
| DELETE | `/reactions`  | ✓    | query `toId`, `toType`                                        | `Reaction` or message |

## Notifications

Generated by database triggers for follows, comments on your posts and
reactions to your posts (never for your own actions).

| Method | Path                            | Auth | Query             | Returns |
|--------|---------------------------------|------|-------------------|---------|
| GET    | `/notifications`                | ✓    | `limit`, `offset` | `Notification[]` newest first |
| GET    | `/notifications/unread-count`   | ✓    |                   | number |
| POST   | `/notifications/{id}/read`      | ✓    |                   | message (404 if not yours) |
| POST   | `/notifications/read-all`       | ✓    |                   | number updated |

`Notification` fields: `id, userId, actorId, actorName, kind: "follow" |
"comment" | "reaction", postId?, commentId?, isRead, createdAt`.

## Files

| Method | Path                    | Auth                | Body / Query                                             | Returns |
|--------|-------------------------|---------------------|----------------------------------------------------------|---------|
| GET    | `/files`                | depends on scope    | `scope=public` (default, anonymous) / `mine` (✓) / `all` (admin) | `File[]` |
| POST   | `/files`                | ✓                   | multipart: one or more `files` parts, optional `description`; query `public=true|false` (default true) | `File[]` |
| GET    | `/files/{id}/download`  | public or owner/admin | `If-None-Match` honoured (304)                        | file bytes, `Content-Disposition: attachment`, `Cache-Control: immutable` (public) + `ETag` |
| GET    | `/files/{id}/stream`    | public or owner/admin | `Range` header supported incl. `bytes=-N` (max 4 MiB per request); `If-None-Match` honoured | 206 partial content |

Uploading a file whose SHA-256 matches one of *your own* earlier uploads
soft-deletes that older record, unless it is attached to a post or comment
or used as an avatar; other users' files are never touched. One upload
request may carry at most `MAX_UPLOAD_MB` (default 300) in total and a
`description` of at most 4 KB (413 beyond either). Attach files to a post or comment by passing their ids in
`attachments`; only the uploader's own files can be attached.

## Health

`GET /api/health` → `{ "code": 200, "message": "ok" }` (liveness: the process
answers).

`GET /api/health/ready` → the same, but only after a `select 1` round-trip to
PostgreSQL succeeds; `503` otherwise (readiness probes, load balancers).
