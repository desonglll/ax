# Architecture

## Backend

The backend is a single crate, `tweet_server`, organised by responsibility:

| Module          | Responsibility                                                                    |
|-----------------|-----------------------------------------------------------------------------------|
| `main.rs`       | Loads config, connects to PostgreSQL, **runs migrations**, mounts middleware       |
| `config.rs`     | Environment variables → `ServerConfig`; session key derivation                     |
| `routes.rs`     | The URL table under `/api`                                                        |
| `handlers/`     | One module per resource. Resolve the session, validate input, call `db`, respond |
| `db/`           | One module per table. All SQL lives here, checked at compile time by SQLx         |
| `models/`       | Row types, request payloads and their `normalize()` / `validate()` methods       |
| `auth.rs`       | Session helpers: `current_user`, `require_user`, `require_admin`, `authorize_owner` |
| `response.rs`   | The JSON envelope (`ok`, `ok_paged`, `ok_message`) and `PageQuery`                |
| `errors.rs`     | `AxError` → HTTP status + `{code, message}` body                                    |
| `services/`     | Background work: OpenAI-compatible client and the title queue                     |

### Request flow

```
HTTP request
  └─ middleware: request log → per-IP rate limit → cookie session → CORS
       └─ routes.rs matches /api/...
            └─ handler: auth::require_user(&session)? → payload.normalize()? → db::...
                 └─ db: sqlx::query!(...) against PgPool
            └─ response::ok(...) / AxError (real HTTP status)
```

### Sessions

`actix-session` with `CookieSessionStore`: the session is an encrypted,
signed cookie holding `user_id`, `user_name` and `is_admin`. Set
`SESSION_SECRET_KEY` (32+ characters) so cookies survive restarts.

### Hydration instead of N+1

List endpoints return posts and comments already "hydrated": attachments,
comment counts, reaction counts and the viewer's own reaction are fetched
with a handful of batched `= any($1)` queries per page
(`db::post::hydrate`, `db::comment::hydrate`). The frontend never issues
per-card requests.

### Trending

`db::post::trending` ranks posts in SQL with a Hacker-News style score:

```
(likes × 2 − dislikes + comments × 3) / (age_in_hours + 2) ^ 1.5
```

### AI titles

`services::title_queue` starts only when `OPENAI_API_KEY` is set. Posts
published without a title are sent down an in-memory channel; a worker asks
the model for a short title and writes it back if the post is still
untitled. On startup it also queues any existing untitled posts.

### Migrations

`sqlx::migrate!("../migrations")` embeds the SQL files into the binary and
applies pending ones on startup, so a fresh database needs no separate
step. The compile-time query check uses the committed `.sqlx/` cache
(`SQLX_OFFLINE=true`), so CI and Docker builds need no database.

## Frontend

Vue 3 + TypeScript + Vite, styled with Tailwind CSS 4 and daisyUI 5, state in
Pinia.

| Path                     | Purpose                                                        |
|--------------------------|----------------------------------------------------------------|
| `src/main.ts`            | App bootstrap; restores the session once before the first route |
| `src/router.ts`          | Routes; `requiresAuth` / `guest` meta guards                    |
| `src/api/`               | Axios instance (`/api`, cookies on) and typed endpoint functions |
| `src/stores/`            | `auth` (current user) and `toast`                               |
| `src/views/`             | One component per page                                          |
| `src/components/`        | `AppShell` (nav + drawer), `PostCard`, `CommentCard`, `ReactionBar`, `ComposerCard`, `MarkdownEditor`, `MarkdownBody`, `FilePreview`, `PaginationBar`, `Avatar`, `EmptyState`, `UserRow`, `NotificationBell`, `ToastHost` |
| `src/lib/markdown.ts`    | markdown-it + DOMPurify rendering, `embedsImage`, `excerpt`           |
| `src/lib/format.ts`      | `timeAgo`, `formatSize`, `initials`                            |

### Content

Post and comment bodies are Markdown. The editor uploads pasted, dropped or
picked files immediately (`POST /api/files`) and inserts `![name](url)` for
images or `[name](url)` for other files at the cursor; the same ids are sent
as `attachments` so the server links them to the post. Rendering uses
markdown-it (no raw HTML) sanitized with DOMPurify; attachments already
embedded as images are not repeated below the body.

In development Vite proxies `/api` to the backend port found in
`.server-port`. In production nginx serves `dist/` and proxies `/api` to
the backend container (`frontend/nginx.conf.template`).
