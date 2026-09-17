# Changelog

This document logs the development history and version alterations of Project Ax.

## [Unreleased]

### Changed
- **Sign in without leaving the page.** Guests who like, comment, follow or
  open the Following tab get a sign-in/register dialog; once signed in the
  action carries on. The login and register pages share the same form and
  both honour `?redirect=`.
- **Post cards open the post** when clicked anywhere that isn't a link,
  button, image or text selection (⌘/Ctrl-click opens a new tab). A post
  opened from a list renders instantly from the list's copy.
- **Post detail:** the comment box is a single line that expands (drafts are
  kept per post for the session), the comment button and `#comments` links
  jump to it, new comments appear at the top highlighted, comments have a
  Reply button that mentions the author, and deleting shows a confirmation
  toast. Back goes home when the post was opened directly; a deleted post and
  a failed load are told apart (with Retry).
- **Likes, comment counts, edits and deletions stay in sync** across the
  detail page and the kept-alive list pages (`stores/posts.ts`).
- **Notifications:** the navbar dropdown now actually loads (daisyUI disables
  pointer events on a focused trigger, so its click handler never ran), shows
  load errors with Retry, and re-syncs the unread count when opened. The
  sidebar shows the unread badge and the tab title is prefixed with it.
- Reaction notifications are deduplicated per actor and post, and withdrawn
  while unread when the reaction is undone.

### Fixed
- Clicking Like/Dislike inside a list card no longer also opens the post.
- "Who to follow" no longer lists yourself after signing in from the dialog.

## [0.5.0] - 2026-09-16

A simplification release: one database, one backend crate, one frontend, and
fast tests that need no infrastructure.

### Removed
- **Redis.** Sessions now live in an encrypted cookie (`actix-session`
  `CookieSessionStore`) holding only the user id, name and admin flag.
- **The Python recommendation service** (`model_server/`). Trending is a single
  SQL query with a Hacker-News style score (`db::post::trending`).
- **The `ai` crate.** The OpenAI-compatible client is now
  `tweet_server/src/services/ai.rs`; the title worker starts only when
  `OPENAI_API_KEY` is set.
- **Frontends v1.0 and v1.1.** The Vue app moved from `frontend/v1.2/` to
  `frontend/`.
- **The `/stats` endpoint and System page**, which reported request counts that
  were never fully collected.
- **49 database-backed handler tests**, the custom colored `Log` facade,
  `juniper`, `clap`, `actix-redis`, `colored`, `env_logger` and other unused
  dependencies. `adminer` left `compose.yml`.

### Changed
- **REST-style routes**: `POST /api/posts`, `GET /api/posts/{id}`,
  `PUT /api/reactions`, `POST /api/files?public=…`, `GET /api/auth/me`, etc.
  (see `docs/src/api.md`). Errors return real HTTP statuses (400/401/403/404)
  with a `{code, message}` body instead of `200` with an embedded code.
- **Hydrated lists**: posts include `attachments`, `commentCount` and
  `viewerReaction`; comments include reaction counts and `viewerReaction`. The
  frontend no longer issues one to two requests per card.
- **Migrations run on startup** (`sqlx::migrate!`), so a new database needs
  no `sqlx-cli`. SQL is checked against the committed `.sqlx/` cache, so
  `cargo check`, tests, CI and Docker builds work without a database.
- **Backend layout**: `routes.rs` → `handlers/` → `db/` (renamed from
  `dbaccess/`), `models/` with `normalize()`/`validate()`, `auth.rs`,
  `response.rs`, `config.rs`. Logging is `tracing` only.
- **CI** is two jobs (backend fmt/clippy/test, frontend typecheck/build) with
  no service containers; the image workflow builds two images.
- `compose.yml` runs PostgreSQL only; `compose.prod.yml` runs the published
  images.
- Frontend: Markdown editor (toolbar, shortcuts, preview, paste/drop/pick
  uploads inserted at the cursor) for posts and comments, sanitized Markdown
  rendering; the standalone Files page is gone — files exist as attachments.
  Plainer copy throughout.
- Frontend: collapsed one-line composer with draft autosave, themed
  confirm/prompt dialogs (no native `confirm()`), image lightbox, colour-coded
  avatars, shared unread badge (navbar, bottom nav, notifications page),
  back-to-top button, phone search bar, `/` to focus search, page titles,
  favicon.
- Frontend: English / Simplified Chinese UI (vue-i18n, auto-detected,
  switchable); infinite scrolling for feeds, profile posts, notifications and
  comments; kept-alive list pages so Back restores scroll position; page and
  list transitions, like animation, skeleton loaders; phone bottom navigation
  and compose button; side-by-side Markdown preview with image thumbnails;
  "Show more" expands posts in place; clearer section headers on the home page.
