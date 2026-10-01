# Development plan — `dev` branch (started 2026-10-01)

Working context for the multi-agent development round. Each workstream is
developed in its own git worktree on its own branch cut from `dev`, verified
with `just ci`, then merged back into `dev` by the integrator. User-facing
changes are recorded in `CHANGELOG.md` under `[Unreleased]`.

## Layout

| Workstream | Branch | Worktree | Dev database | Migration prefix |
|---|---|---|---|---|
| Search | `feat/search` | `../ax-wt/search` | `ax_search` | `20261001100000`+ |
| Bookmarks + profile editing | `feat/bookmarks-profile` | `../ax-wt/bookmarks-profile` | `ax_bookmarks` | `20261001200000`+ |
| Backend performance + hardening | `perf/backend` | `../ax-wt/backend` | `ax_perf` | `20261001300000`+ |
| Frontend performance + a11y | `perf/frontend` | `../ax-wt/frontend` | `ax` | none (frontend only) |
| Infra / docs (PG17, CI on `dev`) | `dev` directly | main checkout | `ax` | none |

All databases live in the one compose container (PostgreSQL 17, host port
55432). Each backend branch has its own database so migrations from different
branches never collide during development.

## Workstreams

1. **Search** — full-text search over posts (title + content) and people,
   backed by PostgreSQL indexes (tsvector/GIN and/or pg_trgm); a
   `GET /api/search` endpoint (paginated, typed results); a Search page with
   debounced input, result tabs and highlighting, reachable from the navbar.
2. **Bookmarks + profile editing** — save/unsave posts (`viewerBookmarked`
   hydrated in lists, a Saved page); users edit their own profile (full name,
   bio, avatar via the existing upload pipeline / `profile_picture`).
3. **Backend performance + hardening** — EXPLAIN-driven index audit of every
   query in `db/`, response compression, security headers, cache headers /
   conditional requests for files, graceful shutdown and pool tuning, more
   pure-function unit tests. No new product features.
4. **Frontend performance + a11y** — route-level code splitting and vendor
   chunking, lazy heavy dependencies (markdown/sanitizer/editor), image
   loading hints, keyboard and screen-reader pass (dialog focus, labels),
   bundle size measured before/after. No backend changes.
5. **Infra / docs** — PostgreSQL 17 everywhere in docs, CI runs on `dev`.

## Rules for every workstream

Follow `AGENT.md`. Before finishing: `SQLX_OFFLINE=true just ci` passes,
`.sqlx/` regenerated if SQL changed, docs (`docs/src/api.md`,
`docs/src/database.md`) updated, a `CHANGELOG.md` line added, work committed
on the workstream branch.

## Status

- [x] PostgreSQL 17 compose + `just run` offline compile (`4b41276`)
- [ ] Search
- [x] Bookmarks + profile editing (merged)
- [ ] Backend performance + hardening
- [x] Frontend performance + a11y (merged)
- [x] Infra / docs
- [ ] Integration: merge all into `dev`, full `just ci`, smoke test on a fresh DB
