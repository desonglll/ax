use actix_web::web;

use crate::errors::AxError;
use crate::services::features::get_user_features;
use crate::services::ml_predict::predict;
use crate::state::AppState;

/// 推荐推文
///
/// 根据用户 ID 获取用户特征，然后调用 ML 模型预测推荐推文 ID 列表。
///
/// # 参数
///
/// - `app_state`: 应用状态，包含数据库连接池
/// - `user_id`: 用户 ID
///
/// # 返回值
///
/// 成功时返回推荐的推文 ID 列表，失败时返回 [`AxError`]。
pub async fn recommend_posts(
    app_state: web::Data<AppState>,
    user_id: i32,
) -> Result<Vec<i32>, AxError> {
    // 假设这是调用深度学习模型的逻辑
    // 这里可以使用外部服务或模型库
    let model_input = get_user_features(&app_state.db, user_id).await?;

    // 调用深度学习模型进行预测
    let recommended_ids = predict(model_input).await?;

    Ok(recommended_ids)
}
