use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::errors::AxError;
use crate::services::features::UserFeatures;

/// Invoke the machine learning prediction model.
///
/// This function transmits USER_FEATURES to the prediction service configured
/// via `MODEL_SERVER_URL` (default `http://127.0.0.1:8001`) and returns a
/// vector of recommended post IDs.
///
/// # Parameters
///
/// - `user_features`: Compiled metrics describing user behavior features.
///
/// # Returns
///
/// A vector of recommended post IDs on success, or an [`AxError`] on failure.
pub async fn predict(user_features: UserFeatures) -> Result<Vec<uuid::Uuid>, AxError> {
    let base_url =
        std::env::var("MODEL_SERVER_URL").unwrap_or_else(|_| "http://127.0.0.1:8001".to_string());
    let client = Client::builder().no_proxy().build()?;
    let response = client
        .post(format!("{}/predict", base_url.trim_end_matches('/')))
        .json(&json!({
            "liked_posts_count": user_features.liked_posts_count,
            "average_comment_count": user_features.average_comment_count,
            "engagement_rate": user_features.engagement_rate
        }))
        .send()
        .await?
        .json::<ModelApiResponse>()
        .await?;

    // Extract the list of recommended post identifiers.
    let recommended_ids = response.data; // The expected schema is { "data": [recommended_ids] }

    Ok(recommended_ids)
}

#[derive(Deserialize)]
struct ModelApiResponse {
    #[allow(dead_code)]
    message: String,
    data: Vec<uuid::Uuid>,
}

#[cfg(test)]
mod tests {
    use crate::services::features::UserFeatures;

    use super::*;

    #[tokio::test]
    async fn test_predict_success() {
        // Construct mock user features for testing.
        let user_features = UserFeatures {
            user_id: 0,
            name: "".to_string(),
            liked_posts_count: 50,
            average_like_count: 0.75,
            average_comment_count: 0.5,
            recent_activity_score: 0.9,
            engagement_rate: 0.85,
        };

        // Execute the prediction method.
        match predict(user_features).await {
            Ok(recommended_ids) => {
                // Assert that the returned recommendations are not empty.
                assert!(
                    !recommended_ids.is_empty(),
                    "Recommended IDs list should not be empty"
                );
            }
            Err(e) => {
                eprintln!("Prediction failed with error: {:?}", e);
            }
        }
    }
}
