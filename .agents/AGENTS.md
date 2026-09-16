# Ax — notes for developers and coding agents

Read `README.md` first for setup. This file covers conventions that are not
obvious from the code.

## Shape of the codebase

- **Backend** (`tweet_server/`, one crate): `routes.rs` → `handlers/` →
  `db/`. Handlers do auth + validation + response shaping; `db/` owns every
  SQL statement; `models/` holds request/response types with their
  `normalize()`/`validate()` methods. No SQL outside `db/`.
- **Frontend** (`frontend/`): one view per page under `src/views/`, shared
  pieces under `src/components/`, all HTTP through `src/api/index.ts`.
- **Database**: PostgreSQL. Counters (`like_count`, `dislike_count`,
  `engagement_rate`), the cached `user_name` on posts, and the
  `notifications` rows are maintained by triggers in `migrations/`. Never
  update them from application code.

## Rules

1. **Sessions**: call `auth::require_user(&session)?` (or `require_admin`)
   at the top of any handler that needs a signed-in user. Use
   `auth::current_user` when anonymous access is fine but the viewer matters.
2. **Ownership**: for update/delete, load the row, then
   `user.authorize_owner(row.user_id)?` (owner or admin). Never trust a user
   id from the request body.
3. **Errors**: return `AxError`; never `unwrap()` on request data or query
   results. `sqlx::Error::RowNotFound` already maps to 404, unique
   violations to 400.
4. **SQL**: parameterized (`$1`) always. Dynamic `ORDER BY` must go through
   a whitelist (see `PostListQuery::ordering`). After changing any query run
   `just sqlx-prepare` and commit `.sqlx/`.
5. **Schema**: add a new file under `migrations/`; never edit an applied one.
   Migrations run at backend startup.
6. **Responses**: use `response::ok`, `ok_paged`, `ok_message`. Errors use
   real HTTP statuses (401/403/404/400), not `200` with an error code.
7. **Frontend**: keep payload types in `src/types.ts` in camelCase (the
   backend serializes with `rename_all = "camelCase"`). Lists come
   pre-hydrated (`viewerReaction`, `commentCount`, attachments); do not add
   per-card requests.
8. **Tests**: unit tests only, colocated (`#[cfg(test)]`), no database or
   network. Put logic worth testing in pure functions (validation, parsing,
   query building) and test those.

## Before finishing a change

```bash
just ci        # fmt/clippy/test + frontend typecheck/build
```

Once a change is implemented and verified, commit it with a clear,
descriptive message.

Update `docs/src/api.md` when routes or payloads change, `docs/src/database.md`
when the schema changes, and add a line to `CHANGELOG.md`.