- Frontend: relative timestamps, optimistic reactions with rollback, shared
  `Avatar` / `EmptyState` / `UserRow` / `ReactionBar` components, a trending
  widget on the home page, paginated profile posts and comments, feed tab and
  page kept in the URL, auto sign-in after registration, mobile search in the
  drawer, structured error toasts.

### Fixed
- Horizontal overflow on phones (the feed column and editor could exceed the
  viewport); verified at 390px on every page.
- Image-heavy feeds tripped the rate limiter (HTTP 429 on images); the limit
  now applies to write requests only.
- File downloads send `Cache-Control` and an `ETag` (304 on revalidation).
- Theme no longer flashes light before dark on load.
- The Vite dev proxy follows the backend to a new port without a restart.
- Markdown paragraphs no longer render doubled blank lines.
- Admins can delete users and other users' comments from the UI (the API
  previously only allowed self-deletion).
- Deactivated accounts can no longer sign in; `last_login` is now recorded.
- A missing post/user/comment returns 404 instead of a 500 "Database error";
  duplicate user names/emails return 400.
- Post search also matches the title.
- Uploading with no file part returns 400 instead of an empty success.
- Trending no longer requires sign-in.

## [0.4.0] - 2026-06-18

### Added
- Added Dockerfile (`tweet_server/Dockerfile`) for compiling and packaging the Rust backend server.
- Added GitHub Actions CI/CD workflow `.github/workflows/deploy.yml` to automatically build, tag, and publish the backend, frontend, and AI model-server Docker images to the GitHub Container Registry (GHCR) on pushes to the main branch.
- Implemented file attachments support for comments. Users can upload and associate multiple public files when publishing a comment or nested reply, which are stored and associated via a new `comment_id` column in the `files` table.
- Created database migration `20260618190000_add_comment_id_to_files.sql` to add the `comment_id` column referencing `comments(id)` to the `files` table.
- Added `CommentDetail` data transfer model on the backend and updated comment retrieval/creation API endpoints to fetch and return comments along with their nested attachments.
- Implemented an asynchronous background message queue (`QueueWorker`) using `tokio::sync::mpsc::unbounded_channel` to handle post title completion via the `ai` crate. When a post is created without a title, its ID is sent to the queue to generate a title from its content using the OpenAI Chat Completion API.
- Added a database scanner (`scan_and_enqueue_empty_titles`) running on server startup to scan all posts with empty or null titles and enqueue them for background title completion.
- Added integration test coverage (`test_queue_worker_process_post`) to verify correct enqueuing, mock OpenAI API response processing, and post title updating.
- Implemented text-based post search functionality. Extended the database query in `post.rs` to support dynamic query keyword filtering using `content ILIKE $1` on the PostgreSQL level, and added a search input bar above the home page timeline.
- Added direct page selection jumping. Configured a `<select>` dropdown next to timeline and comments pagination controls to allow standard HTTP page-reload navigation directly to any page number while preserving active search query filters.
- Implemented multi-file attachments support for posts. Users can upload and link multiple public files when publishing a post, which are stored and associated via a new `post_id` column in the `files` table.
- Added a global floating scroll-to-top button in the master application layout (`root.tsx`) that appears when scrolling down past 300px.

### Changed
- Refactored the frontend container image build (`frontend/v1.1/Dockerfile`) to use Bun instead of npm, ensuring alignment with the project lockfile (`bun.lock`) and dev environment, reducing the final image footprint.
- Extended frontend comment components (`post.tsx` and `CommentNode.tsx`) to manage comment attachments. Users can now select and upload files during comment creation and replies, and view attachments directly under comments with toggleable previews.
- Exported and reused `AttachmentItemRenderer` component from `PostItem.tsx` to display file attachments consistently for both posts and comments.
- Configured the navigation header (`root.tsx`) to wrap and stack layout elements dynamically on narrow mobile viewports, resolving layout overlaps.
- Configured header containers, edit lists, action bars, and latency tables of posts, comments, and monitoring widgets to dynamically wrap and adjust element widths on small mobile screens.
- Refactored the entire frontend page layouts, form inputs, buttons, tables, alerts, and stats panels to utilize standard, clean daisyUI components. Standardized the theme design to follow flat, content-focused minimalist principles without flashy gradients, animations, or shadows.
- Configured the default sorting column of the post list database query to fallback to `created_at` instead of random `id` (UUID), sorting timeline posts chronologically by default.
- Configured post titles to be optional in the frontend creation form and editing view, removing the `required` HTML attributes and updating validation logic.
- Restricted access to the files list manager (`/files` page and navbar tab) exclusively to administrators. Regular users can upload files through post attachments but cannot view the overall files list.
- Replaced immediate timeline media rendering with user-triggered stateful previews using expand/collapse buttons to keep the layout compact.
- Replaced standard file input state mapping with an accumulated list interface in the post composer to allow uploading multiple files across multiple selection triggers, removing individual attachments, and restricting thumbnail previews exclusively to image and video types.
- Extended post editing handlers and components to manage attachments, permitting the user to remove existing linked files when updating a post.

