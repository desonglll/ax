use serde::Serialize;
use sqlx::PgPool;

use crate::errors::AxError;

#[derive(Debug, Serialize)]
pub struct UserFeatures {
    pub user_id: i32,
    pub name: String,
    pub liked_posts_count: i64,
    pub average_like_count: f64,           // 用户发布内容的平均点赞数
    pub average_comment_count: f64,        // 用户发布内容的平均评论数
    pub recent_activity_score: f64,        // 用户近期活跃度得分
    pub engagement_rate: f64,              // 用户参与率
}

pub async fn get_user_features(pool: &PgPool, user_id: i32) -> Result<UserFeatures, AxError> {
    // 查询用户的基础信息
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

    // 查询用户的其他特征统计
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
        .map_err(|e| AxError::DBError(String::from("Error when fetching user stats.")))?;

    // 创建并返回 UserFeatures
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