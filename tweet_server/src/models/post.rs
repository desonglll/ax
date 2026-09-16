use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{errors::AxError, models::file::File, response::PageQuery};

pub const MAX_TITLE_LEN: usize = 120;
pub const MAX_CONTENT_LEN: usize = 10_000;

/// Row of `posts`.
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: i32,
    pub reply_to: Option<Uuid>,
    pub user_name: String,
    pub like_count: i32,
    pub dislike_count: i32,
    pub engagement_rate: f64,
}

/// A post as returned to clients: the row plus everything a card needs to
/// render without further requests.
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PostDetail {
    #[serde(flatten)]
    pub post: Post,
    pub attachments: Vec<File>,
    pub comment_count: i64,
    /// `"Like"` / `"Dislike"` for the signed-in viewer, otherwise `None`.
    pub viewer_reaction: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
    pub title: Option<String>,
    pub content: String,
    #[serde(default)]
    pub attachments: Vec<Uuid>,
}

impl CreatePost {
    pub fn normalize(&mut self) -> Result<(), AxError> {
        self.title = self
            .title
            .take()
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty());
        self.content = self.content.trim().to_string();
        validate_text(self.title.as_deref(), Some(&self.content))
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    /// When present, replaces the whole attachment set.
    pub attachments: Option<Vec<Uuid>>,
}

impl UpdatePost {
    pub fn normalize(&mut self) -> Result<(), AxError> {
        self.title = self.title.take().map(|t| t.trim().to_string());
        self.content = self.content.take().map(|c| c.trim().to_string());
        if self.content.as_deref() == Some("") {
            return Err(AxError::invalid("content cannot be empty"));
        }
        validate_text(self.title.as_deref(), self.content.as_deref())
    }
}

fn validate_text(title: Option<&str>, content: Option<&str>) -> Result<(), AxError> {
    if title.is_some_and(|t| t.chars().count() > MAX_TITLE_LEN) {
        return Err(AxError::invalid(format!(
            "title must be at most {MAX_TITLE_LEN} characters"
        )));
    }
    match content {
        Some("") => Err(AxError::invalid("content cannot be empty")),
        Some(c) if c.chars().count() > MAX_CONTENT_LEN => Err(AxError::invalid(format!(
            "content must be at most {MAX_CONTENT_LEN} characters"
        ))),
        _ => Ok(()),
    }
}

/// Query parameters for `GET /api/posts`.
#[derive(Deserialize, Debug, Default, Clone)]
pub struct PostListQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub order_by: Option<String>,
    pub sort: Option<String>,
    pub search: Option<String>,
    #[serde(alias = "userId")]
    pub user_id: Option<i32>,
}

/// Sort columns a client may ask for. Anything else is rejected so the
/// column name can be interpolated into SQL safely.
const SORT_COLUMNS: &[&str] = &[
    "created_at",
    "updated_at",
    "like_count",
    "dislike_count",
    "engagement_rate",
];

impl PostListQuery {
    pub fn page(&self) -> PageQuery {
        PageQuery {
            limit: self.limit,
            offset: self.offset,
        }
    }

    /// Returns the validated `(column, direction)` pair for `ORDER BY`.
    pub fn ordering(&self) -> Result<(&'static str, &'static str), AxError> {
        let column = match self.order_by.as_deref() {
            None => "created_at",
            Some(requested) => SORT_COLUMNS
                .iter()
                .copied()
                .find(|c| *c == requested)
                .ok_or_else(|| AxError::invalid(format!("Invalid order_by field: {requested}")))?,
        };
        let direction = match self.sort.as_deref().map(str::to_ascii_lowercase).as_deref() {
            None | Some("desc") => "DESC",
            Some("asc") => "ASC",
            Some(other) => {
                return Err(AxError::invalid(format!("Invalid sort direction: {other}")))
            }
        };
        Ok((column, direction))
    }

    /// `ILIKE` pattern for the search term, if one was given.
    pub fn search_pattern(&self) -> Option<String> {
        self.search
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| format!("%{s}%"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_rejects_unknown_columns() {
        let q = PostListQuery {
            order_by: Some("id; DROP TABLE posts;".into()),
            ..Default::default()
        };
        assert!(matches!(q.ordering(), Err(AxError::InvalidInput(_))));

        let q = PostListQuery {
            sort: Some("sideways".into()),
            ..Default::default()
        };
        assert!(matches!(q.ordering(), Err(AxError::InvalidInput(_))));
    }

    #[test]
    fn ordering_defaults_to_newest_first() {
        assert_eq!(
            PostListQuery::default().ordering().unwrap(),
            ("created_at", "DESC")
        );
        let q = PostListQuery {
            order_by: Some("like_count".into()),
            sort: Some("ASC".into()),
            ..Default::default()
        };
        assert_eq!(q.ordering().unwrap(), ("like_count", "ASC"));
    }

    #[test]
    fn create_post_requires_content() {
        let mut post = CreatePost {
            title: Some("   ".into()),
            content: "  ".into(),
            attachments: vec![],
        };
        assert!(post.normalize().is_err());

        let mut post = CreatePost {
            title: Some("  Hello ".into()),
            content: " body ".into(),
            attachments: vec![],
        };
        post.normalize().unwrap();
        assert_eq!(post.title.as_deref(), Some("Hello"));
        assert_eq!(post.content, "body");
    }
}
