use serde::Serialize;
use sqlx::PgPool;

use crate::errors::AxError;

/// User feature metrics.
///
/// This structure represents feature inputs containing user activity statistics
/// passed to the recommendation machine learning model.
#[derive(Debug, Serialize)]
pub struct UserFeatures {
    pub user_id: i32,
    pub name: String,
    pub liked_posts_count: i64,
    pub average_like_count: f64,
    pub average_comment_count: f64,
    pub recent_activity_score: f64,
    pub engagement_rate: f64,
}

/// Compile feature metrics for a user.
///
/// This function queries user profile information and behavioral statistics (such as like counts,
/// averages, activity score, and engagement rate) and constructs a [`UserFeatures`] payload.
///
/// # Parameters
///
/// - `pool`: Reference to the PostgreSQL connection pool.
/// - `user_id`: The identifier of the user to fetch features for.
///
/// # Returns
///
/// A [`UserFeatures`] structure on success, or an [`AxError`] on database failure.
pub async fn get_user_features(pool: &PgPool, user_id: i32) -> Result<UserFeatures, AxError> {
    // Query base profile information of the user.
    let user = sqlx::query!(
        r#"
        SELECT id, user_name
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AxError::DBError(e.to_string()))?;

    // Query aggregated behavioral metrics from the statistics tables.
    let stats = sqlx::query!(
        r#"
        SELECT
            COALESCE(SUM(liked_posts_count), 0) AS liked_posts_count,
            COALESCE(AVG(average_like_count), 0.0) AS average_like_count,
            COALESCE(AVG(average_comment_count), 0.0) AS average_comment_count,
            COALESCE(AVG(recent_activity_score), 0.0) AS recent_activity_score,
            COALESCE(AVG(engagement_rate), 0.0) AS engagement_rate
        FROM user_stats
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_e| AxError::DBError(String::from("Error when fetching user stats.")))?;

    // Construct and return the features instance.
    let user_features = UserFeatures {
        user_id: user.id,
        name: user.user_name,
        liked_posts_count: stats.liked_posts_count.unwrap_or(0),
        average_like_count: stats.average_like_count.unwrap_or(0.0),
        average_comment_count: stats.average_comment_count.unwrap_or(0.0),
        recent_activity_score: stats.recent_activity_score.unwrap_or(0.0),
        engagement_rate: stats.engagement_rate.unwrap_or(0.0),
    };

    Ok(user_features)
}
