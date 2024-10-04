use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::errors::AxError;
use crate::services::features::user::UserFeatures;

pub async fn predict(user_features: UserFeatures) -> Result<Vec<i32>, AxError> {
    let client = Client::builder().no_proxy().build()?;
    let response = client.post("http://127.0.0.1:8001/predict")
        .json(&json!({
            "liked_posts_count": user_features.liked_posts_count,
            "average_comment_count": user_features.average_comment_count,
            "engagement_rate": user_features.engagement_rate
        }))
        .send()
        .await?
        .json::<ModelApiResponse>()
        .await?;

    // 解析推荐的推文 ID
    let recommended_ids = response.data; // 假设返回的数据格式为 { "data": [推荐ID列表] }

    Ok(recommended_ids)
}

#[derive(Deserialize)]
struct ModelApiResponse {
    #[allow(dead_code)]
    message: String,
    data: Vec<i32>,
}


#[cfg(test)]
mod tests {
    use crate::services::features::user::UserFeatures;

    use super::*;

    #[tokio::test]
    async fn test_predict_success() {
        // 创建一个假定的用户特征
        let user_features = UserFeatures {
            user_id: 0,
            name: "".to_string(),
            liked_posts_count: 50,
            average_like_count: 0.75,
            average_comment_count: 0.5,
            recent_activity_score: 0.9,
            engagement_rate: 0.85,
        };

        // 调用预测函数
        match predict(user_features).await {
            Ok(recommended_ids) => {
                // 测试返回的推荐ID是否有效（假设至少有一个推荐结果）
                assert!(!recommended_ids.is_empty(), "Recommended IDs list should not be empty");
            }
            Err(e) => {
                panic!("Prediction failed with error: {:?}", e);
            }
        }
    }
}
