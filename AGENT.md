# Project Ax: Developer & AI Agent Guidelines

Welcome, Developer/AI Agent! This document outlines the architecture, coding standards, security requirements, database design, and testing protocols for the Project Ax codebase. 

Please read and adhere to these guidelines to maintain consistency, security, and stability.

---

## 1. Project Overview

Project Ax is a secure micro-blogging and media server system.
- **Backend (`tweet_server`)**: Written in Rust using the **Actix-web** framework.
  - **Database**: PostgreSQL managed via **SQLx** (with offline compile-time SQL verification).
  - **Caching / Session Store**: Redis (via `actix-session`).
- **Frontend**: Vite TypeScript SPA (under `frontend/v1.0`).

---

## 2. Architecture & File Structure

The backend follows a layered, modular architecture:
- [src/main.rs](file:///Volumes/Tuo-APFS/workspace/ax/tweet_server/src/main.rs): Server entrypoint, configuration of AppState, CORS, Session Middleware, and routes.
- [src/routes/](file:///Volumes/Tuo-APFS/workspace/ax/tweet_server/src/routes): Scoped route configurations (e.g., `/api/posts`, `/api/comments`).
- [src/handlers/](file:///Volumes/Tuo-APFS/workspace/ax/tweet_server/src/handlers): Request handlers containing controller logic, payload parsing, and HTTP responses.
- [src/dbaccess/](file:///Volumes/Tuo-APFS/workspace/ax/tweet_server/src/dbaccess): Data access layer handling raw SQLx operations.
- [src/models/](file:///Volumes/Tuo-APFS/workspace/ax/tweet_server/src/models): DTOs, entity structures, and payload validation structs.
- [src/extractors/](file:///Volumes/Tuo-APFS/workspace/ax/tweet_server/src/extractors): Custom Actix request extractors and response wrapper structures.

---

## 3. Strict Coding Guidelines

### 3.1 Security & Authentication
- **Session Verification**: All endpoints requiring authentication must check the session state early. Use `login_in_unauthentic`:
  ```rust
  if let Ok(resp) = login_in_unauthentic(&session).await {
      return Ok(resp);
  }
  ```
- **Access Control (IDOR Prevention)**: Destructive actions (such as updates, deletions, uploads) must check ownership. Verify that the session `user_id` matches the owner of the resource or that the logged-in user is an administrator:
  ```rust
  let is_admin_user = crate::extractors::session::is_admin(session.clone()).await.unwrap_or(false);
  if resource.user_id != user_id && !is_admin_user {
      return Ok(HttpResponse::Unauthorized().json(...));
  }
  ```
- **Password Security**: Never store raw passwords. Use the `Hash` utility wrapper (`bcrypt` underneath) for hashing and verification.

### 3.2 Stability & Panic Prevention
- **Safe Query Handling**: Do NOT use `.unwrap()` on query parameters. Always treat query parameters as optional and fallback safely:
  ```rust
  let query_map = query.map(|q| q.into_inner()).unwrap_or_default();
  ```
- **Avoid Thread Panics**: Never call `.unwrap()` on database fetch results or session parameters where failure could crash the Actix request worker thread. Propagate errors gracefully using the `?` operator or `.unwrap_or_default()`.

### 3.3 SQL Injection Protection
- Do not interpolate dynamic values directly into SQL strings. Always use parameterized queries (`$1`, `$2`, etc.) via SQLx.
- If dynamic sorting or column selection is required (which cannot use SQL parameters), validate the input against a strict whitelist of fields:
  ```rust
  let valid_order_by = ["id", "created_at", "updated_at"];
  if !valid_order_by.contains(&order_by) {
      return Err(AxError::InvalidInput("Invalid sorting column".to_string()));
  }
  ```

---

## 4. Database & Triggers Design

PostgreSQL manages real-time counts and user scoring asynchronously or via triggers:
- **`update_post_reaction_counts`**: Automatically recalculates likes and dislikes on the `posts` table whenever reactions are modified (ensure it is guarded to only run for `to_type = 'post'`).
- **`update_user_stats`**: Triggered on inserts, updates, and deletes of `posts`. It automatically updates the user's averages, post counts, comment counts, and engagement rate metrics in the `user_stats` table.
- **Username Cache**: Trigger `set_user_name` automatically synchronizes the creator's username into the posts/comments record on creation.

All triggers and tables are version-controlled in the [migrations](file:///Volumes/Tuo-APFS/workspace/ax/migrations) folder. When modifying schemas, write an incremental SQL migration file.

---

## 5. Testing Requirements & Execution

- **Test Runner**: Project Ax uses `cargo-nextest` for faster and isolated testing.
- **Running Tests**: Run tests from the root or the `tweet_server` directory:
  ```bash
  cargo nextest run
  ```
- **Writing Tests**:
  - Write unit/integration tests directly in the handler files under `#[cfg(test)] mod tests`.
  - To test session-bound endpoints without needing a running Redis server, perform **direct handler testing** by creating a mock session locally rather than calling the handlers via `test::init_service`:
    ```rust
    let session = get_demo_session().await;
    let resp = insert_new_post(session, app_state.clone(), post_payload).await.unwrap();
    ```
  - Always clean up test data inserted into the database at the end of the test.

---

## 6. How to Extend Project Ax

If you are asked to implement a new endpoint:
1. **Model**: Define request/response payload structs in `src/models`.
2. **Database Access**: Implement parameterized queries in `src/dbaccess` returning `Result<T, AxError>`.
3. **Handler**: Write a controller function in `src/handlers` executing authentication, authorization, db access, and format response.
4. **Route**: Register the route scope in the corresponding routes file under `src/routes`.
5. **Test**: Write direct handler test cases verifying both success and failure (e.g., unauthorized access, invalid input) execution paths.
