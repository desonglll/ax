//! Search over posts and people. The matching rules and their indexes are
//! described in `migrations/20261001100000_add_search_indexes.sql`.

use sqlx::PgPool;

use crate::{
    errors::AxError,
    models::{post::Post, search::SearchTerm, user::User},
    response::Pagination,
};

/// Posts whose words match the query or whose title/content contains it.
/// Best matches first (title hits outrank body hits), then newest.
pub async fn posts(
    pool: &PgPool,
    term: &SearchTerm,
    limit: i64,
    offset: i64,
) -> Result<(Vec<Post>, Pagination), AxError> {
    let posts = sqlx::query_as!(
        Post,
        r#"select p.id, p.title, p.content, p.created_at, p.updated_at, p.user_id,
                  p.reply_to, p.user_name, p.like_count, p.dislike_count, p.engagement_rate
           from posts p, websearch_to_tsquery('simple', $1) q
           where posts_search_document(p.title, p.content) @@ q
              or p.title ilike $2
              or p.content ilike $2
           order by
             ts_rank(posts_search_document(p.title, p.content), q)
               + case when p.title ilike $2 then 1.0 else 0.0 end
               + case when p.content ilike $2 then 0.25 else 0.0 end desc,
             p.created_at desc,
             p.id
           limit $3 offset $4"#,
        term.text,
        term.pattern,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!"
           from posts p, websearch_to_tsquery('simple', $1) q
           where posts_search_document(p.title, p.content) @@ q
              or p.title ilike $2
              or p.content ilike $2"#,
        term.text,
        term.pattern
    )
    .fetch_one(pool)
    .await?;

    Ok((
        posts,
        Pagination {
            limit,
            offset,
            count,
        },
    ))
}

/// Active users whose user name or full name contains the query. Exact user
/// name first, then prefix matches, then closest by trigram similarity.
pub async fn users(
    pool: &PgPool,
    term: &SearchTerm,
    limit: i64,
    offset: i64,
) -> Result<(Vec<User>, Pagination), AxError> {
    let users = sqlx::query_as!(
        User,
        r#"select u.* from users u
           where u.is_active
             and (u.user_name ilike $2 or u.full_name ilike $2)
           order by
             (lower(u.user_name) = lower($1)) desc,
             (u.user_name ilike $3 or u.full_name ilike $3) desc,
             greatest(similarity(u.user_name, $1), similarity(coalesce(u.full_name, ''), $1)) desc,
             u.id
           limit $4 offset $5"#,
        term.text,
        term.pattern,
        prefix_pattern(&term.pattern),
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    let count = sqlx::query_scalar!(
        r#"select count(*) as "count!" from users u
           where u.is_active
             and (u.user_name ilike $1 or u.full_name ilike $1)"#,
        term.pattern
    )
    .fetch_one(pool)
    .await?;

    Ok((
        users,
        Pagination {
            limit,
            offset,
            count,
        },
    ))
}

/// `%text%` → `text%` (the leading `%` is never escaped, so dropping it is safe).
fn prefix_pattern(contains: &str) -> &str {
    contains.strip_prefix('%').unwrap_or(contains)
}

#[cfg(test)]
mod tests {
    use super::prefix_pattern;

    #[test]
    fn prefix_pattern_drops_leading_wildcard_only() {
        assert_eq!(prefix_pattern("%ax%"), "ax%");
        assert_eq!(prefix_pattern("%50\\%%"), "50\\%%");
    }
}
