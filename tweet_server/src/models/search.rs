use serde::Deserialize;

use crate::{errors::AxError, response::PageQuery};

pub const MAX_QUERY_LEN: usize = 100;

/// Query parameters for `GET /api/search/posts` and `GET /api/search/users`.
#[derive(Deserialize, Debug, Default, Clone)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// A validated search: the cleaned-up text plus its `ILIKE` pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchTerm {
    /// Trimmed, whitespace-collapsed query, fed to `websearch_to_tsquery`.
    pub text: String,
    /// `%text%` with `\`, `%` and `_` escaped, for substring matching.
    pub pattern: String,
}

impl SearchQuery {
    pub fn page(&self) -> PageQuery {
        PageQuery {
            limit: self.limit,
            offset: self.offset,
        }
    }

    /// Cleans up `q` and rejects empty or over-long queries.
    pub fn term(&self) -> Result<SearchTerm, AxError> {
        let text = normalize_query(self.q.as_deref().unwrap_or(""));
        if text.is_empty() {
            return Err(AxError::invalid("q is required"));
        }
        if text.chars().count() > MAX_QUERY_LEN {
            return Err(AxError::invalid(format!(
                "q must be at most {MAX_QUERY_LEN} characters"
            )));
        }
        let pattern = format!("%{}%", escape_like(&text));
        Ok(SearchTerm { text, pattern })
    }
}

/// Trims and collapses runs of whitespace into single spaces.
pub fn normalize_query(raw: &str) -> String {
    raw.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Escapes `LIKE` metacharacters so user input only ever matches literally
/// (PostgreSQL's default escape character is `\`).
pub fn escape_like(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if matches!(c, '\\' | '%' | '_') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query(q: &str) -> SearchQuery {
        SearchQuery {
            q: Some(q.into()),
            ..Default::default()
        }
    }

    #[test]
    fn term_collapses_whitespace() {
        let term = query("  rust \t\n actix  ").term().unwrap();
        assert_eq!(term.text, "rust actix");
        assert_eq!(term.pattern, "%rust actix%");
    }

    #[test]
    fn term_rejects_empty_and_long_queries() {
        assert!(matches!(
            SearchQuery::default().term(),
            Err(AxError::InvalidInput(_))
        ));
        assert!(matches!(query("   ").term(), Err(AxError::InvalidInput(_))));
        assert!(query(&"a".repeat(MAX_QUERY_LEN)).term().is_ok());
        assert!(matches!(
            query(&"a".repeat(MAX_QUERY_LEN + 1)).term(),
            Err(AxError::InvalidInput(_))
        ));
    }

    #[test]
    fn length_counts_characters_not_bytes() {
        // 100 CJK characters are 300 bytes but still within the limit.
        assert!(query(&"搜".repeat(MAX_QUERY_LEN)).term().is_ok());
    }

    #[test]
    fn like_metacharacters_are_escaped() {
        assert_eq!(escape_like("100%_done\\"), "100\\%\\_done\\\\");
        assert_eq!(query("50%").term().unwrap().pattern, "%50\\%%");
        assert_eq!(escape_like("中文"), "中文");
    }
}
