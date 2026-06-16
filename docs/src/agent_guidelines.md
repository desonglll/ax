# Developer & Agent Guidelines

This chapter defines the code quality, safety, and documentation updating standards required for Project Ax.

## Coding Standards

1. **Security & Authentication**:
   Verify session login status early in the handler. Always handle authentication failure and return 401 response explicitly:
   ```rust
   if let Ok(resp) = login_in_unauthentic(&session).await {
       return Ok(resp);
   }
   ```

2. **Access Control (IDOR Prevention)**:
   Any write or destructive handler (such as PUT, DELETE) must verify that the session user ID matches the owner of the entity being operated on (unless the user has administrative privileges):
   ```rust
   if entity.user_id != user_id && !is_admin {
       return Ok(HttpResponse::Unauthorized().json(...));
   }
   ```

3. **Panic Prevention**:
   Do not unwrap query parameters directly. Handle `Option<web::Query<...>>` values using defensive programming techniques:
   ```rust
   let query_map = query.map(|q| q.into_inner()).unwrap_or_default();
   ```

4. **SQL Injection Mitigation**:
   Always use parameterized queries. If sorting columns or ordering parameters are dynamic, validate them against a strict whitelist before building SQL query strings.

---

## Documentation Requirement

Whenever code modifications are introduced to the project:
1. Update `docs/src/api.md` if routes, query parameters, payloads, or HTTP response formats change.
2. Update `docs/src/database.md` if database tables, schema, triggers, or indexes are added or changed.
3. Update `CHANGELOG.md` with version numbers, dates, and detailed lists of additions, alterations, and bug fixes.
4. Ensure all documentation matches the GNU style, maintaining third-person narrative, objective descriptions, and clean plain formats.