### Fixed
- Fixed comment creation API payload field mismatch in `commentApi.create` where the frontend sent `replyType` instead of `replyToType`, causing the backend to reject replies with HTTP status code 400.
- Fixed home timeline API query sorting parameters in `home.tsx` to request sorting by `created_at` DESC instead of arbitrary `id` values, ensuring correct chronological order.

## [0.3.2] - 2026-06-17

### Added
- Implemented reaction cancellation and toggle logic in `PostItem.tsx` and `CommentNode.tsx`.
- Migrated timeline and comments pagination from memory state to URL search parameters (`offset`) using standard HTTP `href` links to preserve page memory on browser refreshes.
- Re-structured the timeline homepage into a modern, minimalist two-column layout: timeline timeline list and post-creation form on the left, session status/server monitor/project info widgets in a right sidebar.
- Redesigned likes, dislikes, comments, and delete actions into clean, flat-bordered mono buttons with subtle visual cues.
- Added automatic scroll position saving (to `sessionStorage`) and restoration globally across all async views (home timeline, trending, post details, files hub, and profile stats), ensuring seamless back-navigation and refresh state preservation.

### Changed
- Expanded root layout max-width constraint from `max-w-3xl` to `max-w-5xl` to support desktop grid splitting.

### Fixed
- Fixed runtime crash in `SystemStatsWidget` where route latencies `response_times` and request count `request_count` received from the backend stats endpoint were serialized in snake_case, causing undefined exceptions on camelCase references. The API client now maps these keys properly.

## [0.3.1] - 2026-06-17

### Added
- Added frontend automation recipes to the root `justfile` (`fe-install`, `fe-dev`, `fe-check`, `fe-build`).
- Implemented Reddit-style content truncation and inline expand/collapse toggles in `PostItem.tsx` for posts longer than 280 characters in list views.

### Fixed
- Fixed login session persistence bug where refreshing the browser cleared the logged-in status. The frontend now queries the full user profile details from `/api/users/profile` to restore sessions.

## [0.3.0] - 2026-06-16

### Added
- Reconstructed the frontend in `frontend/v1.1` using Bun + Vite + React TS + React Router v7 + TailwindCSS + Axios, replacing the old `v1.0` client.
- Implemented a unified Axios HTTP client in `app/utils/api.ts` with credentials support to manage session states.
- Created `AuthContext.tsx` providing global authentication state (login, logout, registration) and session checks.
- Designed a minimalist, content-focused GNU-style UI header, timeline list, and footer.
- Added a recursive comment tree component (`CommentNode.tsx`) to support multi-level nested replies, likes/dislikes on comments, and comment deletion.
- Added client-side real-time user statistics computation (total posts, average likes/dislikes, engagement rate) for the profile view.
- Added admin panels for user list viewing/deletion and comprehensive file listing.
- Wrote frontend development guidelines in `frontend/v1.1/FRONTEND_AGENT.md` and added reference rules to `AGENT.md`.
- Created the Frontend Design & Guide chapter in the mdBook documentation.

### Changed
- Migrated all route configurations to the React Router v7 config-based layout.
- Styled all components using pure TailwindCSS without complex transition animations or layout shadows, matching GNU's visual identity.

## [0.2.0] - 2026-06-16

### Added
- Created `justfile` in the project root to automate database setup, testing, compiling, and documentation.
- Integrated `mdBook` documentation inside the `docs` directory, detailing the design, API reference, database schema, and developer guidelines.
- Created `CHANGELOG.md` to keep record of modifications.
- Introduced safety checks on the sorting column parameter `order_by` and order parameter `sort` to mitigate SQL injection.
- Added comprehensive test coverage validating safety check behaviors, unauthorized request blocking, and comment pagination.

### Changed
- Ported all source code comments from Chinese to GNU-style English.
- Refactored `get_comment_by_query` and `get_comment_by_query_db` to support pagination parameters (`limit` and `offset`).

### Fixed
- Fixed unauthenticated access bypass in comment and post handlers where the result of `login_in_unauthentic` was discarded.
- Fixed route parameter definition for comment deletion by appending `/delete/{id}` placeholder.
- Fixed panics caused by unwrapping empty query parameter options in backend handlers.
- Fixed a trigger logic error where reactions on comments updated the reaction counts of posts containing the same numerical ID.
- Fixed a trigger logic error where `average_comment_count` was calculated using post dislikes instead of comment count statistics.
- Fixed user stats updates to properly trigger and calculate metrics when posts are deleted.
- Fixed GitHub Actions CI workflow to target the correct database name (`ax`), install development tools via precompiled binaries (`taiki-e/install-action`), utilize dependency caching (`swatinem/rust-cache`), and execute validation tasks using `just` recipes.

### Removed
- Deleted obsolete Bash script `scripts/init_db.sh`.
