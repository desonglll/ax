use actix_web::web;

use crate::errors::AxError;
use crate::services::features::user::get_user_features;
use crate::services::ml::predict::predict;
use crate::state::AppState;

pub async fn recommend_posts(app_state: web::Data<AppState>, user_id: i32) -> Result<Vec<i32>, AxError> {
    // 假设这是调用深度学习模型的逻辑
    // 这里可以使用外部服务或模型库
    let model_input = get_user_features(&app_state.db, user_id).await?;

    // 调用深度学习模型进行预测
    let recommended_ids = predict(model_input).await?;

    Ok(recommended_ids)
}
